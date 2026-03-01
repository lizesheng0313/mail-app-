import api from '@/services/api'

/**
 * 用户管理API - 管理员专用
 */
export const adminUsersApi = {
  /**
   * 获取用户列表
   * @param {Object} params - 查询参数
   * @param {number} params.page - 页码
   * @param {number} params.limit - 每页数量
   * @param {string} params.search - 搜索关键词
   * @param {boolean} params.proxy_enabled - 代理权限筛选
   */
  getUserList(params = {}) {
    return api.get('/admin/users/list', { params })
  },

  /**
   * 更新用户代理权限
   * @param {number} userId - 用户ID
   * @param {Object} data - 更新数据
   * @param {boolean} data.proxy_enabled - 是否启用代理
   * @param {number} data.proxy_quota - 代理配额
   */
  updateUserProxyPermission(userId, data) {
    return api.put(`/admin/users/${userId}/proxy`, data)
  },

  /**
   * 获取用户代理使用情况
   * @param {number} userId - 用户ID
   */
  getUserProxyUsage(userId) {
    return api.get(`/admin/users/${userId}/proxy-usage`)
  },

  /**
   * 重置用户代理配额
   * @param {number} userId - 用户ID
   */
  resetUserProxyQuota(userId) {
    return api.post(`/admin/users/${userId}/reset-proxy-quota`)
  }
}

export default adminUsersApi