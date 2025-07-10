/**
 * 事件级别类型
 */
export type EventLevel = 'high' | 'warning' | 'normal' | 'low';

/**
 * 事件数据接口
 */
export interface Event {
  id: number;
  time: string;
  title: string;
  description: string;
  level: EventLevel;
}

/**
 * 目标状态类型
 */
export type TargetStatus = 'friendly' | 'neutral' | 'unknown';

/**
 * 目标数据接口
 */
export interface Target {
  id: string;
  label: string;
  x: number;
  y: number;
  color: string;
  strokeColor: string;
  distance: string;
  bearing: string;
  status: TargetStatus;
}

/**
 * 环境信息数据接口
 */
export interface EnvironmentData {
  label: string;
  value: string;
}

/**
 * 坐标信息接口
 */
export interface Coordinates {
  longitude: number;
  latitude: number;
}

/**
 * 屏幕尺寸接口
 */
export interface ScreenSize {
  width: number;
  height: number;
}

/**
 * 按钮变体类型
 */
export type ButtonVariant =
  | 'default'
  | 'primary'
  | 'success'
  | 'warning'
  | 'danger'
  | 'ghost'
  | 'text';

/**
 * 按钮尺寸类型
 */
export type ButtonSize = 'small' | 'medium' | 'large';

/**
 * 卡片变体类型
 */
export type CardVariant = 'default' | 'ghost' | 'elevated';

/**
 * 卡片尺寸类型
 */
export type CardSize = 'small' | 'medium' | 'large';

/**
 * 图例项目接口
 */
export interface LegendItem {
  id: string;
  label: string;
  color: string;
  visible: boolean;
}

/**
 * 可见图层接口
 */
export interface VisibleLayers {
  targets: boolean;
  platforms: boolean;
  tracks: boolean;
  zones: boolean;
  grid: boolean;
  terrain: boolean;
}

/**
 * 测量结果接口
 */
export interface MeasurementResult {
  distance: number;
  bearing: number;
  startPoint: Coordinates;
  endPoint: Coordinates;
}

/**
 * 工具类型
 */
export type MapTool = 'select' | 'pan' | 'zoom' | 'measure' | 'annotate';

/**
 * 状态消息类型
 */
export type StatusType = 'info' | 'success' | 'warning' | 'error';
