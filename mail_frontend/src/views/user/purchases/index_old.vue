<template>
  <div>
    <!-- 页面标题 -->
    <div class="mb-3">
      <h2 class="text-xl font-bold text-gray-900">我的购买</h2>
    </div>

    <!-- 购买统计 -->
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-3 mb-3">
      <div class="bg-white rounded-xl shadow p-3">
        <div class="mb-2">
          <span class="text-sm font-medium text-gray-600">已购买</span>
        </div>
        <p class="text-3xl font-bold text-gray-900">{{ summary.total }}</p>
        <p class="text-xs text-gray-500 mt-1">总数量</p>
      </div>

      <div class="bg-white rounded-xl shadow p-3">
        <div class="mb-2">
          <span class="text-sm font-medium text-gray-600">使用中</span>
        </div>
        <p class="text-3xl font-bold text-primary-600">{{ summary.active }}</p>
        <p class="text-xs text-gray-500 mt-1">可用状态</p>
      </div>

      <div class="bg-white rounded-xl shadow p-3">
        <div class="mb-2">
          <span class="text-sm font-medium text-gray-600">即将到期</span>
        </div>
        <p class="text-3xl font-bold text-warning-600">{{ summary.expiring }}</p>
        <p class="text-xs text-gray-500 mt-1">7天内</p>
      </div>

      <div class="bg-white rounded-xl shadow p-3">
        <div class="mb-2">
          <span class="text-sm font-medium text-gray-600">总消费</span>
        </div>
        <p class="text-3xl font-bold text-danger-600">¥{{ summary.totalSpent }}</p>
        <p class="text-xs text-gray-500 mt-1">累计金额</p>
      </div>
    </div>

    <!-- 筛选栏 -->
    <div class="bg-white rounded-lg shadow p-3 mb-3">
      <div class="flex items-center gap-4">
        <!-- 搜索框 -->
        <div class="flex-1">
          <input
            v-model="searchKeyword"
            type="text"
            placeholder="搜索工作流名称..."
            class="w-full px-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-2 focus:ring-primary-500 focus:border-transparent"
          />
        </div>

        <!-- 状态筛选 -->
        <div class="w-40">
          <CustomSelect
            v-model="filterStatus"
            :options="filterStatusOptions"
            placeholder="全部状态"
          />
        </div>

        <!-- 排序 -->
        <div class="w-40">
          <CustomSelect
            v-model="sortBy"
            :options="sortByOptions"
            placeholder="排序方式"
          />
        </div>

        <!-- 按钮组 -->
        <div class="flex gap-2">
          <button
            @click="handleSearch"
            class="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-md hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
          >
            查询
          </button>
          <button
            @click="handleReset"
            class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
          >
            重置
          </button>
        </div>
      </div>
    </div>

    <!-- 购买列表 -->
    <AdminDataTable
      title="已购工作流"
      :pagination="pagination"
      :loading="loading"
      :column-count="8"
      @page-change="changePage"
      @page-size-change="changePageSize"
    >
      <template #thead>
        <tr>
          <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">工作流</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">作者</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">购买时间</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">使用情况</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">最终结果</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">有效期</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">状态</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">操作</th>
        </tr>
      </template>
      <template #tbody>
        <tr v-for="item in purchases" :key="item.id" class="hover:bg-gray-50">
          <td class="px-6 py-4 whitespace-nowrap">
            <div>
              <div class="text-sm font-medium text-black">{{ item.workflow_name }}</div>
              <div class="text-sm text-gray-500">{{ item.workflow_description }}</div>
            </div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-black">
            {{ item.author_name }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-black">
            {{ formatDate(item.purchase_time) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap">
            <div class="text-sm text-black">
              <span v-if="item.pricing_model === 'per_use'">
                已用 {{ item.used_count }}/{{ item.total_count }}
              </span>
              <span v-else>
                本月已用 {{ item.month_count }} 次
              </span>
            </div>
            <div class="text-xs text-gray-500">
              {{ item.pricing_model === 'per_use' ? '按次计费' : '订阅制' }}
            </div>
          </td>
          <td class="px-6 py-4">
            <div v-if="item.last_execution_result" class="text-sm">
              <div class="font-medium text-black">{{ item.last_execution_result.result_summary || '已执行' }}</div>
              <div v-if="item.last_execution_result.account_info" class="text-xs text-gray-600 mt-1 space-y-0.5">
                <div v-if="item.last_execution_result.account_info.username">
                  账号: {{ item.last_execution_result.account_info.username }}
                </div>
                <div v-if="item.last_execution_result.account_info.password" class="font-mono">
                  密码: {{ item.last_execution_result.account_info.password }}
                </div>
              </div>
              <div class="text-xs text-gray-400 mt-1">
                {{ formatDate(item.last_execution_result.executed_at) }}
              </div>
            </div>
            <div v-else class="text-sm text-gray-400">暂无执行记录</div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap">
            <div v-if="item.expire_at" class="text-sm">
              <span :class="getExpireClass(item.expire_at)">
                {{ formatDate(item.expire_at) }}
              </span>
              <div class="text-xs text-gray-500">
                {{ getDaysRemaining(item.expire_at) }}
              </div>
            </div>
            <div v-else class="text-sm text-gray-500">永久有效</div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap">
            <span :class="getStatusClass(item.status)" class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full">
              {{ getStatusText(item.status) }}
            </span>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm font-medium space-x-2">
            <ActionButton
              icon="play"
              tooltip="使用"
              variant="primary"
              :disabled="item.status !== 'active'"
              @click="useWorkflow(item)"
            />
            <ActionButton
              v-if="item.status === 'active' && item.pricing_model === 'subscription'"
              icon="refresh"
              tooltip="续费"
              variant="secondary"
              @click="renewWorkflow(item)"
            />
            <ActionButton
              v-if="item.status === 'active' && item.pricing_model === 'per_use' && item.used_count >= item.total_count"
              icon="shopping-cart"
              tooltip="购买次数"
              variant="primary"
              @click="buyMore(item)"
            />
            <ActionButton
              icon="eye"
              tooltip="详情"
              variant="view"
              @click="viewDetail(item)"
            />
          </td>
        </tr>
        <tr v-if="purchases.length === 0">
          <td colspan="8" class="px-6 py-12 text-center text-black">
            暂无购买记录
          </td>
        </tr>
      </template>
    </AdminDataTable>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { showMessage } from '@/utils/message'
import { showConfirm, showPrompt, showAlert } from '@/utils/dialog'
import { getMyPurchases, purchaseWorkflow } from '@/api/workflowMarket'
import AdminDataTable from '@/components/AdminDataTable/index.vue'
import ActionButton from '@/components/ActionButton/index.vue'
import CustomSelect from '@/components/CustomSelect/index.vue'

const router = useRouter()

// 当前用户ID（需要从store或路由获取）
const userId = ref(parseInt(localStorage.getItem('userId')) || 1)

// 搜索和筛选
const searchKeyword = ref('')
const filterStatus = ref('')
const sortBy = ref('purchase_time')

// Select选项
const filterStatusOptions = [
  { label: '全部状态', value: '' },
  { label: '使用中', value: 'active' },
  { label: '已过期', value: 'expired' },
  { label: '已暂停', value: 'suspended' }
]

const sortByOptions = [
  { label: '购买时间', value: 'purchase_time' },
  { label: '使用次数', value: 'usage' },
  { label: '到期时间', value: 'expire_time' }
]

// 统计数据
const summary = ref({
  total: 0,
  active: 0,
  expiring: 0,
  totalSpent: 0
})

// 购买列表
const purchases = ref([])

// 分页
const page = ref(1)
const pageSize = ref(20)
const total = ref(0)

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
  loadPurchases()
}

// 切换每页数量
const changePageSize = (newPageSize) => {
  pageSize.value = newPageSize
  page.value = 1
  loadPurchases()
}

// 加载状态
const loading = ref(false)

// 加载购买列表（包含统计数据）
const loadPurchases = async () => {
  loading.value = true
  try {
    const res = await getMyPurchases({
      user_id: userId.value,
      status: filterStatus.value || null,
      keyword: searchKeyword.value || null,
      sort_by: sortBy.value,
      page: page.value,
      page_size: pageSize.value
    })

    if (res.code === 0) {
      purchases.value = res.data.items || []
      total.value = res.data.total || 0

      // 更新统计数据
      if (res.data.summary) {
        summary.value = {
          total: res.data.summary.total || 0,
          active: res.data.summary.active || 0,
          expiring: res.data.summary.expiring || 0,
          totalSpent: res.data.summary.total_spent || 0
        }
      }
    }
  } catch (error) {
    console.error('加载购买记录失败:', error)
  } finally {
    loading.value = false
  }
}

// 获取到���样式
const getExpireClass = (expireAt) => {
  const days = Math.floor((expireAt - Date.now()) / 86400000)
  if (days <= 7) return 'text-red-600'
  if (days <= 30) return 'text-orange-600'
  return 'text-gray-900'
}

// 获取剩余天数
const getDaysRemaining = (expireAt) => {
  const days = Math.floor((expireAt - Date.now()) / 86400000)
  if (days < 0) return '已过期'
  if (days === 0) return '今天到期'
  if (days === 1) return '明天到期'
  return `还剩 ${days} 天`
}

// 获取状态样式
const getStatusClass = (status) => {
  const classes = {
    'active': 'bg-green-100 text-green-700',
    'expired': 'bg-red-100 text-red-700',
    'suspended': 'bg-gray-100 text-gray-700'
  }
  return classes[status] || 'bg-gray-100 text-gray-700'
}

// 获取状态文本
const getStatusText = (status) => {
  const texts = {
    'active': '使用中',
    'expired': '已过期',
    'suspended': '已暂停'
  }
  return texts[status] || status
}

// 格式化日期
const formatDate = (timestamp) => {
  const date = new Date(timestamp)
  return date.toLocaleDateString('zh-CN')
}

// 使用工作流
const useWorkflow = async (item) => {
  if (item.status !== 'active') {
    showMessage('该工作流不可用', 'info')
    return
  }

  if (item.pricing_model === 'per_use' && item.used_count >= item.total_count) {
    const confirmed = await showConfirm(
      '可用次数已用完，是否购买更多次数？',
      '提示'
    )
    
    if (confirmed) {
      buyMore(item)
    }
    return
  }

  // 跳转到工作流执行页面
  router.push(`/workflows/${item.workflow_id}/execute`)
}

// 续费
const renewWorkflow = async (item) => {
  const confirmed = await showConfirm(
    `确定续费工作流"${item.workflow_name}"吗？`,
    '续费确认'
  )
  
  if (!confirmed) return

  try {
    // 调用购买API进行续费（相当于再次购买）
    const res = await purchaseWorkflow(item.workflow_id, userId.value, 1)
    if (res.success) {
      showMessage('续费成功', 'success')
      loadPurchases()
    } else {
      showMessage(res.message || '续费失败', 'error')
    }
  } catch (error) {
    console.error('续费失败:', error)
    showMessage(error.response?.data?.detail || '续费失败', 'error')
  }
}

// 购买更多次数
const buyMore = async (item) => {
  const value = await showPrompt('请输入要购买的次数', '购买次数')
  
  if (!value || !/^\d+$/.test(value)) {
    showMessage('请输入有效的数字', 'error')
    return
  }

  try {
    const quantity = parseInt(value)
    const res = await purchaseWorkflow(item.workflow_id, userId.value, quantity)
    if (res.success) {
      showMessage(`购买 ${value} 次成功`, 'success')
      loadPurchases()
    } else {
      showMessage(res.message || '购买失败', 'error')
    }
  } catch (error) {
    console.error('购买失败:', error)
    showMessage(error.response?.data?.detail || '购买失败', 'error')
  }
}

// 查看详情
const viewDetail = (item) => {
  router.push(`/market/workflow/${item.workflow_id}`)
}

// 前往市场
const goToMarket = () => {
  router.push('/market')
}

// 查询按钮
const handleSearch = () => {
  page.value = 1 // 重置到第一页
  loadPurchases()
}

// 重置按钮
const handleReset = () => {
  searchKeyword.value = ''
  filterStatus.value = ''
  sortBy.value = 'purchase_time'
  page.value = 1
  loadPurchases()
}

onMounted(() => {
  loadPurchases()
})
</script>