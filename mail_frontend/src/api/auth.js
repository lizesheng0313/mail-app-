/**
 * 认证相关API接口
 */

import api from '@/services/api'

export const authAPI = {
  // 发送邮箱验证码
  sendCode: (email, purpose = 'register') => api.post('/auth/send-code', { email, purpose }),

  // 用户注册
  register: (email, password, verification_code) =>
    api.post('/auth/register', { email, password, verification_code }),

  // 用户登录
  login: (email, password) =>
    api.post('/auth/login', { email, password }),

  // 重置密码
  resetPassword: (email, verification_code, new_password) =>
    api.post('/auth/reset-password', { email, verification_code, new_password }),

  // 获取当前用户
  getCurrentUser: () => api.get('/auth/me'),

  // 登出
  logout: () => api.post('/auth/logout'),

  // 获取允许的邮箱域名
  getAllowedDomains: () => api.get('/auth/allowed-domains')
}

// 导出个别函数供测试使用
export const { login, register, getCurrentUser, logout, sendCode, resetPassword, getAllowedDomains } = authAPI

export default authAPI