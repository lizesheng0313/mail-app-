/**
 * 代理管理API
 */

import api from '@/services/api'

export const proxyApi = {
  // 获取代理池列表
  getProxyPools() {
    return api.get('/proxy/pools')
  },

  // 请求代理
  requestProxy(data) {
    return api.post('/proxy/request', data)
  },

  // 释放代理
  releaseProxy(proxyId) {
    return api.post(`/proxy/release/${proxyId}`)
  },

  // 记录代理使用情况
  recordProxyUsage(data) {
    return api.post('/proxy/usage', data)
  },

  // 获取代理使用统计
  getProxyStats() {
    return api.get('/proxy/stats')
  },

  // 获取用户代理策略
  getUserProxyStrategy() {
    return api.get('/proxy/strategy')
  },

  // 更新用户代理策略
  updateUserProxyStrategy(data) {
    return api.post('/proxy/strategy', data)
  }
}

// 付费代理管理API
export const paidProxyApi = {
  // 获取代理列表
  getProxyList(params = {}) {
    return api.get('/paid-proxy/list', { params })
  },

  // 获取活跃代理
  getActiveProxy() {
    return api.get('/paid-proxy/active')
  },

  // 创建代理
  createProxy(data) {
    return api.post('/paid-proxy/create', data)
  },

  // 从API创建代理
  createProxyFromApi(data) {
    return api.post('/paid-proxy/create-from-api', data)
  },

  // 更新代理
  updateProxy(proxyId, data) {
    return api.put(`/paid-proxy/${proxyId}`, data)
  },

  // 删除代理
  deleteProxy(proxyId) {
    return api.delete(`/paid-proxy/${proxyId}`)
  },

  // 设置活跃代理
  setActiveProxy(proxyId) {
    return api.put(`/paid-proxy/${proxyId}/active`)
  },

  // 记录代理使用
  recordUsage(proxyId, data) {
    return api.post(`/paid-proxy/${proxyId}/usage`, data)
  },

  // 获取使用统计
  getUsageStats(proxyId = null) {
    const params = proxyId ? { proxy_id: proxyId } : {}
    return api.get('/paid-proxy/stats', { params })
  }
}
