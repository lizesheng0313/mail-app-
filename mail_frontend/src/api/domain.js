/**
 * 域名管理API接口
 */

import api from '@/services/api'

export const domainAPI = {
  // 获取域名列表（管理员）
  getDomains: (page = 1, limit = 20, search = '') =>
    api.get('/domains/list', { params: { page, limit, search } }),

  // 创建域名（管理员）
  createDomain: (data) =>
    api.post('/domains/create', data),

  // 更新域名（管理员）
  updateDomain: (id, data) =>
    api.put(`/domains/${id}`, data),

  // 删除域名（管理员）
  deleteDomain: (id) => api.delete(`/domains/${id}`),

  // 获取域名统计（管理员）
  getStats: () => api.get('/domains/stats')
}

export default domainAPI