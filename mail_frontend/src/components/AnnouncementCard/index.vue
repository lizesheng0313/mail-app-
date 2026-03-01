<template>
  <div class="bg-white rounded-lg shadow-sm border p-4 h-full flex flex-col">
    <div class="flex items-center justify-between mb-3">
      <h3 class="text-base font-semibold text-gray-900">系统公告</h3>
    </div>

    <!-- 加载状态 -->
    <div v-if="loading" class="flex-1 flex items-center justify-center">
      <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
    </div>

    <!-- 公告列表 -->
    <div v-else-if="announcements && announcements.length > 0" class="flex-1 space-y-2 overflow-y-auto">
      <div
        v-for="announcement in announcements"
        :key="announcement.id"
        class="border-l-4 pl-3 py-2"
        :class="getTypeColor(announcement.type)"
      >
        <div class="flex items-start justify-between">
          <div class="flex-1">
            <div class="flex items-center gap-2">
              <span
                class="px-2 py-0.5 text-xs rounded"
                :class="getTypeBadgeClass(announcement.type)"
              >
                {{ getTypeText(announcement.type) }}
              </span>
              <h4 class="text-sm font-medium text-gray-900">{{ announcement.title }}</h4>
            </div>
            <p class="text-xs text-gray-500 mt-1">{{ formatDate(announcement.created_at) }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else class="flex-1 flex flex-col items-center justify-center text-gray-500">
      <svg class="h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
      </svg>
      <p class="mt-2 text-sm">暂无公告</p>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { getAnnouncements } from '@/api/announcement'

const announcements = ref([])
const loading = ref(false)

// 加载公告列表
const loadAnnouncements = async () => {
  loading.value = true
  try {
    console.log('正在加载公告列表...')
    const res = await getAnnouncements({
      page: 1,
      page_size: 3  // 只显示最新3条
    })

    console.log('公告接口返回:', res)

    if (res.code === 0 && res.data) {
      announcements.value = res.data.items || []
      console.log('公告数据:', announcements.value)
    } else {
      console.warn('公告接口返回异常:', res)
    }
  } catch (error) {
    console.error('加载公告失败:', error)
  } finally {
    loading.value = false
  }
}

// 获取类型颜色
const getTypeColor = (type) => {
  const colors = {
    'system': 'border-blue-500',
    'maintenance': 'border-orange-500',
    'feature': 'border-green-500',
    'notice': 'border-purple-500'
  }
  return colors[type] || 'border-gray-500'
}

// 获取类型徽章样式
const getTypeBadgeClass = (type) => {
  const classes = {
    'system': 'bg-blue-100 text-blue-700',
    'maintenance': 'bg-orange-100 text-orange-700',
    'feature': 'bg-green-100 text-green-700',
    'notice': 'bg-purple-100 text-purple-700'
  }
  return classes[type] || 'bg-gray-100 text-gray-700'
}

// 获取类型文本
const getTypeText = (type) => {
  const texts = {
    'system': '系统',
    'maintenance': '维护',
    'feature': '新功能',
    'notice': '通知'
  }
  return texts[type] || '公告'
}

// 格式化日期
const formatDate = (timestamp) => {
  const date = new Date(timestamp)
  const now = new Date()
  const diff = now - date
  const seconds = Math.floor(diff / 1000)
  const minutes = Math.floor(seconds / 60)
  const hours = Math.floor(minutes / 60)
  const days = Math.floor(hours / 24)

  if (seconds < 60) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  if (hours < 24) return `${hours}小时前`
  if (days < 7) return `${days}天前`

  return date.toLocaleDateString('zh-CN')
}

onMounted(() => {
  loadAnnouncements()
})
</script>
