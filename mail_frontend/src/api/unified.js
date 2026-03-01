/**
 * 统一的邮件和邮箱API
 * 前端统一调用，后端根据type参数自动路由到不同的表
 */

import api from '@/services/api'

export const unifiedAPI = {
  // ==================== 邮件操作 ====================
  
  /**
   * 删除邮件
   * @param {number} emailId - 邮件ID
   * @param {string} type - 'system' 系统邮箱 | 'external' 外部邮箱
   */
  deleteEmail: (emailId, type = 'system') => {
    return api.delete(`/unified-emails/emails/${emailId}`, { params: { type } })
  },

  /**
   * 批量删除邮件
   * @param {number[]} emailIds - 邮件ID列表
   * @param {string} type - 'system' | 'external'
   */
  batchDeleteEmails: (emailIds, type = 'system') => {
    // 统一的批量删除接口
    return api.post('/unified-emails/emails/batch-delete', { 
      email_ids: emailIds,
      type: type
    })
  },

  /**
   * 标记邮件已读
   * @param {number} emailId - 邮件ID
   * @param {string} type - 'system' | 'external'
   */
  markEmailRead: (emailId, type = 'system') => {
    return api.put(`/unified-emails/emails/${emailId}/read`, null, { params: { type } })
  },

  // ==================== 邮箱操作 ====================

  /**
   * 删除邮箱
   * @param {number} mailboxId - 邮箱ID
   * @param {string} type - 'system' | 'external'
   */
  deleteMailbox: (mailboxId, type = 'system') => {
    return api.delete(`/unified-emails/mailboxes/${mailboxId}`, { params: { type } })
  },

  /**
   * 批量删除邮箱
   * @param {number[]} mailboxIds - 邮箱ID列表
   * @param {string} type - 'system' | 'external'
   */
  batchDeleteMailboxes: (mailboxIds, type = 'system') => {
    // 统一的批量删除接口
    return api.post('/unified-emails/mailboxes/batch-delete', {
      mailbox_ids: mailboxIds,
      type: type
    })
  }
}

export default unifiedAPI
