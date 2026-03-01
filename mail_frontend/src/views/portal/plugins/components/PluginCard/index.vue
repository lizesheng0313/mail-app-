<template>
  <div 
    class="bg-white rounded-xl shadow-sm border border-gray-100 hover:shadow-xl hover:border-gray-200 transition-all duration-300 cursor-pointer overflow-hidden"
    :class="{ 
      'opacity-60': !plugin.is_enabled || plugin.is_expired,
      'ring-2 ring-yellow-400': plugin.is_featured,
      'border-accent-300': plugin.is_verified
    }"
    @click="$emit('click', plugin)"
  >
    <!-- 特色标签 -->
    <div v-if="plugin.is_featured" class="absolute top-2 right-2 z-10">
      <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
        <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
          <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
        </svg>
        推荐
      </span>
    </div>

    <!-- 验证标签 -->
    <div v-if="plugin.is_verified" class="absolute top-2 left-2 z-10">
      <div class="w-6 h-6 bg-primary-500 rounded-full flex items-center justify-center">
        <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
        </svg>
      </div>
    </div>

    <div class="p-6">
      <!-- 插件图标和基本信息 -->
      <div class="flex items-start space-x-4">
        <PluginIcon :plugin-id="plugin.plugin_id" size="md" />
        
        <div class="flex-1 min-w-0">
          <h3 style="font-size: 14px;" class="font-semibold text-black truncate">{{ plugin.name }}</h3>
          <p style="font-size: 12px;" class="text-black mt-1 line-clamp-2">{{ plugin.description }}</p>
        </div>
      </div>

      <!-- 插件元数据 -->
      <div class="mt-4 flex items-center justify-between">
        <div class="flex items-center space-x-2 text-black" style="font-size: 12px;">
          <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium"
                :class="getPluginTypeClass(plugin.plugin_type)">
            {{ getPluginTypeLabel(plugin.plugin_type) }}
          </span>
          <span class="text-gray-400">·</span>
          <span>v{{ plugin.version }}</span>
          <span v-if="plugin.author" class="text-gray-400">·</span>
          <span v-if="plugin.author">{{ plugin.author }}</span>
        </div>
        
        <!-- 运行状态指示器（仅未过期且已启用时显示绿点） -->
        <div v-if="showStatus && plugin.is_enabled && !plugin.is_expired" class="flex items-center space-x-1.5">
          <span class="w-2 h-2 rounded-full bg-primary-500 animate-pulse"></span>
          <span class="text-xs text-primary-600">运行中</span>
        </div>
      </div>

      <!-- 评分和统计 -->
      <div v-if="plugin.rating !== undefined || plugin.install_count !== undefined" class="mt-3 flex items-center justify-between text-black" style="font-size: 12px;">
        <div v-if="plugin.rating !== undefined" class="flex items-center space-x-1">
          <div class="flex items-center">
            <svg v-for="i in 5" :key="i" 
                 :class="i <= Math.floor(plugin.rating) ? 'text-yellow-400' : 'text-gray-300'"
                 class="w-4 h-4 fill-current" viewBox="0 0 20 20">
              <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
            </svg>
          </div>
          <span>{{ plugin.rating.toFixed(1) }}</span>
          <span v-if="plugin.rating_count">({{ plugin.rating_count }})</span>
        </div>
        
        <div v-if="plugin.install_count !== undefined" class="text-right">
          {{ formatNumber(plugin.install_count) }} 安装
        </div>
        
        <div v-if="plugin.usage_count !== undefined" class="text-right">
          使用 {{ plugin.usage_count }} 次
        </div>
      </div>



      <!-- 授权信息显示区域 -->
      <div v-if="showExpiry" class="mt-4">
        <div class="flex items-center justify-between">
          <!-- 免费插件 -->
          <div v-if="plugin.is_free" class="flex items-center space-x-2">
            <svg class="w-4 h-4 text-primary-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span class="text-xs text-gray-500">免费使用</span>
            <span class="text-xs px-2 py-0.5 rounded-full bg-primary-100 text-primary-700 font-medium">
              永久有效
            </span>
          </div>
          
          <!-- 付费插件 -->
          <div v-else class="flex-1">
            <div class="flex items-center space-x-2">
              <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <span class="text-xs text-gray-500">
                {{ plugin.expires_at && getRemainingDays(plugin.expires_at) > 0 ? `${formatExpiryDate(plugin.expires_at)} 到期` : '已过期' }}
              </span>
              <span 
                v-if="plugin.expires_at && getRemainingDays(plugin.expires_at) > 0"
                class="text-xs px-2 py-0.5 rounded-full"
                :class="isExpiringSoon(plugin.expires_at) ? 'bg-red-100 text-red-700 font-medium' : 'bg-gray-100 text-gray-600'"
              >
                剩余 {{ getRemainingDays(plugin.expires_at) }} 天
              </span>
              <span v-else class="text-xs px-2 py-0.5 rounded-full bg-red-100 text-red-700 font-medium">
                请续费
              </span>
            </div>
          </div>
          
          <!-- 续费按钮（仅付费插件显示） -->
          <button
            v-if="!plugin.is_free && showRenewButton"
            @click.stop="$emit('renew', plugin)"
            class="text-xs px-3 py-1.5 bg-primary-600 hover:bg-primary-700 text-white rounded-md transition-colors font-medium flex items-center space-x-1"
          >
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            <span>续费</span>
          </button>
        </div>
      </div>

      <!-- 价格信息 -->
      <div v-if="showPrice" class="mt-3 text-right">
        <span v-if="plugin.is_free" style="font-size: 14px;" class="font-semibold text-primary-600">免费</span>
        <span v-else style="font-size: 14px;" class="font-semibold text-orange-600">{{ Math.floor(plugin.price) }}奶片</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import PluginIcon from '@/components/PluginIcon/index.vue'

defineProps({
  plugin: {
    type: Object,
    required: true
  },
  showStatus: {
    type: Boolean,
    default: false
  },
  showExpiry: {
    type: Boolean,
    default: false
  },
  showPrice: {
    type: Boolean,
    default: false
  },
  showActions: {
    type: Boolean,
    default: true
  },
  showRenewButton: {
    type: Boolean,
    default: false
  }
})

defineEmits(['click', 'renew'])

// 获取插件类型样式
const getPluginTypeClass = (type) => {
  const classes = {
    crawler: 'bg-gray-100 text-gray-700',
    proxy: 'bg-gray-100 text-gray-700',
    email: 'bg-gray-100 text-gray-700',
    security: 'bg-gray-100 text-gray-700',
    utility: 'bg-gray-100 text-gray-700'
  }
  return classes[type] || 'bg-gray-100 text-black'
}

// 获取插件类型标签
const getPluginTypeLabel = (type) => {
  const labels = {
    crawler: '网页助手',
    proxy: '代理',
    email: '邮件',
    security: '安全',
    utility: '工具'
  }
  return labels[type] || type
}

// 格式化数字
const formatNumber = (num) => {
  if (!num && num !== 0) return '0'
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M'
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K'
  }
  return num.toString()
}

// 格式化到期日期
const formatExpiryDate = (timestamp) => {
  const date = new Date(timestamp)
  return date.toLocaleDateString('zh-CN', { year: 'numeric', month: '2-digit', day: '2-digit' })
}

// 计算剩余天数
const getRemainingDays = (timestamp) => {
  const now = new Date()
  const expiry = new Date(timestamp)
  const diff = expiry - now
  return Math.ceil(diff / (1000 * 60 * 60 * 24))
}

// 判断是否即将过期（少于7天）
const isExpiringSoon = (timestamp) => {
  const days = getRemainingDays(timestamp)
  return days > 0 && days <= 7
}
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
