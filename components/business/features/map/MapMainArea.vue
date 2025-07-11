<template>
  <div ref="mapContainer" class="relative h-full w-full" />
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import maplibregl from 'maplibre-gl';
import 'maplibre-gl/dist/maplibre-gl.css';

// Props
const props = defineProps({
  activeTool: String,
  visibleLayers: Object,
});

// Emits
const emit = defineEmits([
  'coordinates-changed',
  'zoom-changed',
  'bearing-changed',
  'measurement-result',
  'equipment-selected',
]);

// 响应式数据
const mapContainer = ref();
let map = null;
const isGlobeView = ref(false);

// 初始化地图
onMounted(() => {
  map = new maplibregl.Map({
    container: mapContainer.value,
    zoom: 8, // 调整缩放级别以显示杭州区域
    center: [120.1551, 30.2741], // 杭州坐标
    style: {
      version: 8,
      projection: {
        type: 'globe',
      },
      sources: {
        satellite: {
          url: 'https://api.maptiler.com/tiles/satellite-v2/tiles.json?key=get_your_own_OpIi9ZULNHzrESv6T2vL',
          type: 'raster',
        },
      },
      layers: [
        {
          id: 'satellite',
          type: 'raster',
          source: 'satellite',
        },
      ],
    },
    // 交互控制
    dragRotate: false, // 禁用拖拽旋转（包括右键）
    dragPan: true, // 允许左键拖拽平移
    scrollZoom: true, // 允许滚轮缩放
    boxZoom: false, // 禁用框选缩放
    doubleClickZoom: false, // 禁用双击缩放
    keyboard: false, // 禁用键盘控制
    // 禁用右键相关功能
    cooperativeGestures: false, // 禁用协作手势
  });

  // 禁用右键菜单
  map.getCanvas().addEventListener('contextmenu', e => {
    e.preventDefault();
  });

  // 设置初始状态为地球视图
  isGlobeView.value = true;

  // 监听地图事件
  map.on('moveend', () => {
    const center = map.getCenter();
    const zoom = map.getZoom();
    const bearing = map.getBearing();

    emit('coordinates-changed', {
      longitude: center.lng,
      latitude: center.lat,
    });
    emit('zoom-changed', zoom);
    emit('bearing-changed', bearing);
  });

  // 监听点击事件
  map.on('click', event => {
    const features = map.queryRenderedFeatures(event.point);
    if (features.length > 0) {
      const feature = features[0];
      emit('equipment-selected', {
        id: feature.id || feature.properties?.id,
        name: feature.properties?.name || '未命名对象',
        type: feature.layer?.type || '未知类型',
        properties: feature.properties,
      });
    }
  });

  // 监听地图加载完成
  map.on('load', () => {
    console.log('卫星地球地图加载完成');
  });
});

// 切换到地球视图
const switchToGlobe = () => {
  if (!map) return;

  isGlobeView.value = true;

  // 切换到球面投影
  map.setProjection({
    type: 'globe',
  });

  // 设置3D视角 - 正对着地球表面
  map.setPitch(0);
  map.setBearing(0);

  // 调整到合适的缩放级别以显示杭州区域
  map.setZoom(8);
  map.setCenter([120.1551, 30.2741]); // 杭州坐标

  console.log('切换到地球视图');
};

// 切换到平面视图
const switchToFlat = () => {
  if (!map) return;

  isGlobeView.value = false;

  // 切换到墨卡托投影
  map.setProjection({
    type: 'mercator',
  });

  // 重置视角
  map.setPitch(0);
  map.setBearing(0);
  map.setZoom(8);
  map.setCenter([120.1551, 30.2741]); // 杭州坐标

  console.log('切换到平面视图');
};

// 切换视图模式
const toggleViewMode = () => {
  if (isGlobeView.value) {
    switchToFlat();
  } else {
    switchToGlobe();
  }
};

// 加载S57数据（GeoJSON格式）
const loadS57Data = async geojsonUrl => {
  if (!map) return;

  try {
    const response = await fetch(geojsonUrl);
    const geojsonData = await response.json();

    // 添加数据源
    map.addSource('s57-data', {
      type: 'geojson',
      data: geojsonData,
    });

    // 添加图层
    map.addLayer({
      id: 's57-points',
      type: 'circle',
      source: 's57-data',
      filter: ['==', ['geometry-type'], 'Point'],
      paint: {
        'circle-radius': 6,
        'circle-color': '#FF6B6B',
        'circle-stroke-color': '#FFF',
        'circle-stroke-width': 2,
      },
    });

    map.addLayer({
      id: 's57-lines',
      type: 'line',
      source: 's57-data',
      filter: ['==', ['geometry-type'], 'LineString'],
      paint: {
        'line-color': '#4682B4',
        'line-width': 2,
      },
    });

    map.addLayer({
      id: 's57-polygons',
      type: 'fill',
      source: 's57-data',
      filter: ['==', ['geometry-type'], 'Polygon'],
      paint: {
        'fill-color': 'rgba(173, 216, 230, 0.3)',
        'fill-outline-color': '#4682B4',
      },
    });

    // 缩放到数据范围
    const bounds = new maplibregl.LngLatBounds();
    geojsonData.features.forEach(feature => {
      if (feature.geometry.type === 'Point') {
        bounds.extend(feature.geometry.coordinates);
      } else if (feature.geometry.coordinates) {
        feature.geometry.coordinates.forEach(coord => {
          if (Array.isArray(coord[0])) {
            coord.forEach(point => bounds.extend(point));
          } else {
            bounds.extend(coord);
          }
        });
      }
    });

    if (!bounds.isEmpty()) {
      map.fitBounds(bounds, { padding: 20 });
    }

    console.log('S57数据加载成功');
  } catch (error) {
    console.error('加载S57数据失败:', error);
  }
};

// 切换图层可见性
const toggleLayer = (layerId, visible) => {
  if (map && map.getLayer(layerId)) {
    map.setLayoutProperty(layerId, 'visibility', visible ? 'visible' : 'none');
  }
};

// 设置3D视角
const set3DView = (pitch = 45, bearing = 0) => {
  if (!map) return;

  map.setPitch(pitch);
  map.setBearing(bearing);
};

// 旋转地球
const rotateGlobe = bearing => {
  if (!map) return;

  map.setBearing(bearing);
};

// 暴露方法
defineExpose({
  setSelectedEquipment: equipment => {
    console.log('选中设备:', equipment);
  },
  clearSelection: () => {
    console.log('清除选择');
  },
  loadS57Data,
  toggleLayer,
  switchToGlobe,
  switchToFlat,
  toggleViewMode,
  set3DView,
  rotateGlobe,
  isGlobeView: () => isGlobeView.value,
  map: () => map,
});

onUnmounted(() => {
  if (map) {
    map.remove();
  }
});
</script>

<style scoped>
/* MapLibre GL JS 会自动添加样式 */
</style>
