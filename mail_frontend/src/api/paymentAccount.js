/**
 * 收款账户 API (从token自动获取user_id)
 */
import api from '@/services/api'

/**
 * 获取用户收款账户列表 (从token自动获取user_id)
 */
export function getUserAccounts() {
  return api.get('/api/payment-accounts/')
}

/**
 * 获取账户详情 (从token自动获取user_id)
 */
export function getAccountDetail(accountId) {
  return api.get(`/api/payment-accounts/${accountId}`)
}

/**
 * 创建收款账户 (从token自动获取user_id)
 */
export function createAccount(data) {
  return api.post('/api/payment-accounts/', {
    account_type: data.account_type,
    account_name: data.account_name,
    account_no: data.account_no,
    bank_name: data.bank_name,
    bank_branch: data.bank_branch,
    is_default: data.is_default
  })
}

/**
 * 更新收款账户 (从token自动获取user_id)
 */
export function updateAccount(accountId, data) {
  return api.put(`/api/payment-accounts/${accountId}`, {
    account_name: data.account_name,
    account_no: data.account_no,
    bank_name: data.bank_name,
    bank_branch: data.bank_branch,
    is_default: data.is_default
  })
}

/**
 * 设置为默认账户 (从token自动获取user_id)
 */
export function setDefaultAccount(accountId) {
  return api.post(`/api/payment-accounts/${accountId}/set-default`)
}

/**
 * 删除收款账户 (从token自动获取user_id)
 */
export function deleteAccount(accountId) {
  return api.delete(`/api/payment-accounts/${accountId}`)
}

/**
 * 获取默认收款账户 (从token自动获取user_id)
 */
export function getDefaultAccount() {
  return api.get('/api/payment-accounts/default/get')
}
