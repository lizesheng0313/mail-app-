<template>
  <div class="bg-white rounded-lg shadow-sm border p-6">
    <div class="flex items-center justify-between mb-4">
      <div class="flex items-center space-x-3">
        <div class="w-10 h-10 bg-primary-100 rounded-lg flex items-center justify-center">
          <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
        </div>
        <div>
          <h3 class="text-lg font-semibold text-black">网页助手连接池状态</h3>
          <p class="text-sm text-black">实时监控网页助手工作流执行情况</p>
        </div>
      </div>
      <button 
        @click="refreshStatus" 
        :disabled="loading"
        class="px-3 py-2 text-sm font-medium text-black bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50"
      >
        <svg class="w-4 h-4 inline-block" :class="{'animate-spin': loading}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
        刷新
      </button>
    </div>

    <div v-if="loading" class="text-center py-8">
      <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
      <p class="mt-2 text-black">加载中...</p>
    </div>

    <div v-else class="space-y-4">
      <!-- 状态卡片 -->
      <div class="grid grid-cols-4 gap-4">
        <div class="bg-primary-50 rounded-lg p-4">
          <div class="flex items-center justify-between">
            <div>
              <div class="text-sm text-primary-600 font-medium">运行中</div>
              <div class="text-3xl font-bold text-primary-700 mt-1">{{ status?.active_count || 0 }}</div>
            </div>
            <div class="text-primary-600">
              <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
          <div class="text-xs text-primary-600 mt-2">/ {{ status?.max_concurrent || 5 }} 最大并发</div>
        </div>

        <div class="bg-yellow-50 rounded-lg p-4">
          <div class="flex items-center justify-between">
            <div>
              <div class="text-sm text-yellow-600 font-medium">等待中</div>
              <div class="text-3xl font-bold text-yellow-700 mt-1">{{ status?.queue_count || 0 }}</div>
            </div>
            <div class="text-yellow-600">
              <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
          <div class="text-xs text-yellow-600 mt-2">队列任务</div>
        </div>

        <div class="bg-green-50 rounded-lg p-4">
          <div class="flex items-center justify-between">
            <div>
              <div class="text-sm text-green-600 font-medium">成功总数</div>
              <div class="text-3xl font-bold text-green-700 mt-1">{{ completedWorkflowsCount || 0 }}</div>
            </div>
            <div class="text-green-600">
              <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
          <div class="text-xs text-green-600 mt-2">累计成功工作流</div>
        </div>

        <div class="bg-red-50 rounded-lg p-4">
          <div class="flex items-center justify-between">
            <div>
              <div class="text-sm text-red-600 font-medium">失败总数</div>
              <div class="text-3xl font-bold text-red-700 mt-1">{{ failedWorkflowsCount || 0 }}</div>
            </div>
            <div class="text-red-600">
              <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
          <div class="text-xs text-red-600 mt-2">累计失败工作流</div>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { workflowApi } from '@/api/workflow'
import { showMessage } from '@/utils/message'

// 响应式数据
const loading = ref(false)
const status = ref<any>(null)
const failedWorkflowsCount = ref(0)
const completedWorkflowsCount = ref(0)

// 加载爬虫连接池状态
const loadStatus = async () => {
  try {
    loading.value = true
    const response = await workflowApi.getCrawlerPoolStatus()
    if (response.code === 0) {
      status.value = response.data
    }
  } catch (error: any) {
    console.error('获取爬虫池状态失败:', error)
    showMessage('获取爬虫池状态失败', 'error')
  } finally {
    loading.value = false
  }
}

// 加载失败工作流总数
const loadFailedWorkflowsCount = async () => {
  try {
    // 使用新的统计API，管理员可以查看所有用户的失败工作流
    const response = await workflowApi.getWorkflowCountByStatus('failed')
    if (response.code === 0 && response.data) {
      failedWorkflowsCount.value = response.data.count || 0
    }
  } catch (error: any) {
    console.error('获取失败工作流数量失败:', error)
  }
}

// 加载成功工作流总数
const loadCompletedWorkflowsCount = async () => {
  try {
    // 使用新的统计API，管理员可以查看所有用户的成功工作流
    const response = await workflowApi.getWorkflowCountByStatus('completed')
    if (response.code === 0 && response.data) {
      completedWorkflowsCount.value = response.data.count || 0
    }
  } catch (error: any) {
    console.error('获取成功工作流数量失败:', error)
  }
}

// 刷新状态
const refreshStatus = async () => {
  await Promise.all([
    loadStatus(),
    loadFailedWorkflowsCount(),
    loadCompletedWorkflowsCount()
  ])
  showMessage('爬虫池状态已刷新', 'success')
}

// 暴露刷新方法给父组件
defineExpose({
  refreshStatus
})

// 组件加载时获取数据
onMounted(() => {
  refreshStatus()
})
</script>