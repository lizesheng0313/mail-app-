/**
 * 用户API接口
 */

import api from '@/services/api'

export const userAPI = {
  // 获取用户信息
  getProfile: (params = {}) => api.get('/users/me', { params }),

  // 更新用户信息
  updateProfile: (data) => api.patch('/users/me', data),

  // 获取用户统计
  getStats: () => api.get('/users/stats')
}

export default userAPI
