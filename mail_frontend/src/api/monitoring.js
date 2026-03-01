/**
 * 监控API接口
 */

import api from '@/services/api'

export const monitoringAPI = {
  // 获取监控概览
  getOverview: () => api.get('/monitoring/overview'),

  // 获取用户统计
  getUserStats: (days = 7) => api.get('/monitoring/user-stats', { params: { days } }),

  // 获取邮件活动统计
  getEmailActivity: (days = 7) => api.get('/monitoring/email-activity', { params: { days } }),

  // 获取系统健康状态
  getSystemHealth: () => api.get('/monitoring/system-health'),

  // 记录页面访问
  recordPageView: (page, userAgent) => 
    api.post('/monitoring/page-view', { page, user_agent: userAgent }),

  // 获取页面访问统计
  getPageAnalytics: (days = 7) => api.get('/monitoring/page-analytics', { params: { days } }),

  // 获取地理分布统计
  getGeoDistribution: (mode = 'global', days = 30) => 
    api.get('/monitoring/geo-distribution', { params: { mode, days } })
}

export default monitoringAPI