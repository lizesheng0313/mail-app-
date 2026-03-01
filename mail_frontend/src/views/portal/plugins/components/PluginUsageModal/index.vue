<template>
  <!-- 模态框背景 -->
  <div class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50" @click="$emit('close')">
    <!-- 模态框内容 -->
    <div class="relative top-10 mx-auto p-6 border w-11/12 max-w-4xl shadow-lg rounded-lg bg-white" @click.stop>
      <!-- 头部 -->
      <div class="flex items-center justify-between pb-4 border-b border-gray-200">
        <div>
          <h3 class="text-lg font-semibold text-black">使用统计</h3>
          <p class="text-sm text-black mt-1">插件使用情况分析</p>
        </div>
        <button
          @click="$emit('close')"
          class="text-gray-400 hover:text-black transition-colors"
        >
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- 时间范围选择 -->
      <div class="py-4 border-b border-gray-200">
        <div class="flex items-center space-x-4">
          <label class="text-sm font-medium text-black">时间范围:</label>
          <select
            v-model="selectedDays"
            @change="fetchStats"
            class="px-3 py-1 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
          >
            <option value="7">最近7天</option>
            <option value="30">最近30天</option>
            <option value="90">最近90天</option>
          </select>
        </div>
      </div>

      <!-- 内容区域 -->
      <div class="py-6">
        <!-- 加载状态 -->
        <div v-if="loading" class="flex justify-center py-12">
          <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
        </div>

        <!-- 统计数据 -->
        <div v-else-if="stats.summary" class="space-y-6">
          <!-- 概览卡片 -->
          <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div class="bg-primary-50 rounded-lg p-4">
              <div class="flex items-center">
                <div class="flex-shrink-0">
                  <svg class="h-8 w-8 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                  </svg>
                </div>
                <div class="ml-4">
                  <p class="text-sm font-medium text-primary-600">总调用次数</p>
                  <p class="text-2xl font-semibold text-blue-900">{{ stats.summary.total_calls || 0 }}</p>
                </div>
              </div>
            </div>

            <div class="bg-primary-50 rounded-lg p-4">
              <div class="flex items-center">
                <div class="flex-shrink-0">
                  <svg class="h-8 w-8 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                </div>
                <div class="ml-4">
                  <p class="text-sm font-medium text-primary-600">成功调用</p>
                  <p class="text-2xl font-semibold text-success-800">{{ stats.summary.success_calls || 0 }}</p>
                </div>
              </div>
            </div>

            <div class="bg-red-50 rounded-lg p-4">
              <div class="flex items-center">
                <div class="flex-shrink-0">
                  <svg class="h-8 w-8 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                </div>
                <div class="ml-4">
                  <p class="text-sm font-medium text-red-600">失败调用</p>
                  <p class="text-2xl font-semibold text-red-900">{{ stats.summary.error_calls || 0 }}</p>
                </div>
              </div>
            </div>

            <div class="bg-yellow-50 rounded-lg p-4">
              <div class="flex items-center">
                <div class="flex-shrink-0">
                  <svg class="h-8 w-8 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                </div>
                <div class="ml-4">
                  <p class="text-sm font-medium text-yellow-600">成功率</p>
                  <p class="text-2xl font-semibold text-yellow-900">{{ successRate }}%</p>
                </div>
              </div>
            </div>
          </div>

          <!-- 使用趋势图表 -->
          <div class="bg-white border border-gray-200 rounded-lg p-6">
            <h4 class="text-lg font-medium text-black mb-4">使用趋势</h4>
            <SimpleChart
              title="每日使用统计"
              :data="stats.daily_stats || []"
            />
          </div>

          <!-- 最近调用记录 -->
          <div class="bg-white border border-gray-200 rounded-lg p-6">
            <h4 class="text-lg font-medium text-black mb-4">最近调用记录</h4>
            
            <div v-if="stats.recent_logs && stats.recent_logs.length > 0" class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
              <table class="min-w-full divide-y divide-gray-300">
                <thead class="bg-gray-50">
                  <tr>
                    <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">时间</th>
                    <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">操作</th>
                    <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">状态</th>
                    <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">耗时</th>
                    <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">结果</th>
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  <tr v-for="log in stats.recent_logs" :key="log.id" class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-black">
                      {{ formatDateTime(log.created_at) }}
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-black">
                      {{ log.action || 'execute' }}
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <span
                        :class="[
                          'inline-flex px-2 py-1 text-xs font-semibold rounded-full',
                          log.status === 'success'
                            ? 'bg-primary-100 text-success-800'
                            : 'bg-red-100 text-red-800'
                        ]"
                      >
                        {{ log.status === 'success' ? '成功' : '失败' }}
                      </span>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-black">
                      {{ log.execution_time ? `${log.execution_time}ms` : '-' }}
                    </td>
                    <td class="px-6 py-4 text-sm text-black max-w-xs truncate">
                      {{ log.result_summary || log.error_message || '-' }}
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
            
            <div v-else class="text-center py-8 text-black">
              暂无调用记录
            </div>
          </div>
        </div>

        <!-- 空状态 -->
        <div v-else class="text-center py-12">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
          </svg>
          <h3 class="mt-2 text-sm font-medium text-black">暂无统计数据</h3>
          <p class="mt-1 text-sm text-black">该插件还没有使用记录</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import SimpleChart from '../SimpleChart/index.vue'
import { pluginApi } from '@/api/plugin'
import { showMessage } from '@/utils/message'

const props = defineProps({
  pluginId: {
    type: String,
    required: true
  }
})

defineEmits(['close'])

const loading = ref(false)
const stats = ref({})
const selectedDays = ref(7)

// 计算成功率
const successRate = computed(() => {
  if (!stats.value.summary) return 0
  const total = stats.value.summary.total_calls || 0
  const success = stats.value.summary.success_calls || 0
  return total > 0 ? Math.round((success / total) * 100) : 0
})

// 获取统计数据
const fetchStats = async () => {
  try {
    loading.value = true
    const response = await pluginApi.getPluginStats(props.pluginId, selectedDays.value)
    
    if (response.code === 0) {
      stats.value = response.data
    } else {
      showMessage(response.message, 'error')
    }
  } catch (error) {
    showMessage('获取统计数据失败', 'error')
  } finally {
    loading.value = false
  }
}

// 格式化日期时间
const formatDateTime = (dateString) => {
  const date = new Date(dateString)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

onMounted(() => {
  fetchStats()
})
</script>
