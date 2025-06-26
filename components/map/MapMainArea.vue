<template>
  <div class="map-area">
    <svg viewBox="0 0 1000 600" class="main-map">
      <!-- 背景网格和渐变定义 -->
      <defs>
        <pattern id="grid" width="20" height="20" patternUnits="userSpaceOnUse">
          <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#1a365d" stroke-width="0.5" opacity="0.3"/>
        </pattern>
        <linearGradient id="oceanGradient" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" style="stop-color:#1e3a5f;stop-opacity:1" />
          <stop offset="50%" style="stop-color:#2d5a87;stop-opacity:0.8" />
          <stop offset="100%" style="stop-color:#1a365d;stop-opacity:0.9" />
        </linearGradient>
        <filter id="glow">
          <feGaussianBlur stdDeviation="3" result="coloredBlur"/>
          <feMerge> 
            <feMergeNode in="coloredBlur"/>
            <feMergeNode in="SourceGraphic"/>
          </feMerge>
        </filter>
        <pattern id="satellite" width="40" height="40" patternUnits="userSpaceOnUse">
          <rect width="40" height="40" fill="#1a2332"/>
          <circle cx="10" cy="10" r="1" fill="#2d3748" opacity="0.6"/>
          <circle cx="30" cy="15" r="0.5" fill="#4a5568" opacity="0.4"/>
          <circle cx="20" cy="30" r="1.5" fill="#2d3748" opacity="0.5"/>
        </pattern>
      </defs>
      
      <!-- 网格背景 -->
      <rect width="1000" height="600" fill="url(#grid)" />
      
      <!-- 卫星地图背景 -->
      <rect width="1000" height="600" fill="url(#satellite)"/>
      
      <!-- 海域背景渐变 -->
      <rect width="1000" height="600" fill="url(#oceanGradient)" opacity="0.6"/>
      
      <!-- 地理轮廓 -->
      <path
        d="M0,350 Q100,330 200,340 L350,335 Q500,325 650,330 L800,335 Q900,340 1000,345 L1000,600 L0,600 Z" 
        fill="#3d4852" stroke="#5a6572" stroke-width="1" opacity="0.8"/>
      
      <!-- 岛屿群 -->
      <g class="islands">
        <ellipse cx="180" cy="180" rx="40" ry="25" fill="#4a5568" stroke="#6b7280" stroke-width="1" opacity="0.9"/>
        <ellipse cx="380" cy="160" rx="30" ry="18" fill="#4a5568" stroke="#6b7280" stroke-width="1" opacity="0.9"/>
        <ellipse cx="620" cy="140" rx="35" ry="22" fill="#4a5568" stroke="#6b7280" stroke-width="1" opacity="0.9"/>
        <ellipse cx="780" cy="200" rx="25" ry="15" fill="#4a5568" stroke="#6b7280" stroke-width="1" opacity="0.9"/>
      </g>
      
      <!-- 控制区域 -->
      <g class="control-zones">
        <circle cx="250" cy="250" r="80" fill="none" stroke="#fbb6ce" stroke-width="2" stroke-dasharray="5,5" opacity="0.6" filter="url(#glow)"/>
        <circle cx="500" cy="200" r="100" fill="none" stroke="#9ae6b4" stroke-width="2" stroke-dasharray="5,5" opacity="0.6" filter="url(#glow)"/>
        <circle cx="750" cy="300" r="90" fill="none" stroke="#feb2b2" stroke-width="2" stroke-dasharray="5,5" opacity="0.6" filter="url(#glow)"/>
      </g>
      
      <!-- 目标点 -->
      <g class="targets">
        <g v-for="target in targets" :key="target.id" :transform="`translate(${target.x}, ${target.y})`">
          <circle r="8" :fill="target.color" :stroke="target.strokeColor" stroke-width="2" filter="url(#glow)"/>
          <circle r="15" fill="none" :stroke="target.color" stroke-width="1" opacity="0.5"/>
          <circle r="25" fill="none" :stroke="target.color" stroke-width="1" opacity="0.3">
            <animate attributeName="r" values="25;35;25" dur="3s" repeatCount="indefinite"/>
            <animate attributeName="opacity" values="0.3;0.1;0.3" dur="3s" repeatCount="indefinite"/>
          </circle>
          <text y="-30" text-anchor="middle" fill="#e2e8f0" font-size="12" font-family="Microsoft YaHei">{{ target.label }}</text>
        </g>
      </g>
      
      <!-- 航迹线 -->
      <g class="tracks">
        <path
          d="M100,280 Q200,260 300,270 T500,250 T700,270 T900,250" 
          fill="none" stroke="#60a5fa" stroke-width="2" opacity="0.8"/>
        <path
          d="M150,320 Q250,300 350,310 T550,290 T750,310" 
          fill="none" stroke="#34d399" stroke-width="2" opacity="0.8"/>
      </g>
      
      <!-- 移动目标 -->
      <g class="vessels">
        <g class="vessel-1" transform="translate(320,285)">
          <circle r="15" fill="none" stroke="#00d9ff" stroke-width="2" opacity="0.8">
            <animate attributeName="r" values="15;25;15" dur="2s" repeatCount="indefinite"/>
            <animate attributeName="opacity" values="0.8;0.3;0.8" dur="2s" repeatCount="indefinite"/>
          </circle>
          <circle r="8" fill="#00d9ff" stroke="#0ea5e9" stroke-width="2"/>
          <text y="-25" text-anchor="middle" fill="#e2e8f0" font-size="10" font-weight="bold">1 749</text>
          <text y="-12" text-anchor="middle" fill="#a0aec0" font-size="8">CKB_1435.763</text>
        </g>
        <g class="vessel-2" transform="translate(580,310)">
          <circle r="15" fill="none" stroke="#4ade80" stroke-width="2" opacity="0.8">
            <animate attributeName="r" values="15;25;15" dur="2.5s" repeatCount="indefinite"/>
            <animate attributeName="opacity" values="0.8;0.3;0.8" dur="2.5s" repeatCount="indefinite"/>
          </circle>
          <circle r="8" fill="#4ade80" stroke="#22c55e" stroke-width="2"/>
          <text y="-25" text-anchor="middle" fill="#e2e8f0" font-size="10" font-weight="bold">743</text>
          <text y="-12" text-anchor="middle" fill="#a0aec0" font-size="8">CKB_1435.764</text>
        </g>
        <g class="vessel-3" transform="translate(450,200)">
          <circle r="15" fill="none" stroke="#f59e0b" stroke-width="2" opacity="0.8">
            <animate attributeName="r" values="15;25;15" dur="3s" repeatCount="indefinite"/>
            <animate attributeName="opacity" values="0.8;0.3;0.8" dur="3s" repeatCount="indefinite"/>
          </circle>
          <circle r="8" fill="#f59e0b" stroke="#d97706" stroke-width="2"/>
          <text y="-25" text-anchor="middle" fill="#e2e8f0" font-size="10" font-weight="bold">2 291</text>
          <text y="-12" text-anchor="middle" fill="#a0aec0" font-size="8">CKB_1435.765</text>
        </g>
      </g>
    </svg>
  </div>
</template>

<script setup>
import { useTargets } from '~/composables/useTargets'

const { targets } = useTargets()
</script>

<style scoped>
.map-area {
  flex: 1;
  position: relative;
  background: #0a1828;
  width: 100%;
  height: 100%;
}

.main-map {
  width: 100%;
  height: 100%;
}
</style> 