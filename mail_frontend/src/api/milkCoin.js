/**
 * 奶片系统 API
 */
import api from '@/services/api'

/**
 * 获取手续费配置（无需登录）
 */
export function getFeeConfig() {
  return api.get('/milk-coins/fee-config')
}

/**
 * 获取用户奶片余额 (从token自动获取user_id)
 */
export function getBalance() {
  return api.get('/milk-coins/balance')
}

/**
 * 创建充值订单 (从token自动获取user_id)
 */
export function createRecharge(data) {
  return api.post('/milk-coins/recharge', {
    amount: data.amount,
    payment_method: data.payment_method
  })
}

/**
 * 充值回调（模拟支付成功）
 */
export function rechargeCallback(orderNo, transactionId) {
  return api.post('/milk-coins/recharge/callback', null, {
    params: {
      order_no: orderNo,
      transaction_id: transactionId
    }
  })
}

/**
 * 获取交易记录 (从token自动获取user_id)
 */
export function getTransactions(params) {
  return api.get('/milk-coins/transactions', { params })
}

/**
 * 获取充值记录 (从token自动获取user_id)
 */
export function getRechargeRecords(params) {
  return api.get('/milk-coins/recharge-records', { params })
}

/**
 * 申请提现 (从token自动获取user_id)
 */
export function createWithdrawal(data) {
  return api.post('/milk-coins/withdraw', {
    amount: data.amount,
    account_type: data.account_type,
    account_name: data.account_name,
    account_no: data.account_no
  })
}

/**
 * 获取奶片统计 (从token自动获取user_id)
 */
export function getMilkCoinStats() {
  return api.get('/milk-coins/stats')
}

// ========== 管理员接口 ==========

/**
 * 管理员获取提现申请列表
 */
export function adminGetWithdrawals(params) {
  return api.get('/milk-coins/admin/withdrawals', { params })
}

/**
 * 管理员批准提现 (从token自动获取admin_id)
 */
export function adminApproveWithdrawal(withdrawalId) {
  return api.post(`/milk-coins/admin/withdrawals/${withdrawalId}/approve`)
}

/**
 * 管理员拒绝提现 (从token自动获取admin_id)
 */
export function adminRejectWithdrawal(withdrawalId, reason) {
  return api.post(`/milk-coins/admin/withdrawals/${withdrawalId}/reject`, {
    reason
  })
}

/**
 * 管理员确认提现完成
 */
export function adminCompleteWithdrawal(withdrawalId) {
  return api.post(`/milk-coins/admin/withdrawals/${withdrawalId}/complete`)
}
