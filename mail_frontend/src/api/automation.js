import api from '@/services/api'

// 自动化工作流 API
export const automationApi = {
  // 创建工作流
  createWorkflow: (data) => {
    return api.post('/automation/workflows', data)
  },

  // 获取工作流列表
  getWorkflows: (params = {}) => {
    return api.get('/automation/workflows', { params })
  },

  // 获取工作流详情
  getWorkflow: (id) => {
    return api.get(`/automation/workflows/${id}`)
  },

  // 更新工作流
  updateWorkflow: (id, data) => {
    return api.put(`/automation/workflows/${id}`, data)
  },

  // 删除工作流
  deleteWorkflow: (id) => {
    return api.delete(`/automation/workflows/${id}`)
  },

  // 执行工作流
  executeWorkflow: (id) => {
    return api.post(`/automation/workflows/${id}/execute`)
  },

  // 获取执行状态
  getExecutionStatus: (executionId) => {
    return api.get(`/automation/executions/${executionId}`)
  },

  // 获取执行历史
  getExecutionHistory: (params = {}) => {
    return api.get('/automation/executions', { params })
  }
}