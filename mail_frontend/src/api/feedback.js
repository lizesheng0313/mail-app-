/**
 * 反馈API接口
 */

import api from '@/services/api'

export const feedbackAPI = {
  // 创建反馈
  create: (data) => api.post('/feedback/create', data),

  // 获取用户自己的反馈列表
  getMyFeedbacks: (page = 1, page_size = 20) =>
    api.get('/feedback/my', { params: { page, page_size } })
}

export default feedbackAPI