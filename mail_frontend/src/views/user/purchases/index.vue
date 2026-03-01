<template>
  <div>
    <!-- Tab 切换 -->
    <div class="bg-white rounded-lg shadow-sm p-1 inline-flex mb-4">
      <button
        @click="activeTab = 'all'"
        :class="[
          'px-5 py-2 rounded-md font-medium text-sm transition-all',
          activeTab === 'all'
            ? 'bg-primary-600 text-white shadow-md'
            : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50'
        ]"
      >
        全部交易
      </button>
      <button
        @click="activeTab = 'recharge'"
        :class="[
          'px-5 py-2 rounded-md font-medium text-sm transition-all',
          activeTab === 'recharge'
            ? 'bg-primary-600 text-white shadow-md'
            : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50'
        ]"
      >
        充值记录
      </button>
      <button
        @click="activeTab = 'expense'"
        :class="[
          'px-5 py-2 rounded-md font-medium text-sm transition-all',
          activeTab === 'expense'
            ? 'bg-primary-600 text-white shadow-md'
            : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50'
        ]"
      >
        支出记录
      </button>
      <button
        @click="activeTab = 'income'"
        :class="[
          'px-5 py-2 rounded-md font-medium text-sm transition-all',
          activeTab === 'income'
            ? 'bg-primary-600 text-white shadow-md'
            : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50'
        ]"
      >
        收益记录
      </button>
    </div>

    <!-- 筛选栏 -->
    <div class="bg-white rounded-lg shadow p-4 mb-4">
      <div class="flex items-center gap-4">
        <!-- 搜索框 -->
        <div class="flex-1">
          <input
            v-model="searchKeyword"
            type="text"
            placeholder="搜索描述..."
            class="w-full px-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            @keyup.enter="handleSearch"
          />
        </div>

        <!-- 按钮组 -->
        <div class="flex gap-2">
          <button
            @click="handleSearch"
            class="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-md hover:bg-primary-700"
          >
            查询
          </button>
          <button
            @click="handleReset"
            class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
          >
            重置
          </button>
        </div>
      </div>
    </div>

    <!-- 订单列表 -->
    <div class="bg-white rounded-lg shadow">
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">交易号</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">交易类型</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">描述</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">金额</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">余额变化</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">时间</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">操作</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="item in transactions" :key="item.id" class="hover:bg-gray-50">
              <td class="px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900">
                {{ item.transaction_no || '#' + item.id }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <span :class="getTypeClass(item.transaction_type)" class="px-2 py-1 text-xs font-medium rounded-full">
                  {{ getTypeName(item.transaction_type) }}
                </span>
              </td>
              <td class="px-6 py-4 text-sm text-gray-900">
                {{ item.description }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm font-medium" :class="item.amount > 0 ? 'text-success-600' : 'text-danger-600'">
                {{ item.amount > 0 ? '+' : '' }}{{ item.amount }} 奶片
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600">
                {{ item.balance_before }} → {{ item.balance_after }}
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
              <td colspan="7" class="px-6 py-12 text-center text-gray-500">
                <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
                <p class="mt-2">加载中...</p>
              </td>
            </tr>
            <tr v-else-if="transactions.length === 0">
              <td colspan="7" class="px-6 py-12 text-center text-gray-500">
                暂无交易记录
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- 分页 -->
      <div class="px-6 py-4 flex items-center justify-between border-t border-gray-200">
        <div class="text-sm text-gray-700">
          共 {{ pagination.total }} 条记录
        </div>
        <div class="flex gap-2">
          <button
            @click="changePage(pagination.page - 1)"
            :disabled="pagination.page === 1"
            class="px-3 py-1 text-sm border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            上一页
          </button>
          <span class="px-3 py-1 text-sm text-gray-700">
            第 {{ pagination.page }} / {{ Math.ceil(pagination.total / pagination.pageSize) }} 页
          </span>
          <button
            @click="changePage(pagination.page + 1)"
            :disabled="pagination.page >= Math.ceil(pagination.total / pagination.pageSize)"
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
            <h3 class="text-lg font-bold">交易详情</h3>
            <button @click="showDetailModal = false" class="text-gray-400 hover:text-gray-600">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          
          <div v-if="selectedItem" class="space-y-4">
            <div class="grid grid-cols-2 gap-4">
              <div>
                <div class="text-sm text-gray-600 mb-1">交易号</div>
                <div class="text-sm font-mono font-medium">{{ selectedItem.transaction_no || '#' + selectedItem.id }}</div>
              </div>
              <div>
                <div class="text-sm text-gray-600 mb-1">交易类型</div>
                <div class="text-sm font-medium">{{ getTypeName(selectedItem.transaction_type) }}</div>
              </div>
              <div>
                <div class="text-sm text-gray-600 mb-1">{{ selectedItem.amount > 0 ? '充值金额' : '消费金额' }}</div>
                <div class="text-sm font-medium" :class="selectedItem.amount > 0 ? 'text-success-600' : 'text-danger-600'">
                  {{ selectedItem.amount > 0 ? '+' : '' }}{{ selectedItem.amount }} 奶片
                </div>
              </div>
              <div>
                <div class="text-sm text-gray-600 mb-1">余额变化</div>
                <div class="text-sm font-medium">{{ selectedItem.balance_before }} → {{ selectedItem.balance_after }}</div>
              </div>
              <div class="col-span-2">
                <div class="text-sm text-gray-600 mb-1">描述</div>
                <div class="text-sm">{{ selectedItem.description }}</div>
              </div>
              
              <!-- 工作流相关信息 -->
              <div v-if="selectedItem.order_no" class="col-span-2">
                <div class="text-sm text-gray-600 mb-1">订单号</div>
                <div class="text-sm font-mono">{{ selectedItem.order_no }}</div>
              </div>
              <div v-if="selectedItem.workflow_id">
                <div class="text-sm text-gray-600 mb-1">工作流ID</div>
                <div class="text-sm font-mono">{{ selectedItem.workflow_id }}</div>
              </div>
              <div v-if="selectedItem.execution_id">
                <div class="text-sm text-gray-600 mb-1">执行ID</div>
                <div class="text-sm font-mono">{{ selectedItem.execution_id }}</div>
              </div>
              
              <div class="col-span-2">
                <div class="text-sm text-gray-600 mb-1">时间</div>
                <div class="text-sm">{{ formatDate(selectedItem.created_at) }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, watch } from 'vue'
import { showMessage } from '@/utils/message'
import api from '@/services/api'
import ActionButton from '@/components/ActionButton/index.vue'

// 数据
const transactions = ref([])
const loading = ref(false)
const searchKeyword = ref('')
const activeTab = ref('all') // Tab状态

// 统计
const summary = reactive({
  total: 0,
  totalSpent: 0
})

// 分页
const pagination = reactive({
  page: 1,
  pageSize: 20,
  total: 0
})

// 详情弹窗
const showDetailModal = ref(false)
const selectedItem = ref(null)

// 加载数据
const loadData = async () => {
  loading.value = true
  try {
    const params = {
      page: pagination.page,
      page_size: pagination.pageSize
    }

    // 根据Tab筛选
    if (activeTab.value === 'recharge') {
      // 充值记录
      params.transaction_type = 'recharge'
    } else if (activeTab.value === 'income') {
      // 收益记录
      params.transaction_type = 'earn'
    } else if (activeTab.value === 'expense') {
      // 支出记录: 消费 + 提现
      params.transaction_type = 'consume,withdraw'
    }
    // 如果是'all'则不传transaction_type,显示全部

    const response = await api.get('/milk-coins/transactions', { params })

    if (response.code === 0) {
      transactions.value = response.data.items || []
      pagination.total = response.data.total || 0

      // 计算统计
      summary.total = response.data.total || 0
      summary.totalSpent = Math.abs(transactions.value.reduce((sum, item) => sum + (item.amount || 0), 0))
    }
  } catch (error) {
    console.error('加载失败:', error)
    showMessage('加载失败', 'error')
  } finally {
    loading.value = false
  }
}

// 搜索
const handleSearch = () => {
  pagination.page = 1
  loadData()
}

// 重置
const handleReset = () => {
  searchKeyword.value = ''
  pagination.page = 1
  loadData()
}

// 翻页
const changePage = (page) => {
  if (page < 1 || page > Math.ceil(pagination.total / pagination.pageSize)) return
  pagination.page = page
  loadData()
}

// 查看详情
const viewDetail = (item) => {
  selectedItem.value = item
  showDetailModal.value = true
}

// 格式化日期
const formatDate = (timestamp) => {
  if (!timestamp) return '-'
  const date = new Date(timestamp)
  return date.toLocaleString('zh-CN')
}

// 获取类型名称
const getTypeName = (type) => {
  const typeMap = {
    'recharge': '充值',
    'consume': '消费',
    'earn': '收益',
    'withdraw': '提现'
  }
  return typeMap[type] || type
}

// 获取类型样式
const getTypeClass = (type) => {
  const classMap = {
    'recharge': 'bg-success-100 text-success-700',
    'consume': 'bg-danger-100 text-danger-700',
    'earn': 'bg-primary-100 text-primary-700',
    'withdraw': 'bg-warning-100 text-warning-700'
  }
  return classMap[type] || 'bg-gray-100 text-gray-700'
}

// 监听Tab切换
watch(activeTab, () => {
  pagination.page = 1
  loadData()
})

onMounted(() => {
  loadData()
})
</script>
