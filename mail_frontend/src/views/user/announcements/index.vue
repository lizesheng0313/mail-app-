<template>
  <div>
    <!-- 公告列表 -->
    <div class="bg-white rounded-lg shadow">
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">标题</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">类型</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">内容</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">发布时间</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">操作</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="item in announcements" :key="item.id" class="hover:bg-gray-50">
              <td class="px-6 py-4">
                <div class="flex items-center">
                  <div class="text-sm font-medium text-gray-900">{{ item.title }}</div>
                  <span v-if="!item.is_read" class="ml-2 px-2 py-0.5 text-xs font-medium bg-primary-100 text-primary-700 rounded-full">
                    未读
                  </span>
                </div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <span
                  :class="{
                    'bg-primary-100 text-primary-700': item.type === 'info',
                    'bg-yellow-100 text-yellow-700': item.type === 'warning',
                    'bg-green-100 text-green-700': item.type === 'success',
                    'bg-red-100 text-red-700': item.type === 'error'
                  }"
                  class="px-2 py-1 text-xs font-medium rounded-full"
                >
                  {{ getTypeName(item.type) }}
                </span>
              </td>
              <td class="px-6 py-4 text-sm text-gray-900 max-w-md">
                <div class="line-clamp-2">{{ item.content }}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600">
                {{ formatDate(item.created_at) }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm">
                <ActionButton
                  icon="eye"
                  tooltip="查看详情"
                  variant="view"
                  @click="viewDetail(item)"
                />
              </td>
            </tr>
            <tr v-if="loading">
              <td colspan="5" class="px-6 py-12 text-center text-gray-500">
                <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
                <p class="mt-2">加载中...</p>
              </td>
            </tr>
            <tr v-else-if="announcements.length === 0">
              <td colspan="5" class="px-6 py-12 text-center text-gray-500">
                暂无公告
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- 分页 -->
      <div class="px-6 py-4 flex items-center justify-between border-t border-gray-200">
        <div class="text-sm text-gray-700">
          共 {{ total }} 条记录
        </div>
        <div class="flex gap-2">
          <button
            @click="changePage(page - 1)"
            :disabled="page === 1"
            class="px-3 py-1 text-sm border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            上一页
          </button>
          <span class="px-3 py-1 text-sm text-gray-700">
            第 {{ page }} / {{ totalPages }} 页
          </span>
          <button
            @click="changePage(page + 1)"
            :disabled="page >= totalPages"
            class="px-3 py-1 text-sm border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            下一页
          </button>
        </div>
      </div>
    </div>

    <!-- 详情弹窗 -->
    <div v-if="showDetailModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[80vh] overflow-y-auto">
        <div class="p-6">
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-lg font-bold">公告详情</h3>
            <button @click="showDetailModal = false" class="text-gray-400 hover:text-gray-600">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          <div v-if="selectedAnnouncement" class="space-y-4">
            <div>
              <div class="text-sm text-gray-600 mb-1">标题</div>
              <div class="text-base font-medium">{{ selectedAnnouncement.title }}</div>
            </div>
            <div>
              <div class="text-sm text-gray-600 mb-1">类型</div>
              <span
                :class="{
                  'bg-primary-100 text-primary-700': selectedAnnouncement.type === 'info',
                  'bg-yellow-100 text-yellow-700': selectedAnnouncement.type === 'warning',
                  'bg-green-100 text-green-700': selectedAnnouncement.type === 'success',
                  'bg-red-100 text-red-700': selectedAnnouncement.type === 'error'
                }"
                class="px-2 py-1 text-xs font-medium rounded-full"
              >
                {{ getTypeName(selectedAnnouncement.type) }}
              </span>
            </div>
            <div>
              <div class="text-sm text-gray-600 mb-1">发布时间</div>
              <div class="text-sm">{{ formatDate(selectedAnnouncement.created_at) }}</div>
            </div>
            <div>
              <div class="text-sm text-gray-600 mb-1">内容</div>
              <div class="text-sm text-gray-700 whitespace-pre-wrap">{{ selectedAnnouncement.content }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { showMessage } from '@/utils/message'
import ActionButton from '@/components/ActionButton/index.vue'

const route = useRoute()
const loading = ref(false)
const announcements = ref([])
const page = ref(1)
const pageSize = ref(20)
const total = ref(0)

const showDetailModal = ref(false)
const selectedAnnouncement = ref(null)

const totalPages = computed(() => Math.ceil(total.value / pageSize.value))

// 加载公告列表
const loadAnnouncements = async () => {
  loading.value = true
  try {
    const token = localStorage.getItem('token')
    const response = await fetch(`/mail-api/v1/announcements/?page=${page.value}&page_size=${pageSize.value}`, {
      headers: {
        'Authorization': `Bearer ${token}`
      }
    })

    const result = await response.json()
    if (result.code === 0) {
      announcements.value = result.data.items || []
      total.value = result.data.total || 0

      // 如果 URL 中有公告 ID，自动打开该公告
      const announcementId = route.query.id
      if (announcementId) {
        const announcement = announcements.value.find(a => a.id === parseInt(announcementId))
        if (announcement) {
          viewDetail(announcement)
        }
      }
    } else {
      showMessage(result.message || '加载失败', 'error')
    }
  } catch (error) {
    console.error('加载公告失败:', error)
    showMessage('加载失败', 'error')
  } finally {
    loading.value = false
  }
}

// 查看详情
const viewDetail = async (announcement) => {
  selectedAnnouncement.value = announcement
  showDetailModal.value = true

  // 标记为已读
  if (!announcement.is_read) {
    await markAsRead(announcement.id)
  }
}

// 标记为已读
const markAsRead = async (announcementId) => {
  try {
    const token = localStorage.getItem('token')
    const response = await fetch(`/mail-api/v1/announcements/${announcementId}/read`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`
      }
    })

    const result = await response.json()
    if (result.code === 0) {
      // 更新本地状态
      const announcement = announcements.value.find(a => a.id === announcementId)
      if (announcement) {
        announcement.is_read = true
      }
    }
  } catch (error) {
    console.error('标记已读失败:', error)
  }
}

// 格式化时间
const formatDate = (timestamp) => {
  if (!timestamp) return '-'
  const date = new Date(timestamp)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 获取类型名称
const getTypeName = (type) => {
  const typeMap = {
    'info': '信息',
    'warning': '警告',
    'success': '成功',
    'error': '错误'
  }
  return typeMap[type] || type
}

// 分页切换
const changePage = (newPage) => {
  if (newPage >= 1 && newPage <= totalPages.value) {
    page.value = newPage
    loadAnnouncements()
  }
}

onMounted(() => {
  loadAnnouncements()
})
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
