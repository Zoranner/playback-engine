//! 地图瓦片代理服务
//!
//! 提供HTTP服务器，代理瓦片请求并支持本地缓存

use crate::geo::config;
use crate::types::{TileCoord, TileProxyConfig, TileProxyStats};
use log::{debug, error, info, warn};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::Path as StdPath;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};
use warp::{Filter, Rejection, Reply};

/// 瓦片代理服务
pub struct TileService {
    /// 服务配置
    config: TileProxyConfig,
    /// 统计信息
    stats: Arc<Mutex<TileProxyStats>>,
    /// HTTP客户端
    client: reqwest::Client,
    /// 服务器地址
    server_addr: SocketAddr,
}

impl TileService {
    /// 创建新的瓦片代理服务
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // 直接内部获取配置
        let config = config::get_tile_config();

        // 创建HTTP客户端
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.request_timeout))
            .build()?;

        // 确保缓存目录存在
        std::fs::create_dir_all(&config.cache_dir)?;

        let stats = Arc::new(Mutex::new(TileProxyStats {
            cache_hits: 0,
            upstream_requests: 0,
            total_requests: 0,
            cache_size: 0,
            last_updated: chrono::Utc::now(),
        }));

        let server_addr = SocketAddr::from(([127, 0, 0, 1], config.port));

        info!("瓦片代理服务初始化完成，缓存目录: {}, 服务地址: {}", config.cache_dir, server_addr);
        Ok(Self {
            config,
            stats,
            client,
            server_addr,
        })
    }

    /// 启动HTTP服务器
    pub async fn start_server(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let stats = Arc::clone(&self.stats);
        let config = self.config.clone();
        let client = self.client.clone();

        // 创建路由
        let tile_route = warp::path!("tile" / u8 / u32 / u32)
            .and(with_deps(stats.clone(), config.clone(), client.clone()))
            .and_then(handle_tile_request);

        let stats_route = warp::path("stats")
            .and(with_stats(stats.clone()))
            .and_then(handle_stats_request);

        let health_route = warp::path("health")
            .map(|| "OK");

        let routes = tile_route
            .or(stats_route)
            .or(health_route)
            .with(warp::cors().allow_any_origin());

        info!("启动瓦片代理服务器: http://{}", self.server_addr);

        // 启动服务器
        warp::serve(routes)
            .run(self.server_addr)
            .await;

        Ok(())
    }

    /// 获取服务URL
    pub fn get_service_url(&self) -> String {
        format!("http://{}", self.server_addr)
    }

    /// 获取瓦片URL模板
    pub fn get_tile_url_template(&self) -> String {
        format!("{}/tile/{{z}}/{{x}}/{{y}}", self.get_service_url())
    }
}

// 依赖注入过滤器
fn with_deps(
    stats: Arc<Mutex<TileProxyStats>>,
    config: TileProxyConfig,
    client: reqwest::Client,
) -> impl Filter<Extract = ((Arc<Mutex<TileProxyStats>>, TileProxyConfig, reqwest::Client),), Error = Infallible> + Clone {
    warp::any().map(move || (Arc::clone(&stats), config.clone(), client.clone()))
}

fn with_stats(
    stats: Arc<Mutex<TileProxyStats>>,
) -> impl Filter<Extract = (Arc<Mutex<TileProxyStats>>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&stats))
}

/// 处理瓦片请求
async fn handle_tile_request(
    z: u8,
    x: u32,
    y: u32,
    (stats, config, client): (Arc<Mutex<TileProxyStats>>, TileProxyConfig, reqwest::Client),
) -> Result<impl Reply, Rejection> {
    let coord = TileCoord::new(x, y, z);
    let start_time = Instant::now();

    // 更新统计信息
    {
        let mut stats_guard = stats.lock().await;
        stats_guard.total_requests += 1;
        stats_guard.last_updated = chrono::Utc::now();
    }

    // 首先尝试从本地缓存获取
    match get_from_cache(&coord, &config).await {
        Ok(cached_data) => {
            debug!("瓦片缓存命中: {:?}", coord);
            {
                let mut stats_guard = stats.lock().await;
                stats_guard.cache_hits += 1;
            }
            return Ok(create_tile_response(cached_data));
        }
        Err(_) => {
            // 缓存未命中，继续处理
        }
    }

    // 缓存未命中，从上游服务获取
    debug!("瓦片缓存未命中，从上游获取: {:?}", coord);
    match fetch_from_upstream(&coord, &config, &client).await {
        Ok(tile_data) => {
            // 保存到本地缓存
            if let Err(e) = save_to_cache(&coord, &tile_data, &config).await {
                warn!("保存瓦片到缓存失败: {:?}, 错误: {}", coord, e);
            }

            // 更新统计信息
            {
                let mut stats_guard = stats.lock().await;
                stats_guard.upstream_requests += 1;
            }

            let duration = start_time.elapsed();
            debug!("瓦片获取完成: {:?}, 耗时: {:?}", coord, duration);

            Ok(create_tile_response(tile_data))
        }
        Err(e) => {
            error!("获取瓦片失败: {:?}, 错误: {}", coord, e);
            Err(warp::reject::not_found())
        }
    }
}

/// 处理统计信息请求
async fn handle_stats_request(
    stats: Arc<Mutex<TileProxyStats>>,
) -> Result<impl Reply, Rejection> {
    let stats_data = stats.lock().await.clone();
    let json = serde_json::to_string(&stats_data).map_err(|_| warp::reject::not_found())?;
    Ok(warp::reply::with_header(json, "Content-Type", "application/json"))
}

/// 从本地缓存获取瓦片
async fn get_from_cache(coord: &TileCoord, config: &TileProxyConfig) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let cache_path = coord.path(&config.cache_dir);

    // 检查文件是否存在
    if !cache_path.exists() {
        return Err("缓存文件不存在".into());
    }

    // 检查文件是否过期
    if is_cache_expired(&cache_path, config.cache_ttl).await {
        debug!("缓存文件已过期: {:?}", cache_path);
        return Err("缓存文件已过期".into());
    }

    // 读取文件
    let data = tokio::fs::read(&cache_path).await?;
    Ok(data)
}

/// 从上游服务获取瓦片
async fn fetch_from_upstream(
    coord: &TileCoord,
    config: &TileProxyConfig,
    client: &reqwest::Client,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let url = build_upstream_url(coord, config);
    debug!("从上游获取瓦片: {}", url);

    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(format!("上游服务返回错误状态: {}", response.status()).into());
    }

    let data = response.bytes().await?;
    Ok(data.to_vec())
}

/// 保存瓦片到本地缓存
async fn save_to_cache(coord: &TileCoord, data: &[u8], config: &TileProxyConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cache_path = coord.path(&config.cache_dir);

    // 确保目录存在
    if let Some(parent) = cache_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    // 写入文件
    tokio::fs::write(&cache_path, data).await?;

    debug!("瓦片已保存到缓存: {:?}", cache_path);
    Ok(())
}

/// 检查缓存是否过期
async fn is_cache_expired(cache_path: &StdPath, cache_ttl: u64) -> bool {
    match tokio::fs::metadata(cache_path).await {
        Ok(metadata) => {
            if let Ok(modified) = metadata.modified() {
                let modified_time: chrono::DateTime<chrono::Utc> = chrono::DateTime::from(modified);
                let now = chrono::Utc::now();
                let age = now.signed_duration_since(modified_time);
                age.num_seconds() > cache_ttl as i64
            } else {
                true // 无法获取修改时间，认为已过期
            }
        }
        Err(_) => true, // 无法获取元数据，认为已过期
    }
}

/// 构建上游服务URL
fn build_upstream_url(coord: &TileCoord, config: &TileProxyConfig) -> String {
    config.upstream_url
        .replace("{z}", &coord.z.to_string())
        .replace("{y}", &coord.y.to_string())
        .replace("{x}", &coord.x.to_string())
}

/// 创建瓦片响应
fn create_tile_response(data: Vec<u8>) -> impl Reply {
    warp::reply::with_header(data, "Content-Type", "image/png")
}

impl Default for TileService {
    fn default() -> Self {
        Self::new().expect("创建默认瓦片服务失败")
    }
}
