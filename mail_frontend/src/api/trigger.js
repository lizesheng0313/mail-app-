/**
 * 触发器API接口
 */

import api from '@/services/api'

export const triggerApi = {
  // 获取触发器列表
  getTriggers(params = {}) {
    return api.get('/triggers/', { params })
  },

  // 获取触发器详情
  getTrigger(triggerId) {
    return api.get(`/triggers/${triggerId}`)
  },

  // 创建触发器
  createTrigger(data) {
    return api.post('/triggers/', data)
  },

  // 更新触发器
  updateTrigger(triggerId, data) {
    return api.put(`/triggers/${triggerId}`, data)
  },

  // 删除触发器
  deleteTrigger(triggerId) {
    return api.delete(`/triggers/${triggerId}`)
  },

  // 手动触发
  triggerManual(triggerId) {
    return api.post(`/triggers/${triggerId}/trigger`)
  },

  // 获取触发器执行日志
  getTriggerLogs(params = {}) {
    return api.get('/triggers/logs', { params })
  }
}
