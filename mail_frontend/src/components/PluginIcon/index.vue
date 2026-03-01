<template>
  <div 
    class="flex items-center justify-center rounded-lg"
    :class="[sizeClass, bgClass]"
  >
    <!-- 网页爬虫 - Globe -->
    <svg v-if="pluginId === 'web_crawler'" :class="iconSizeClass" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="12" cy="12" r="10" />
      <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" />
      <path d="M2 12h20" />
    </svg>

    <!-- 邮件内容提取 - Microscope/Search -->
    <svg v-else-if="pluginId === 'email_content_extractor'" :class="iconSizeClass" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="11" cy="11" r="8" />
      <path d="m21 21-4.35-4.35" />
      <path d="M11 8a3 3 0 0 0-6 0" />
    </svg>

    <!-- 邮件转发器 - Arrow Right Circle -->
    <svg v-else-if="pluginId === 'email_forwarder'" :class="iconSizeClass" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="12" cy="12" r="10" />
      <path d="M8 12h8" />
      <path d="m13 9 3 3-3 3" />
    </svg>

    <!-- 邮件管理中心 - Chart Bar -->
    <svg v-else-if="pluginId === 'email_manager'" :class="iconSizeClass" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M3 3v18h18" />
      <path d="M18 17V9" />
      <path d="M13 17V5" />
      <path d="M8 17v-3" />
    </svg>

    <!-- 验证码提取器 - Key/Shield Check -->
    <svg v-else-if="pluginId === 'verification_code_extractor'" :class="iconSizeClass" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
      <path d="M7 11V7a5 5 0 0 1 10 0v4" />
      <circle cx="12" cy="16" r="1" />
    </svg>

    <!-- 默认插件图标 - Puzzle Piece -->
    <svg v-else :class="iconSizeClass" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M19.439 7.85c-.049.322.059.648.289.878l1.568 1.568c.47.47.706 1.087.706 1.704s-.235 1.233-.706 1.704l-1.611 1.611a.98.98 0 0 1-.837.276c-.47-.07-.802-.48-.968-.925a2.501 2.501 0 1 0-3.214 3.214c.446.166.855.497.925.968a.979.979 0 0 1-.276.837l-1.61 1.61a2.404 2.404 0 0 1-1.705.707 2.402 2.402 0 0 1-1.704-.706l-1.568-1.568a1.026 1.026 0 0 0-.877-.29c-.493.074-.84.504-1.02.968a2.5 2.5 0 1 1-3.237-3.237c.464-.18.894-.527.967-1.02a1.026 1.026 0 0 0-.289-.877l-1.568-1.568A2.402 2.402 0 0 1 1.998 12c0-.617.236-1.234.706-1.704L4.23 8.77c.24-.24.581-.353.917-.303.515.077.877.528 1.073 1.01a2.5 2.5 0 1 0 3.259-3.259c-.482-.196-.933-.558-1.01-1.073-.05-.336.062-.676.303-.917l1.525-1.525A2.402 2.402 0 0 1 12 1.998c.617 0 1.234.236 1.704.706l1.568 1.568c.23.23.556.338.877.29.493-.074.84-.504 1.02-.968a2.5 2.5 0 1 1 3.237 3.237c-.464.18-.894.527-.967 1.02Z" />
    </svg>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'

interface Props {
  pluginId: string
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  variant?: 'gradient' | 'solid' | 'light'
}

const props = withDefaults(defineProps<Props>(), {
  size: 'md',
  variant: 'gradient'
})

// 调试：打印接收到的 pluginId
onMounted(() => {
  console.log('PluginIcon received pluginId:', props.pluginId, typeof props.pluginId)
})

// 尺寸映射
const sizeClass = computed(() => {
  const sizeMap = {
    xs: 'w-8 h-8',
    sm: 'w-10 h-10',
    md: 'w-12 h-12',
    lg: 'w-16 h-16',
    xl: 'w-20 h-20'
  }
  return sizeMap[props.size]
})

// 图标大小
const iconSizeClass = computed(() => {
  const iconSizeMap = {
    xs: 'w-4 h-4',
    sm: 'w-5 h-5',
    md: 'w-6 h-6',
    lg: 'w-8 h-8',
    xl: 'w-10 h-10'
  }
  return `${iconSizeMap[props.size]} text-white`
})

// 背景颜色
const bgClass = computed(() => {
  const bgColors: Record<string, { gradient: string, solid: string, light: string }> = {
    web_crawler: {
      gradient: 'bg-gradient-to-br from-primary-600 to-cyan-600',
      solid: 'bg-primary-500',
      light: 'bg-primary-50 text-primary-600'
    },
    email_content_extractor: {
      gradient: 'bg-gradient-to-br from-purple-500 to-pink-600',
      solid: 'bg-purple-500',
      light: 'bg-purple-50 text-purple-600'
    },
    email_forwarder: {
      gradient: 'bg-gradient-to-br from-primary-600 to-primary-700',
      solid: 'bg-primary-500',
      light: 'bg-primary-50 text-primary-600'
    },
    email_manager: {
      gradient: 'bg-gradient-to-br from-indigo-500 to-blue-600',
      solid: 'bg-indigo-500',
      light: 'bg-indigo-50 text-indigo-600'
    },
    verification_code_extractor: {
      gradient: 'bg-gradient-to-br from-red-500 to-orange-600',
      solid: 'bg-red-500',
      light: 'bg-red-50 text-red-600'
    }
  }

  const defaultColors = {
    gradient: 'bg-gradient-to-br from-gray-500 to-gray-600',
    solid: 'bg-gray-500',
    light: 'bg-gray-50 text-black'
  }

  const colors = bgColors[props.pluginId] || defaultColors
  return colors[props.variant]
})
</script>
