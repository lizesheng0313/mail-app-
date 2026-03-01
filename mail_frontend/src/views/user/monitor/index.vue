<template>
  <div class="h-full">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-full flex flex-col">
      <!-- 页面标题 -->
      <div class="mb-6">
        <h2 class="text-2xl font-semibold text-black">工作流监控</h2>
        <p class="text-sm text-gray-500 mt-1">查看工作流的调用情况和收益统计</p>
      </div>

      <!-- 筛选栏 -->
      <div class="bg-white rounded-lg shadow-sm border p-6 mb-6">
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
          <!-- 选择工作流 -->
          <CustomSelect
            v-model="selectedWorkflow"
            :options="workflowOptions"
            placeholder="全部工作流"
          />

          <!-- 时间范围 -->
          <CustomSelect
            v-model="timeRange"
            :options="timeRangeOptions"
            placeholder="选择时间范围"
          />

          <!-- 自定义日期 -->
          <template v-if="timeRange === 'custom'">
            <BaseInput
              v-model="startDate"
              type="date"
              placeholder="开始日期"
            />
            <BaseInput
              v-model="endDate"
              type="date"
              placeholder="结束日期"
            />
          </template>
        </div>
      </div>

      <!-- 统计卡片 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-6 mb-6">
        <!-- 总调用次数 -->
        <div class="bg-white rounded-lg shadow-sm border p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600">总调用次数</p>
              <p class="text-2xl font-bold text-black mt-1">{{ stats.totalCalls }}</p>
              <p class="text-xs text-gray-500 mt-1">成功率 {{ stats.successRate }}%</p>
            </div>
            <div class="h-12 w-12 bg-blue-100 rounded-lg flex items-center justify-center">
              <BaseIcon name="lightning-bolt" size="w-6 h-6" class="text-blue-600" />
            </div>
          </div>
        </div>

        <!-- 总收益 -->
        <div class="bg-white rounded-lg shadow-sm border p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600">总收益</p>
              <p class="text-2xl font-bold text-black mt-1">¥{{ stats.totalRevenue }}</p>
              <p class="text-xs text-green-600 mt-1">+{{ stats.revenueGrowth }}%</p>
            </div>
            <div class="h-12 w-12 bg-green-100 rounded-lg flex items-center justify-center">
              <BaseIcon name="currency-dollar" size="w-6 h-6" class="text-green-600" />
            </div>
          </div>
        </div>

        <!-- 活跃用户 -->
        <div class="bg-white rounded-lg shadow-sm border p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600">活跃用户</p>
              <p class="text-2xl font-bold text-black mt-1">{{ stats.activeUsers }}</p>
              <p class="text-xs text-gray-500 mt-1">新增 {{ stats.newUsers }}</p>
            </div>
            <div class="h-12 w-12 bg-purple-100 rounded-lg flex items-center justify-center">
              <BaseIcon name="users" size="w-6 h-6" class="text-purple-600" />
            </div>
          </div>
        </div>

        <!-- 平均响应时间 -->
        <div class="bg-white rounded-lg shadow-sm border p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600">平均响应时间</p>
              <p class="text-2xl font-bold text-black mt-1">{{ stats.avgResponseTime }}ms</p>
              <p class="text-xs text-gray-500 mt-1">P95: {{ stats.p95ResponseTime }}ms</p>
            </div>
            <div class="h-12 w-12 bg-orange-100 rounded-lg flex items-center justify-center">
              <BaseIcon name="clock" size="w-6 h-6" class="text-orange-600" />
            </div>
          </div>
        </div>
      </div>

      <!-- 图表区域 -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
        <!-- 调用趋势图 -->
        <div class="bg-white rounded-lg shadow-sm border p-6">
          <h3 class="text-lg font-semibold text-black mb-4">调用趋势</h3>
          <div class="h-64 flex items-center justify-center text-gray-400">
            <!-- 这里集成图表组件 -->
            <p>调用趋势图</p>
          </div>
        </div>

        <!-- 收益趋势图 -->
        <div class="bg-white rounded-lg shadow-sm border p-6">
          <h3 class="text-lg font-semibold text-black mb-4">收益趋势</h3>
          <div class="h-64 flex items-center justify-center text-gray-400">
            <!-- 这里集成图表组件 -->
            <p>收益趋势图</p>
          </div>
        </div>
      </div>

      <!-- 调用明细表格 -->
      <AdminDataTable
        title="调用明细"
        :pagination="pagination"
        :loading="loading"
        :show-page-size-selector="true"
        :column-count="7"
        @page-change="changePage"
        @page-size-change="changePageSize"
      >
        <template #thead>
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">时间</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">工作流</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">用户</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">耗时</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">收益</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">状态</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">操作</th>
          </tr>
        </template>

        <template #tbody>
          <tr v-for="record in callRecords" :key="record.id" class="hover:bg-gray-50">
            <td class="px-6 py-4 whitespace-nowrap text-sm text-black">
              {{ formatDateTime(record.created_at) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm font-medium text-black">{{ record.workflow_name }}</div>
              <div class="text-sm text-gray-500">{{ record.workflow_id }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-black">
              {{ record.user_name }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-black">
              {{ record.duration }}ms
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
              <span v-if="record.revenue > 0" class="text-green-600">
                ¥{{ record.revenue }}
              </span>
              <span v-else class="text-gray-400">-</span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span :class="getStatusClass(record.status)" class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full">
                {{ getStatusText(record.status) }}
              </span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium space-x-2">
              <ActionButton
                icon="eye"
                tooltip="查看详情"
                variant="view"
                @click="viewDetail(record)"
              />
            </td>
          </tr>
        </template>
      </AdminDataTable>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch, computed } from 'vue'
import { showMessage } from '@/utils/message'
import { showConfirm, showPrompt, showAlert } from '@/utils/dialog'
import { getMyWorkflows, getWorkflowMonitor } from '@/api/workflowMarket'
import BaseInput from '@/components/BaseInput/index.vue'
import CustomSelect from '@/components/CustomSelect/index.vue'
import AdminDataTable from '@/components/AdminDataTable/index.vue'
import ActionButton from '@/components/ActionButton/index.vue'
import BaseIcon from '@/components/BaseIcon/index.vue'

// 当前用户ID
const userId = ref(parseInt(localStorage.getItem('userId')) || 1)

// 选中的工作流
const selectedWorkflow = ref('')

// 时间范围
const timeRange = ref('week')
const startDate = ref('')
const endDate = ref('')

// 我的工作流列表
const myWorkflows = ref([])

// 工作流选项
const workflowOptions = computed(() => {
  const options = [{ label: '全部工作流', value: '' }]
  myWorkflows.value.forEach(workflow => {
    options.push({
      label: workflow.name,
      value: workflow.id
    })
  })
  return options
})

// 时间范围选项
const timeRangeOptions = [
  { label: '今天', value: 'today' },
  { label: '昨天', value: 'yesterday' },
  { label: '最近7天', value: 'week' },
  { label: '最近30天', value: 'month' },
  { label: '自定义', value: 'custom' }
]

// 统计数据
const stats = ref({
  totalCalls: 0,
  successRate: 0,
  totalRevenue: 0,
  revenueGrowth: 0,
  activeUsers: 0,
  newUsers: 0,
  avgResponseTime: 0,
  p95ResponseTime: 0
})

// 调用记录
const callRecords = ref([])

// 分页
const page = ref(1)
const pageSize = ref(20)
const total = ref(0)
const loading = ref(false)

// 分页数据
const pagination = computed(() => ({
  currentPage: page.value,
  page: page.value,
  totalPages: Math.ceil(total.value / pageSize.value),
  pages: Math.ceil(total.value / pageSize.value),
  total: total.value,
  pageSize: pageSize.value,
  limit: pageSize.value
}))

// 切换页码
const changePage = (newPage) => {
  page.value = newPage
  loadMonitorData()
}

// 切换每页数量
const changePageSize = (newPageSize) => {
  pageSize.value = newPageSize
  page.value = 1
  loadMonitorData()
}

// 加载我的工作流
const loadMyWorkflows = async () => {
  try {
    const res = await getMyWorkflows({
      user_id: userId.value,
      page: 1,
      page_size: 100
    })

    if (res.success) {
      myWorkflows.value = res.data.items || []
    }
  } catch (error) {
    console.error('加载工作流列表失败:', error)
    showMessage('加载工作流列表失败', 'error')
  }
}

// 加载统计数据和调用记录
const loadMonitorData = async () => {
  if (!selectedWorkflow.value) {
    return
  }

  loading.value = true
  try {
    const params = {
      user_id: userId.value,
      time_range: timeRange.value,
      page: page.value,
      page_size: pageSize.value
    }

    // 如果是自定义时间范围，添加开始和结束日期
    if (timeRange.value === 'custom' && startDate.value && endDate.value) {
      params.start_date = startDate.value
      params.end_date = endDate.value
    }

    const res = await getWorkflowMonitor(selectedWorkflow.value, params)

    if (res.success) {
      // 更新统计数据
      if (res.data.stats) {
        stats.value = {
          totalCalls: res.data.stats.total_calls || 0,
          successRate: res.data.stats.success_rate || 0,
          totalRevenue: res.data.stats.total_revenue || 0,
          revenueGrowth: res.data.stats.revenue_growth || 0,
          activeUsers: res.data.stats.active_users || 0,
          newUsers: res.data.stats.new_users || 0,
          avgResponseTime: res.data.stats.avg_response_time || 0,
          p95ResponseTime: res.data.stats.p95_response_time || 0
        }
      }

      // 更新调用记录
      callRecords.value = res.data.records || []
      total.value = res.data.total || 0
    }
  } catch (error) {
    console.error('加载监控数据失败:', error)
    showMessage('加载监控数据失败', 'error')
  } finally {
    loading.value = false
  }
}

// 查看详情
const viewDetail = (record) => {
  ElMessageBox.alert(
    `工作流：${record.workflow_name}<br>
    用户：${record.user_name}<br>
    耗时：${record.duration}ms<br>
    状态：${record.status}`,
    '调用详情',
    {
      dangerouslyUseHTMLString: true,
      confirmButtonText: '确定'
    }
  )
}

// 获取状态样式
const getStatusClass = (status) => {
  const classes = {
    'success': 'bg-green-100 text-green-700',
    'completed': 'bg-green-100 text-green-700',
    'failed': 'bg-red-100 text-red-700',
    'pending': 'bg-orange-100 text-orange-700',
    'running': 'bg-blue-100 text-blue-700',
    'queued': 'bg-blue-100 text-blue-700'
  }
  return classes[status] || 'bg-gray-100 text-gray-700'
}

// 获取状态文本
const getStatusText = (status) => {
  const texts = {
    'success': '成功',
    'completed': '已完成',
    'failed': '失败',
    'pending': '待执行',
    'running': '执行中',
    'queued': '排队中'
  }
  return texts[status] || status
}

// 格式化日期时间
const formatDateTime = (timestamp) => {
  const date = new Date(timestamp)
  return date.toLocaleString('zh-CN')
}

// 监听筛选条件变化
watch([selectedWorkflow, timeRange, startDate, endDate], () => {
  page.value = 1
  loadMonitorData()
})

onMounted(() => {
  loadMyWorkflows()
})
</script>