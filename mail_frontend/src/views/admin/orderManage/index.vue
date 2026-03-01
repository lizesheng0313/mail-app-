<template>
  <div class="h-full">
    <!-- 加载状态 -->
    <div v-if="!initialized" class="h-full flex items-center justify-center">
      <div class="text-center">
        <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
        <div class="mt-2 text-black">加载中...</div>
      </div>
    </div>
    
    <!-- 主要内容 -->
    <div v-else class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-full flex flex-col">

      <!-- 搜索栏 -->
      <div class="bg-white rounded-lg shadow-sm border p-4 mb-4">
        <div class="flex items-center space-x-3">
          <CustomSelect
            v-model="filters.type"
            :options="typeOptions"
            placeholder="订单类型"
            @change="handleQuery"
          />
          <CustomSelect
            v-model="filters.status"
            :options="statusOptions"
            placeholder="全部状态"
            @change="handleQuery"
          />
          <BaseInput
            v-model="filters.keyword"
            placeholder="搜索订单号或用户"
            class="w-64"
            size="sm"
          >
            <template #left-icon>
              <svg class="h-4 w-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
            </template>
          </BaseInput>
          <button
            @click="handleQuery"
            class="px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white rounded-md text-sm"
          >
            查询
          </button>
        </div>
      </div>

      <!-- 订单列表 -->
      <AdminDataTable
        title="订单列表"
        :pagination="pagination"
        :loading="loading"
        :show-page-size-selector="true"
        :column-count="8"
        @page-change="changePage"
        @page-size-change="changePageSize"
      >
        <template #thead>
          <tr>
            <th class="px-2 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">类型</th>
            <th class="px-2 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">订单号</th>
            <th class="px-2 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">用户</th>
            <th class="px-2 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">商品</th>
            <th class="px-2 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">金额</th>
            <th class="px-2 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">状态</th>
            <th class="px-2 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">创建时间</th>
            <th class="px-2 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">操作</th>
          </tr>
        </template>
        <template #tbody>
          <tr v-for="order in allOrders" :key="`${order.type}-${order.id}`" class="hover:bg-gray-50">
            <td class="px-2 py-3 whitespace-nowrap">
              <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium"
                :class="{
                  'bg-blue-100 text-blue-800': order.type === 'mailbox',
                  'bg-purple-100 text-purple-800': order.type === 'plugin',
                  'bg-green-100 text-green-800': order.type === 'workflow'
                }">
                {{ order.type === 'mailbox' ? '邮箱' : order.type === 'plugin' ? '插件' : '工作流' }}
              </span>
            </td>
            <td class="px-2 py-3 whitespace-nowrap">
              <code class="text-xs font-mono bg-gray-100 px-1 py-0.5 rounded">{{ order.order_no }}</code>
            </td>
            <td class="px-2 py-3 whitespace-nowrap text-sm">
              <div class="text-black">{{ order.username }}</div>
              <div class="text-gray-500 text-xs">{{ order.user_email }}</div>
            </td>
            <td class="px-2 py-3 whitespace-nowrap text-sm text-black">
              <span v-if="order.type === 'mailbox'">{{ order.item_count }}个邮箱</span>
              <span v-else-if="order.type === 'plugin'">{{ order.item_name || order.product_name }}({{ order.item_count }}天)</span>
              <span v-else-if="order.type === 'workflow'">{{ order.item_name || order.product_name }}</span>
              <span v-else>{{ order.product_name }}</span>
            </td>
            <td class="px-2 py-3 whitespace-nowrap text-sm font-medium text-primary-600">¥{{ order.amount.toFixed(2) }}</td>
            <td class="px-2 py-3 whitespace-nowrap">
              <span :class="getStatusClass(order.status)">
                {{ getStatusText(order.status) }}
              </span>
            </td>
            <td class="px-2 py-3 whitespace-nowrap text-sm text-black">
              {{ formatDate(order.created_at) }}
            </td>
            <td class="px-2 py-3 whitespace-nowrap text-sm font-medium">
              <ActionButton
                v-if="order.status !== 'paid'"
                icon="delete"
                tooltip="删除"
                variant="delete"
                @click="deleteOrder(order)"
              />
            </td>
          </tr>
          <tr v-if="allOrders.length === 0">
            <td colspan="8" class="px-6 py-12 text-center text-black">
              暂无订单数据
            </td>
          </tr>
        </template>
      </AdminDataTable>

    </div>

    <!-- 订单详情弹窗 -->
    <div v-if="showDetailModal && selectedOrder" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg p-6 w-full max-w-2xl max-h-[80vh] overflow-y-auto">
        <h3 class="text-lg font-semibold mb-4">订单详情</h3>

        <div class="space-y-4">
          <!-- 基本信息 -->
          <div class="border-b pb-4">
            <h4 class="font-medium text-black mb-3">基本信息</h4>
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div>
                <span class="text-gray-500">订单号：</span>
                <span class="text-black">{{ selectedOrder.order_no }}</span>
              </div>
              <div>
                <span class="text-gray-500">状态：</span>
                <span :class="getStatusClass(selectedOrder.status)">
                  {{ getStatusText(selectedOrder.status) }}
                </span>
              </div>
              <div>
                <span class="text-gray-500">用户：</span>
                <span class="text-black">{{ selectedOrder.username }} ({{ selectedOrder.user_email }})</span>
              </div>
              <div>
                <span class="text-gray-500">金额：</span>
                <span class="text-primary-600 font-medium">¥{{ selectedOrder.amount.toFixed(2) }}</span>
              </div>
              <div>
                <span class="text-gray-500">支付方式：</span>
                <span class="text-black">{{ selectedOrder.payment_method === 'alipay' ? '支付宝' : selectedOrder.payment_method }}</span>
              </div>
              <div>
                <span class="text-gray-500">创建时间：</span>
                <span class="text-black">{{ formatDate(selectedOrder.created_at) }}</span>
              </div>
              <div v-if="selectedOrder.paid_at">
                <span class="text-gray-500">支付时间：</span>
                <span class="text-black">{{ formatDate(selectedOrder.paid_at) }}</span>
              </div>
              <div v-if="activeTab === 'mailbox' && 'mailbox_count' in selectedOrder">
                <span class="text-gray-500">邮箱数量：</span>
                <span class="text-black">{{ (selectedOrder as MailboxOrder).mailbox_count }}</span>
              </div>
              <div v-if="activeTab === 'plugin' && 'duration_days' in selectedOrder">
                <span class="text-gray-500">时长：</span>
                <span class="text-black">{{ (selectedOrder as PluginOrder).duration_days }}天</span>
              </div>
            </div>
          </div>

          <!-- 支付信息 -->
          <div v-if="('alipay_trade_no' in selectedOrder && selectedOrder.alipay_trade_no) || ('trade_no' in selectedOrder && selectedOrder.trade_no)" class="border-b pb-4">
            <h4 class="font-medium text-black mb-3">支付信息</h4>
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div v-if="('alipay_trade_no' in selectedOrder && selectedOrder.alipay_trade_no) || ('trade_no' in selectedOrder && selectedOrder.trade_no)">
                <span class="text-gray-500">交易号：</span>
                <span class="text-black font-mono text-xs">{{ 'alipay_trade_no' in selectedOrder ? (selectedOrder as MailboxOrder).alipay_trade_no : (selectedOrder as PluginOrder).trade_no }}</span>
              </div>
              <div v-if="'alipay_buyer_id' in selectedOrder && selectedOrder.alipay_buyer_id">
                <span class="text-gray-500">买家ID：</span>
                <span class="text-black">{{ (selectedOrder as MailboxOrder).alipay_buyer_id }}</span>
              </div>
            </div>
          </div>

          <!-- 邮箱订单的邮箱列表 -->
          <div v-if="activeTab === 'mailbox' && 'mailboxes' in selectedOrder && selectedOrder.mailboxes && selectedOrder.mailboxes.length > 0">
            <h4 class="font-medium text-black mb-3">创建的邮箱 ({{ (selectedOrder as MailboxOrder).mailboxes!.length }})</h4>
            <div class="max-h-60 overflow-y-auto border rounded p-3 bg-gray-50">
              <div v-for="(mailbox, index) in (selectedOrder as MailboxOrder).mailboxes" :key="index" class="text-sm py-2 border-b last:border-b-0">
                <div class="flex justify-between items-center">
                  <span class="text-black font-mono">{{ mailbox.email_address }}</span>
                  <span :class="mailbox.is_active ? 'text-green-600' : 'text-gray-400'" class="text-xs">
                    {{ mailbox.is_active ? '激活' : '未激活' }}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="flex justify-end space-x-3 mt-6">
          <button
            @click="closeDetailModal"
            class="px-4 py-2 bg-gray-100 hover:bg-gray-200 text-black rounded-md"
          >
            关闭
          </button>
        </div>
      </div>
    </div>

    <!-- 确认删除对话框 -->
    <ConfirmDialog
      v-if="deleteTarget"
      v-model:visible="showDeleteDialog"
      title="确认删除"
      :message="`确定要删除订单 ${deleteTarget.order_no} 吗？只能删除未支付的订单。`"
      type="danger"
      confirm-text="删除"
      @confirm="confirmDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed, watch } from 'vue'
import api from '@/services/api'
import BaseInput from '@/components/BaseInput/index.vue'
import CustomSelect from '@/components/CustomSelect/index.vue'
import ActionButton from '@/components/ActionButton/index.vue'
import AdminDataTable from '@/components/AdminDataTable/index.vue'
import ConfirmDialog from '@/components/ConfirmDialog/index.vue'
import { showMessage } from '@/utils/message'

// 类型定义
interface MailboxOrder {
  id: number
  order_no: string
  user_id: number
  package_id: number
  package_code: string
  mailbox_count: number
  amount: number
  payment_method: string
  status: string
  alipay_trade_no?: string
  alipay_buyer_id?: string
  paid_at?: string
  created_at: string
  username: string
  user_email: string
  package_name: string
  mailboxes?: Array<{
    email_address: string
    display_name: string
    is_active: boolean
    created_at: string
  }>
}

interface PluginOrder {
  id: number
  order_no: string
  user_id: number
  plugin_id: string
  pricing_id: number
  amount: number
  duration_days: number
  status: string
  payment_method: string
  trade_no?: string
  paid_at?: string
  created_at: string
  username: string
  user_email: string
  plugin_name: string
}

interface OrderStats {
  mailbox_orders: {
    total: number
    paid: number
    pending: number
    total_amount: number
  }
  plugin_orders: {
    total: number
    paid: number
    pending: number
    total_amount: number
  }
}

interface ApiResponse<T = any> {
  code: number
  message?: string
  data?: T
}

// 响应式数据
const initialized = ref(false)
const mailboxOrders = ref<MailboxOrder[]>([])
const pluginOrders = ref<PluginOrder[]>([])
const loading = ref(false)
const showDetailModal = ref(false)
const showDeleteDialog = ref(false)
const selectedOrder = ref<MailboxOrder | PluginOrder | null>(null)
const deleteTarget = ref<(MailboxOrder | PluginOrder) | null>(null)

// 筛选器
const filters = reactive({
  type: '',
  status: '',
  keyword: ''
})

// 分页信息
const pagination = reactive({
  page: 1,
  page_size: 20,
  total: 0,
  total_pages: 0
})

// 类型选项
const typeOptions = computed(() => [
  { label: '全部类型', value: '' },
  { label: '邮箱订单', value: 'mailbox' },
  { label: '插件订单', value: 'plugin' },
  { label: '工作流订单', value: 'workflow' }
])

// 状态选项
const statusOptions = computed(() => [
  { label: '全部状态', value: '' },
  { label: '待支付', value: 'pending' },
  { label: '已支付', value: 'paid' },
  { label: '已取消', value: 'cancelled' },
  { label: '已失败', value: 'failed' }
])

// 合并所有订单
const allOrders = computed(() => {
  const orders: any[] = []

  // 因为后端已经按类型筛选了，直接合并两个数组
  orders.push(...mailboxOrders.value.map(o => ({ ...o, type: 'mailbox' })))
  orders.push(...pluginOrders.value.map(o => ({ ...o, type: 'plugin' })))

  // 按创建时间排序（后端已经排序，这里保持即可）
  return orders
})

// 初始化
onMounted(async () => {
  try {
    await loadOrders()
  } catch (error) {
    console.error('初始化失败:', error)
    showMessage('初始化失败', 'error')
  } finally {
    initialized.value = true
  }
})

// 加载订单列表
const loadOrders = async (page = 1) => {
  loading.value = true
  try {
    const params = {
      page,
      page_size: pagination.page_size,
      order_type: filters.type || undefined,
      status: filters.status || undefined,
      keyword: filters.keyword || undefined
    }

    // 调用统一接口
    const response = await api.get('/admin/orders/all', { params })

    if (response.code === 0 && response.data) {
      // 根据订单类型分组
      const orders = response.data.orders || []
      mailboxOrders.value = orders.filter((o: any) => o.type === 'mailbox')
      pluginOrders.value = orders.filter((o: any) => o.type === 'plugin')

      pagination.page = response.data.page
      pagination.total = response.data.total
      pagination.total_pages = response.data.total_pages
    } else {
      showMessage(response.message || '加载订单列表失败', 'error')
    }
  } catch (error) {
    console.error('加载订单列表失败:', error)
    showMessage('加载订单列表失败', 'error')
  } finally {
    loading.value = false
  }
}

// 查询
const handleQuery = () => {
  pagination.page = 1
  loadOrders(1)
}

// 查看订单详情
const viewOrderDetail = async (order: MailboxOrder | PluginOrder, type: 'mailbox' | 'plugin') => {
  try {
    const endpoint = type === 'mailbox' 
      ? `/admin/orders/mailbox/${order.id}` 
      : `/admin/orders/plugin/${order.id}`
    
    const response = await api.get(endpoint) as ApiResponse<MailboxOrder | PluginOrder>
    if (response.code === 0 && response.data) {
      selectedOrder.value = response.data
      showDetailModal.value = true
    } else {
      showMessage(response.message || '获取订单详情失败', 'error')
    }
  } catch (error) {
    console.error('获取订单详情失败:', error)
    showMessage('获取订单详情失败', 'error')
  }
}

// 关闭详情弹窗
const closeDetailModal = () => {
  showDetailModal.value = false
  selectedOrder.value = null
}

// 删除订单
const deleteOrder = (order: any) => {
  deleteTarget.value = order
  showDeleteDialog.value = true
}

// 确认删除
const confirmDelete = async () => {
  if (!deleteTarget.value) return
  
  try {
    const order = deleteTarget.value
    const type = order.type
    const endpoint = type === 'mailbox' 
      ? `/admin/orders/mailbox/${order.id}` 
      : `/admin/orders/plugin/${order.id}`
    
    const response = await api.delete(endpoint) as ApiResponse
    if (response.code === 0) {
      showMessage('删除成功', 'success')
      await loadOrders(pagination.page)
    } else {
      showMessage(response.message || '删除失败', 'error')
    }
  } catch (error) {
    console.error('删除订单失败:', error)
    showMessage('删除订单失败', 'error')
  } finally {
    deleteTarget.value = null
  }
}

// 切换页码
const changePage = (page: number) => {
  if (page >= 1 && page <= pagination.total_pages) {
    loadOrders(page)
  }
}

// 改变每页显示数量
const changePageSize = (size: number) => {
  filters.page_size = size
  loadOrders(1)
}

// 格式化日期
const formatDate = (dateStr: string | undefined) => {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 获取状态样式
const getStatusClass = (status: string) => {
  const classes: Record<string, string> = {
    'pending': 'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800',
    'paid': 'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800',
    'cancelled': 'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800',
    'failed': 'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800',
    'expired': 'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800'
  }
  return classes[status] || classes.pending
}

// 获取状态文本
const getStatusText = (status: string) => {
  const texts: Record<string, string> = {
    'pending': '待支付',
    'paid': '已支付',
    'cancelled': '已取消',
    'failed': '支付失败',
    'expired': '已过期'
  }
  return texts[status] || status
}
</script>
