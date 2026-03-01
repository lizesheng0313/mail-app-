/**
 * 授权码API接口
 */

import api from '@/services/api'

export const authCodeAPI = {
  // 使用邮箱授权码
  useAuthCode: (code) => api.post('/auth-codes/use', { code }),

  // 创建授权码（管理员）
  createAuthCodes: (code_type, count) =>
    api.post('/auth-codes/create', { code_type, count }),

  // 获取卡密列表（管理员）
  getAuthCodes: (page = 1, limit = 20, filters = {}) => {
    const params = { page, limit }
    
    // 添加筛选参数
    if (filters.code_type) {
      params.code_type = filters.code_type
    }
    if (filters.status) {
      params.status = filters.status
    }
    if (filters.keyword) {
      params.keyword = filters.keyword
    }
    
    return api.get('/auth-codes/list', { params })
  },

  // 删除卡密（管理员）
  deleteAuthCode: (id) => api.delete(`/auth-codes/${id}`),

  // 获取授权码统计（管理员）
  getStats: () => api.get('/auth-codes/stats'),

  // 获取授权码类型
  getTypes: () => api.get('/auth-codes/types')
}

export default authCodeAPI