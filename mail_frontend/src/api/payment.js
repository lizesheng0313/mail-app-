/**
 * 支付相关 API
 */
import api from '@/services/api'

/**
 * 获取所有套餐列表
 */
export function getPackages() {
  return api.get('/api/payment/packages')
}

/**
 * 创建订单（支付宝）
 */
export function createOrder(data) {
  return api.post('/api/payment/create-order', data)
}

/**
 * 使用奶片购买邮箱套餐
 */
export function createMilkCoinOrder(data) {
  return api.post('/api/payment/create-milk-coin-order', data)
}

/**
 * 查询订单状态
 */
export function queryOrder(data) {
  return api.post('/api/payment/query-order', data)
}

/**
 * 获取我的订单列表
 */
export function getMyOrders(params) {
  return api.get('/api/payment/my-orders', { params })
}
