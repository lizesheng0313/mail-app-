<template>
  <div>
    <!-- 页面标题 -->
    <div class="mb-6">
      <h2 class="text-2xl font-bold text-gray-900">我的销售</h2>
      <p class="text-sm text-gray-600 mt-1">查看所有收益记录</p>
    </div>

    <!-- 筛选栏 -->
    <div class="bg-white rounded-lg shadow p-3 mb-3">
      <div class="flex items-center gap-4">
        <!-- 搜索框 -->
        <div class="flex-1">
          <input
            v-model="searchKeyword"
            type="text"
            placeholder="搜索订单号、工作流名称..."
            class="w-full px-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-2 focus:ring-primary-500 focus:border-transparent"
          />
        </div>

        <!-- 订单类型 -->
        <div class="w-40">
          <CustomSelect
            v-model="orderType"
            :options="orderTypeOptions"
            placeholder="全部类型"
          />
        </div>

        <!-- 订单状态 -->
        <div class="w-40">
          <CustomSelect
            v-model="orderStatus"
            :options="orderStatusOptions"
            placeholder="全部状态"
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

    <!-- 数据表格 -->
    <AdminDataTable
      title="收益记录"
      :pagination="pagination"
      :loading="loading"
      :column-count="6"
      @page-change="changePage"
      @page-size-change="changePageSize"
    >
      <template #thead>
        <tr>
          <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">交易号</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">收益类型</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">描述</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">金额</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">时间</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">操作</th>
        </tr>
      </template>

      <template #tbody>
        <tr v-for="order in orders" :key="order.id" class="hover:bg-gray-50">
          <td class="px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900">
            {{ order.transaction_no || '#' + order.id }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap">
            <span :class="getTypeClass(order.related_type)" class="px-2 py-1 text-xs font-medium rounded-full">
              {{ getTypeName(order.related_type) }}
            </span>
          </td>
          <td class="px-6 py-4 text-sm text-gray-900">
            {{ order.description }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-success-600">
            +{{ order.amount }} 奶片
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-600">
            {{ formatDate(order.created_at) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm">
            <ActionButton
              label="详情"
              type="info"
              size="small"
              @click="viewDetail(order)"
            />
          </td>
        </tr>
        <tr v-if="orders.length === 0">
          <td colspan="9" class="px-6 py-12 text-center text-black">
            暂无订单记录
          </td>
        </tr>
      </template>
    </AdminDataTable>
  </div>
</template>

<script setup>
import { ref, onMounted, watch, computed } from 'vue'
import { showMessage } from '@/utils/message'
import { showConfirm, showPrompt, showAlert } from '@/utils/dialog'
import api from '@/services/api'
import CustomSelect from '@/components/CustomSelect/index.vue'
import AdminDataTable from '@/components/AdminDataTable/index.vue'
import ActionButton from '@/components/ActionButton/index.vue'

// 当前用户ID
const userId = ref(parseInt(localStorage.getItem('userId')) || 1)

// 筛选条件
const searchKeyword = ref('')
const orderType = ref('')
const orderStatus = ref('')

// Select选项
const orderTypeOptions = [
  { label: '全部类型', value: '' },
  { label: '新购', value: 'purchase' },
  { label: '续费', value: 'renew' },
  { label: '升级', value: 'upgrade' }
]

const orderStatusOptions = [
  { label: '全部状态', value: '' },
  { label: '待支付', value: 'pending' },
  { label: '已支付', value: 'paid' },
  { label: '已退款', value: 'refunded' },
  { label: '已取消', value: 'cancelled' }
]

// 统计数据
const stats = ref({
  total: 0,
  totalAmount: 0,
  pending: 0,
  pendingAmount: 0,
  completed: 0,
  completedAmount: 0,
  refunded: 0,
  refundedAmount: 0
})

// 订单列表
const orders = ref([])

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
  loadOrders()
}

// 切换每页数量
const changePageSize = (newPageSize) => {
  pageSize.value = newPageSize
  page.value = 1
  loadOrders()
}

// 加载订单列表（改为加载收益记录）
const loadOrders = async () => {
  loading.value = true
  try {
    // 改为查询奶片交易记录（收益类型）
    const res = await api.get('/milk-coins/transactions', {
      params: {
        transaction_type: 'earn',  // 只查询收益记录
        page: page.value,
        page_size: pageSize.value
      }
    })

    if (res.code === 0) {
      orders.value = res.data.items || []
      total.value = res.data.total || 0

      // 更新统计（根据收益记录计算）
      const totalEarned = orders.value.reduce((sum, item) => sum + (item.amount || 0), 0)
      stats.value.total = orders.value.length
      stats.value.totalAmount = totalEarned
      stats.value.completed = orders.value.length
      stats.value.completedAmount = totalEarned
      stats.value.pending = 0
      stats.value.pendingAmount = 0
      stats.value.refunded = 0
      stats.value.refundedAmount = 0
    }
  } catch (error) {
    console.error('加载订单失败:', error)
  } finally {
    loading.value = false
  }
}

// 获取类型样式
const getTypeClass = (type) => {
  const typeMap = {
    'workflow_execution': 'bg-primary-100 text-primary-700',
    'plugin_order': 'bg-success-100 text-success-700',
    'mailbox': 'bg-warning-100 text-warning-700'
  }
  return typeMap[type] || 'bg-gray-100 text-gray-700'
}

// 获取类型名称
const getTypeName = (type) => {
  const typeMap = {
    'workflow_execution': '工作流',
    'plugin_order': '插件',
    'mailbox': '邮箱'
  }
  return typeMap[type] || type
}

// 格式化日期
const formatDate = (timestamp) => {
  if (!timestamp) return '-'
  const date = new Date(timestamp)
  return date.toLocaleString('zh-CN')
}

// 查看详情
const viewDetail = (item) => {
  showAlert(
    `<div class="space-y-2">
      <div><strong>交易号:</strong> ${item.transaction_no || '#' + item.id}</div>
      <div><strong>类型:</strong> ${getTypeName(item.related_type)}</div>
      <div><strong>金额:</strong> +${item.amount} 奶片</div>
      <div><strong>描述:</strong> ${item.description}</div>
      <div><strong>时间:</strong> ${formatDate(item.created_at)}</div>
    </div>`,
    '收益详情'
  )
}

// 获取订单类型文本（保留旧方法避免错误）
const getOrderTypeText = (type) => {
  const texts = {
    'purchase': '新购',
    'renew': '续费',
    'upgrade': '升级'
  }
  return texts[type] || type
}

// 获取订单状态样式
const getOrderStatusClass = (status) => {
  const classes = {
    'pending': 'bg-warning-100 text-warning-700',
    'paid': 'bg-primary-100 text-primary-700',
    'refunded': 'bg-danger-100 text-danger-700',
    'cancelled': 'bg-gray-100 text-gray-700'
  }
  return classes[status] || 'bg-gray-100 text-gray-700'
}

// 获取订单状态文本
const getOrderStatusText = (status) => {
  const texts = {
    'pending': '待支付',
    'paid': '已支付',
    'refunded': '已退款',
    'cancelled': '已取消'
  }
  return texts[status] || status
}

// 格式化日期时间
const formatDateTime = (timestamp) => {
  const date = new Date(timestamp)
  return date.toLocaleString('zh-CN')
}

// 判断是否可以退款
const canRefund = (order) => {
  // 7天内可退款
  const daysSincePaid = (Date.now() - order.paid_at) / 86400000
  return daysSincePaid <= 7
}

// 查看订单详情
const viewOrderDetail = async (order) => {
  await showAlert(
    `订单号：${order.order_no}\n工作流：${order.workflow_name}\n买家：${order.buyer_name}\n金额：¥${order.amount}\n佣金：¥${order.commission}\n状态：${getOrderStatusText(order.status)}`,
    '订单详情'
  )
}

// 退款
const refundOrder = async (order) => {
  const confirmed = await showConfirm(
    `确定要退款订单"${order.order_no}"吗？退款后买家将无法继续使用该工作流。`,
    '退款确认'
  )

  if (confirmed) {
    // TODO: 调用退款API
    showMessage('退款成功', 'success')
    loadOrders()
  }
}

// 导出订单
const exportOrders = () => {
  // TODO: 调用导出API
  showMessage('正在导出订单数据...', 'success')
}

// 查询按钮
const handleSearch = () => {
  page.value = 1 // 重置到第一页
  loadOrders()
}

// 重置按钮
const handleReset = () => {
  searchKeyword.value = ''
  orderType.value = ''
  orderStatus.value = ''
  page.value = 1
  loadOrders()
}

// 不使用自动watch，改为手动触发查询
onMounted(() => {
  loadOrders()
})
</script>