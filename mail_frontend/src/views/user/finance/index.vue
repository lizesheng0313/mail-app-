<template>
  <div>
    <!-- 财务概览 -->
    <div>
      <!-- 财务概览卡片 - 简洁版 -->
      <div class="grid grid-cols-2 lg:grid-cols-4 gap-3 mb-3">
        <div class="bg-white rounded-xl shadow p-4 ">
          <div class="flex items-center justify-between mb-3">
            <span class="text-sm font-medium text-gray-600">奶片余额</span>
            <div class="h-10 w-10 bg-primary-50 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
          <p class="text-3xl font-bold text-gray-900">{{ balance.available }}</p>
          <p class="text-xs text-gray-500 mt-1">可提现</p>
        </div>

        <div class="bg-white rounded-xl shadow p-4 ">
          <div class="flex items-center justify-between mb-3">
            <span class="text-sm font-medium text-gray-600">冻结奶片</span>
            <div class="h-10 w-10 bg-primary-50 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
              </svg>
            </div>
          </div>
          <p class="text-3xl font-bold text-gray-900">{{ balance.frozen }}</p>
          <p class="text-xs text-gray-500 mt-1">提现处理中</p>
        </div>

        <div class="bg-white rounded-xl shadow p-4 ">
          <div class="flex items-center justify-between mb-3">
            <span class="text-sm font-medium text-gray-600">本月收益</span>
            <div class="h-10 w-10 bg-primary-50 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
              </svg>
            </div>
          </div>
          <p class="text-3xl font-bold text-gray-900">{{ balance.monthIncome }}</p>
          <p class="text-xs text-primary-600 mt-1">+{{ balance.monthGrowth }}%</p>
        </div>

        <div class="bg-white rounded-xl shadow p-4 ">
          <div class="flex items-center justify-between mb-3">
            <span class="text-sm font-medium text-gray-600">累计收益</span>
            <div class="h-10 w-10 bg-primary-50 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z" />
              </svg>
            </div>
          </div>
          <p class="text-3xl font-bold text-gray-900">{{ balance.totalIncome }}</p>
          <p class="text-xs text-gray-500 mt-1">销售收入</p>
        </div>
      </div>

      <!-- 充值和提现 - 三列布局 -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-3">
        <!-- 充值 -->
        <div id="recharge" class="bg-white rounded-xl shadow-lg p-4 flex flex-col">
          <div class="flex items-center mb-3">
            <div class="h-10 w-10 bg-primary-100 rounded-lg flex items-center justify-center mr-3">
              <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
              </svg>
            </div>
            <h3 class="text-lg font-bold text-gray-900">充值奶片</h3>
          </div>

          <div class="flex-1">
            <div class="mb-3">
              <label class="block text-sm font-medium text-gray-700 mb-2">充值金额</label>
              <div class="relative">
                <span class="absolute left-3 top-2.5 text-gray-500 font-medium">¥</span>
                <input
                  v-model="rechargeAmount"
                  type="number"
                  step="0.01"
                  min="1"
                  placeholder="0.00"
                  class="w-full pl-8 pr-3 py-2 border-2 border-gray-200 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                />
              </div>
            </div>

            <div class="mb-3">
              <div class="grid grid-cols-3 gap-2">
                <button
                  v-for="amount in [10, 50, 100, 300, 500, 1000]"
                  :key="amount"
                  @click="rechargeAmount = amount"
                  class="px-3 py-2 border-2 border-gray-200 rounded-lg hover:border-primary-500 hover:bg-primary-50 hover:text-primary-600 text-sm font-medium transition-all"
                >
                  {{ amount }}
                </button>
              </div>
            </div>

            <div class="bg-primary-50 rounded-lg p-3 mb-3">
              <div class="text-center">
                <p class="text-xs text-gray-600 mb-1">获得奶片</p>
                <p class="text-2xl font-bold text-primary-600">{{ rechargeAmount || '0' }}</p>
              </div>
            </div>
          </div>

          <div>
            <button
              @click="submitRecharge"
              :disabled="!canRecharge"
              class="w-full px-4 py-2.5 bg-primary-600 text-white rounded-lg hover:bg-primary-700 disabled:from-gray-300 disabled:to-gray-300 disabled:cursor-not-allowed font-medium shadow-md hover:shadow-lg transition-all"
            >
              立即充值
            </button>
            <p class="text-xs text-center text-gray-500 mt-2">1元 = 1奶片</p>
          </div>
        </div>

        <!-- 提现 -->
        <div class="bg-white rounded-xl shadow-lg p-4 flex flex-col">
          <div class="flex items-center mb-3">
            <div class="h-10 w-10 bg-primary-100 rounded-lg flex items-center justify-center mr-3">
              <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2m2 4h10a2 2 0 002-2v-6a2 2 0 00-2-2H9a2 2 0 00-2 2v6a2 2 0 002 2zm7-5a2 2 0 11-4 0 2 2 0 014 0z" />
              </svg>
            </div>
            <h3 class="text-lg font-bold text-gray-900">提现奶片</h3>
          </div>

          <div class="flex-1">
            <div class="mb-3">
              <label class="block text-sm font-medium text-gray-700 mb-2">提现数量</label>
              <div class="relative">
                <input
                  v-model="withdrawAmount"
                  type="number"
                  step="1"
                  min="0"
                  :max="balance.available"
                  @input="handleWithdrawInput"
                  placeholder="0"
                  class="w-full pl-3 pr-16 py-2 border-2 border-gray-200 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                />
                <button
                  @click="withdrawAmount = balance.available"
                  class="absolute right-2 top-1.5 px-2 py-1 text-xs text-primary-600 hover:bg-primary-50 rounded font-medium"
                >
                  全部
                </button>
              </div>
              <p class="text-xs text-gray-500 mt-1">最低4，余额 {{ balance.available }}</p>
            </div>

            <div class="mb-3">
              <label class="block text-sm font-medium text-gray-700 mb-2">收款账户</label>
              <CustomSelect
                v-model="selectedAccount"
                :options="accountOptions"
                placeholder="请选择收款账户"
              />
            </div>

            <div class="bg-gray-50 rounded-lg p-3 mb-3 space-y-2">
              <div class="flex justify-between text-xs text-gray-600">
                <span>提现金额</span>
                <span class="font-medium">¥{{ withdrawAmount || '0' }}</span>
              </div>
              <div v-if="feeRate > 0" class="flex justify-between text-xs text-gray-600">
                <span>手续费({{ feeRate }}%)</span>
                <span class="font-medium text-red-600">-¥{{ withdrawFee }}</span>
              </div>
              <div class="border-t border-gray-200 pt-2 flex justify-between">
                <span class="text-sm font-medium text-gray-900">实际到账</span>
                <span class="text-lg font-bold text-primary-600">¥{{ actualAmount }}</span>
              </div>
            </div>
          </div>

          <div>
            <button
              @click="handleWithdrawClick"
              class="w-full px-4 py-2.5 bg-primary-600 text-white rounded-lg hover:bg-primary-700 disabled:bg-gray-300 disabled:cursor-not-allowed font-medium shadow-md hover:shadow-lg transition-all"
              :class="{ 'bg-gray-300': !canWithdraw }"
            >
              申请提现
            </button>
            <p v-if="!selectedAccount && accounts.length === 0" class="text-xs text-center text-amber-600 mt-2">
              请先添加收款账户
            </p>
            <p v-else-if="!selectedAccount" class="text-xs text-center text-amber-600 mt-2">
              请选择收款账户
            </p>
            <p v-else-if="parseFloat(withdrawAmount) < 4" class="text-xs text-center text-amber-600 mt-2">
              最低提现4奶片
            </p>
            <p v-else class="text-xs text-center text-gray-500 mt-2">1-3个工作日到账</p>
          </div>
        </div>

        <!-- 收款账户卡片 -->
        <div class="bg-white rounded-xl shadow-lg p-4">
          <div class="flex items-center justify-between mb-3">
            <div class="flex items-center">
              <div class="h-10 w-10 bg-primary-100 rounded-lg flex items-center justify-center mr-3">
                <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
                </svg>
              </div>
              <h3 class="text-lg font-bold text-gray-900">收款账户</h3>
            </div>
            <button
              @click="showAddAccountDialog"
              class="text-primary-600 hover:text-primary-700 p-1.5 hover:bg-primary-50 rounded-lg transition-all"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
              </svg>
            </button>
          </div>

          <div class="space-y-2 max-h-[280px] overflow-y-auto">
            <div
              v-for="account in accounts"
              :key="account.id"
              class="border-2 border-gray-100 rounded-lg p-3 hover:border-primary-200 hover:bg-primary-50 transition-all group"
            >
              <div class="flex justify-between items-start">
                <div class="flex-1 min-w-0">
                  <div class="flex items-center mb-1">
                    <span class="text-sm font-semibold text-gray-900">{{ account.type_name }}</span>
                    <span v-if="account.is_default" class="ml-2 px-2 py-0.5 text-xs bg-primary-500 text-white rounded-full">
                      默认
                    </span>
                  </div>
                  <p class="text-xs text-gray-600 truncate">{{ account.account_name }}</p>
                  <p class="text-xs text-gray-500">{{ maskAccountNo(account.account_no) }}</p>
                </div>
                <div class="flex space-x-1 opacity-0 group-hover:opacity-100 transition-opacity">
                  <button
                    v-if="!account.is_default"
                    @click="setDefaultAccount(account)"
                    class="p-1 text-primary-600 hover:bg-primary-100 rounded"
                    title="设为默认"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                    </svg>
                  </button>
                  <button
                    @click="deleteAccount(account)"
                    class="p-1 text-red-600 hover:bg-red-100 rounded"
                    title="删除"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                </div>
              </div>
            </div>

            <div v-if="accounts.length === 0" class="text-center py-8">
              <svg class="mx-auto h-10 w-10 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
              </svg>
              <p class="text-xs text-gray-500 mt-2">暂无账户</p>
              <button
                @click="showAddAccountDialog"
                class="mt-2 text-xs text-primary-600 hover:text-primary-700 font-medium"
              >
                + 添加第一个账户
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 充值二维码弹窗 -->
    <div v-if="showQrCode" class="fixed inset-0 bg-gray-900 bg-opacity-75 flex items-center justify-center z-50" @click="cancelRecharge">
      <div class="bg-white rounded-xl shadow-2xl max-w-md w-full mx-4" @click.stop>
        <div class="p-6 text-center">
          <div class="flex items-center justify-center gap-2 mb-2">
            <img src="@/assets/img/zhi.svg" alt="支付宝" class="w-8 h-8" />
            <h3 class="text-xl font-bold text-gray-900">请使用支付宝扫码支付</h3>
          </div>
          <div class="text-sm text-gray-600 mb-4">充值金额：<span class="text-2xl font-bold text-primary-600">¥{{ rechargeAmount }}</span></div>
          <div class="mb-4 p-4 bg-white border-2 border-gray-200 rounded-lg inline-block">
            <canvas ref="qrcodeCanvas"></canvas>
          </div>
          <div class="flex items-center justify-center gap-2 mb-2">
            <img src="@/assets/img/zhi.svg" alt="支付宝" class="w-5 h-5" />
            <span class="text-sm text-gray-600">打开支付宝 APP 扫描上方二维码</span>
          </div>
          <p class="text-xs text-gray-500 mb-6">请在15分钟内完成支付，支付成功后会自动充值奶片</p>
          <div class="flex space-x-3">
            <button
              @click="cancelRecharge"
              class="flex-1 px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50 transition-colors"
            >
              取消支付
            </button>
            <button
              @click="checkOrderStatus"
              class="flex-1 px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors"
            >
              我已支付
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- 添加收款账户弹窗 -->
  <div v-if="showAddAccountModal" class="fixed inset-0 bg-gray-900 bg-opacity-75 flex items-center justify-center z-50" @click="closeAddAccountModal">
    <div class="bg-white rounded-xl shadow-2xl max-w-md w-full mx-4" @click.stop>
      <div class="p-6">
        <h3 class="text-xl font-bold text-gray-900 mb-4">添加收款账户</h3>

        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-2">收款人姓名 <span class="text-red-500">*</span></label>
          <input
            v-model="newAccount.name"
            type="text"
            placeholder="真实姓名"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-primary-500"
          />
        </div>

        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-2">手机号 <span class="text-red-500">*</span></label>
          <input
            v-model="newAccount.phone"
            type="tel"
            placeholder="手机号码"
            maxlength="11"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-primary-500"
          />
          <p class="text-xs text-gray-500 mt-1">用于接收提现通知</p>
        </div>

        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-2">银行卡号 <span class="text-red-500">*</span></label>
          <input
            v-model="newAccount.account"
            type="text"
            placeholder="银行卡号"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-primary-500"
          />
        </div>

        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-2">银行名称 <span class="text-red-500">*</span></label>
          <input
            v-model="newAccount.bank_name"
            type="text"
            placeholder="如：中国工商银行"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-primary-500"
          />
        </div>

        <div class="mb-6">
          <label class="block text-sm font-medium text-gray-700 mb-2">开户行 <span class="text-red-500">*</span></label>
          <input
            v-model="newAccount.bank_branch"
            type="text"
            placeholder="如：北京市朝阳区支行"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-primary-500"
          />
        </div>

        <div class="flex space-x-3">
          <button
            @click="closeAddAccountModal"
            class="flex-1 px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50"
          >
            取消
          </button>
          <button
            @click="submitAddAccount"
            :disabled="!newAccount.name || !newAccount.phone || !newAccount.account || !newAccount.bank_name || !newAccount.bank_branch"
            class="flex-1 px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            确定
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup>
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { showMessage } from '@/utils/message'
import { showConfirm, showPrompt, showAlert } from '@/utils/dialog'
import { getBalance, createRecharge, rechargeCallback, createWithdrawal, getFeeConfig } from '@/api/milkCoin'
import { getUserAccounts, createAccount, setDefaultAccount as setDefault, deleteAccount as delAccount } from '@/api/paymentAccount'
import { useUserStore } from '@/stores/user'
import api from '@/services/api'
import AdminDataTable from '@/components/AdminDataTable/index.vue'
import CustomSelect from '@/components/CustomSelect/index.vue'
import QRCode from 'qrcode'

// 从 userStore 获取当前用户ID
const userStore = useUserStore()
const userId = computed(() => userStore.user?.id || null)

// 充值相关
const rechargeAmount = ref('')

// 账户余额
const balance = ref({
  available: 0,
  frozen: 0,
  monthIncome: 0,
  monthGrowth: 0,
  totalIncome: 0,
  totalWithdraw: 0
})

// 提现相关
const withdrawAmount = ref('')
const selectedAccount = ref('')
const feeRate = ref(0) // 手续费率（提现不收手续费，改为0）
const feeConfig = ref(null) // 手续费配置

// 收款账户
const accounts = ref([])

// 添加账户模态框
const showAddAccountModal = ref(false)
const newAccount = ref({
  name: '',
  phone: '',
  account: '',
  bank_name: '',
  bank_branch: ''
})

// 账户选项（用于 CustomSelect）
const accountOptions = computed(() => {
  return accounts.value.map(account => ({
    label: `${account.type_name} - ${account.account_no}`,
    value: String(account.id)
  }))
})



// 计算提现手续费
const withdrawFee = computed(() => {
  const amount = parseFloat(withdrawAmount.value) || 0
  return (amount * feeRate.value / 100).toFixed(2)
})

// 计算实际到账金额
const actualAmount = computed(() => {
  const amount = parseFloat(withdrawAmount.value) || 0
  const fee = parseFloat(withdrawFee.value)
  return (amount - fee).toFixed(2)
})

// 是否可以充值
const canRecharge = computed(() => {
  const amount = parseFloat(rechargeAmount.value) || 0
  return amount >= 1
})

// 是否可以提现
const canWithdraw = computed(() => {
  const amount = parseFloat(withdrawAmount.value) || 0
  return amount >= 100 && amount <= balance.value.available && selectedAccount.value
})

// 加载余额数据
const loadBalance = async () => {
  try {
    const res = await getBalance()
    if (res.code === 0 && res.data) {
      balance.value = {
        available: res.data.balance || 0,
        frozen: res.data.frozen || 0,
        monthIncome: 0,  // TODO: 需要额外的统计接口
        monthGrowth: 0,
        totalIncome: res.data.total_earned || 0,
        totalWithdraw: res.data.total_withdrawn || 0
      }
    }
  } catch (error) {
    console.error('加载余额失败:', error)
  }
}

// 加载收款账户
const loadAccounts = async () => {
  try {
    const res = await getUserAccounts()
    if (res.code === 0 && res.data) {
      accounts.value = (res.data.accounts || res.data || []).map(account => ({
        ...account,
        type_name: getAccountTypeName(account.account_type)
      }))
    }
  } catch (error) {
    console.error('加载收款账户失败:', error)
  }
}

// 获取账户类型名称
const getAccountTypeName = (type) => {
  const names = {
    'alipay': '支付宝',
    'wechat': '微信',
    'bank': '银行卡'
  }
  return names[type] || type
}

// 充值二维码相关
const showQrCode = ref(false);
const qrCodeUrl = ref('');
const qrcodeCanvas = ref(null);
const currentOrderNo = ref('');
let pollingTimer = null;

// 生成二维码
const generateQRCode = async () => {
  await nextTick()
  if (qrcodeCanvas.value && qrCodeUrl.value) {
    try {
      await QRCode.toCanvas(qrcodeCanvas.value, qrCodeUrl.value, {
        width: 256,
        margin: 2,
        color: {
          dark: '#000000',
          light: '#ffffff'
        }
      })
    } catch (error) {
      console.error('生成二维码失败：', error)
    }
  }
}

// 提交充值
const submitRecharge = async () => {
  if (!canRecharge.value) {
    showMessage('请检查充值信息', 'info')
    return
  }

  try {
    const res = await api.post('/recharge/create-order', {
      amount: parseFloat(rechargeAmount.value)
    })

    if (res.code === 0 && res.data) {
      currentOrderNo.value = res.data.order_no
      qrCodeUrl.value = res.data.qr_code
      showQrCode.value = true

      // 生成二维码
      await generateQRCode()

      // 开始轮询订单状态
      startPollingOrderStatus()
    } else {
      showMessage(res.message || '创建订单失败', 'error')
    }
  } catch (error) {
    console.error('创建充值订单失败:', error)
    showMessage(error.response?.data?.detail || '充值失败', 'error')
  }
}

// 开始轮询订单状态
const startPollingOrderStatus = () => {
  pollingTimer = setInterval(async () => {
    await checkOrderStatus()
  }, 3000)
}

// 检查订单状态
const checkOrderStatus = async () => {
  try {
    const res = await api.get(`/recharge/order-status/${currentOrderNo.value}`)
    
    if (res.code === 0 && res.data?.status === 'paid') {
      if (pollingTimer) {
        clearInterval(pollingTimer)
        pollingTimer = null
      }
      
      showQrCode.value = false
      showMessage('充值成功！', 'success')
      
      rechargeAmount.value = ''
      loadBalance()
    }
  } catch (error) {
    console.error('查询订单状态失败:', error)
  }
}

// 取消支付
const cancelRecharge = () => {
  showQrCode.value = false
  qrCodeUrl.value = ''
  currentOrderNo.value = ''
  if (pollingTimer) {
    clearInterval(pollingTimer)
    pollingTimer = null
  }
}

// 处理提现按钮点击
const handleWithdrawClick = () => {
  // 检查是否满足提现条件，不满足则给出具体提示
  if (accounts.value.length === 0) {
    showMessage('请先添加收款账户', 'warning')
    return
  }
  
  if (!selectedAccount.value) {
    showMessage('请选择收款账户', 'warning')
    return
  }
  
  const amount = parseFloat(withdrawAmount.value) || 0
  
  if (!withdrawAmount.value || amount === 0) {
    showMessage('请输入提现金额', 'warning')
    return
  }
  
  if (amount < 4) {
    showMessage('最低提现金额为4奶片', 'warning')
    return
  }
  
  if (amount > balance.value.available) {
    showMessage(`提现金额不能超过可用余额（${balance.value.available}奶片）`, 'warning')
    return
  }
  
  // 所有条件满足，执行提现
  submitWithdraw()
}

// 提交提现
const submitWithdraw = async () => {

  const confirmed = await showConfirm(
    `确定提现 ${withdrawAmount.value} 奶片吗？扣除手续费后实际到账 ¥${actualAmount.value}`,
    '提现确认'
  )

  if (!confirmed) return

  try {
    // 获取账户信息（selectedAccount 是 string 类型）
    const account = accounts.value.find(a => String(a.id) === selectedAccount.value)
    if (!account) {
      showMessage('请选择收款账户', 'error')
      return
    }

    const res = await createWithdrawal({
      amount: parseFloat(withdrawAmount.value),
      account_type: account.account_type,
      account_name: account.account_name,
      account_no: account.account_no
    })

    if (res.code === 0) {
      showMessage('提现申请已提交，预计1-3个工作日到账', 'success')
      withdrawAmount.value = ''
      selectedAccount.value = ''
      loadBalance()
    } else {
      showMessage(res.message || '提现失败', 'error')
    }
  } catch (error) {
    console.error('提现失败:', error)
    showMessage('提现失败', 'error')
  }
}

// 显示添加账户对话框
const showAddAccountDialog = () => {
  newAccount.value = {
    name: '',
    phone: '',
    account: '',
    bank_name: '',
    bank_branch: ''
  }
  showAddAccountModal.value = true
}

// 关闭添加账户对话框
const closeAddAccountModal = () => {
  showAddAccountModal.value = false
  newAccount.value = {
    name: '',
    phone: '',
    account: '',
    bank_name: '',
    bank_branch: ''
  }
}

// 提交添加账户
const submitAddAccount = async () => {
  // 验证手机号格式
  if (newAccount.value.phone && !/^1[3-9]\d{9}$/.test(newAccount.value.phone)) {
    showMessage('请输入正确的手机号码', 'error')
    return
  }
  
  try {
    const res = await createAccount({
      account_type: 'bank',
      account_name: newAccount.value.name,
      account_no: newAccount.value.account,
      bank_name: newAccount.value.bank_name,
      bank_branch: newAccount.value.bank_branch,
      is_default: accounts.value.length === 0
    })

    if (res.code === 0) {
      showMessage('账户添加成功', 'success')
      closeAddAccountModal()
      loadAccounts()
    } else {
      showMessage(res.message || '添加账户失败', 'error')
    }
  } catch (error) {
    console.error('添加账户失败:', error)
    showMessage(error.response?.data?.detail || '添加账户失败', 'error')
  }
}

// 设置默认账户
const setDefaultAccount = async (account) => {
  try {
    const res = await setDefault(account.id)
    if (res.code === 0) {
      showMessage('已设置为默认账户', 'success')
      loadAccounts()
    }
  } catch (error) {
    console.error('设置默认账户失败:', error)
    showMessage(error.response?.data?.detail || '设置失败', 'error')
  }
}

// 删除账户
const deleteAccount = async (account) => {
  const confirmed = await showConfirm('确定删除该收款账户吗？', '删除确认')

  if (!confirmed) return

  try {
    const res = await delAccount(account.id)
    if (res.code === 0) {
      showMessage('已删除收款账户', 'success')
      loadAccounts()
    }
  } catch (error) {
    console.error('删除账户失败:', error)
    showMessage(error.response?.data?.detail || '删除失败', 'error')
  }
}

// 掩码账号
const maskAccountNo = (accountNo) => {
  if (accountNo.length <= 8) return accountNo
  return accountNo.substring(0, 4) + '****' + accountNo.substring(accountNo.length - 4)
}

// 加载手续费配置
const loadFeeConfig = async () => {
  try {
    const response = await getFeeConfig()
    if (response.code === 0) {
      feeConfig.value = response.data
      // withdrawal_fee_rate 是 0-1 之间的小数，需要转换为百分比
      feeRate.value = (response.data.withdrawal_fee_rate * 100).toFixed(0)
    }
  } catch (error) {
    console.error('获取手续费配置失败:', error)
    // 如果获取失败，使用默认值0%
    feeRate.value = 0
  }
}

// 处理提现金额输入（限制最大值）
const handleWithdrawInput = () => {
  const amount = parseFloat(withdrawAmount.value) || 0
  if (amount > balance.value.available) {
    withdrawAmount.value = balance.value.available
  }
  if (amount < 0) {
    withdrawAmount.value = 0
  }
}

onMounted(() => {
  loadBalance()
  loadAccounts()
  loadFeeConfig() // 加载手续费配置

  // 如果URL有 #recharge 锚点，自动滚动到充值区域
  if (window.location.hash === '#recharge') {
    setTimeout(() => {
      const rechargeElement = document.getElementById('recharge')
      if (rechargeElement) {
        rechargeElement.scrollIntoView({ behavior: 'smooth', block: 'start' })
        // 添加高亮动画
        rechargeElement.classList.add('ring-2', 'ring-primary-500', 'ring-offset-2')
        setTimeout(() => {
          rechargeElement.classList.remove('ring-2', 'ring-primary-500', 'ring-offset-2')
        }, 2000)
      }
    }, 500)
  }
})
</script>