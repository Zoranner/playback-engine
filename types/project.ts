/**
 * 工程元数据接口
 */
export interface ProjectMetadata {
  version: string
  description?: string
  tags: string[]
  participants: string[]
}

/**
 * 工程信息接口
 */
export interface ProjectInfo {
  name: string
  path: string
  totalDuration: number      // 总时长（纳秒）
  fileCount: number
  startTime: string          // ISO格式时间字符串
  endTime: string            // ISO格式时间字符串
  metadata: ProjectMetadata
  pcapFiles: string[]        // PCAP文件路径列表
}

/**
 * 播放状态接口
 */
export interface PlaybackState {
  isPlaying: boolean
  isPaused: boolean
  currentTime: number        // 当前播放时间（纳秒）
  totalDuration: number      // 总时长（纳秒）
  speed: number              // 播放倍速
}

/**
 * 数据包类型
 */
export type PacketType = 'Environment' | 'Event' | 'Target' | 'Unknown'

/**
 * 数据包接口
 */
export interface DataPacket {
  timestampSec: number
  timestampNsec: number
  data: number[]
  packetType: PacketType
  size: number
}

/**
 * 数据更新负载接口
 */
export interface DataUpdatePayload {
  environment: any
  events: any
  targets: any
  timestamp: number
}

/**
 * 应用信息接口
 */
export interface AppInfo {
  name: string
  version: string
  description: string
  rustVersion: string
} 