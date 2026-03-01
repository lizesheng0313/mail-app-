/**
 * SMTP 发件账号 API
 */

import api from '@/services/api'

export const smtpAccountsAPI = {
    // 获取 SMTP 发件账号列表
    getAccounts: () =>
        api.get('/smtp-accounts'),

    // 添加 SMTP 发件账号
    addAccount: (accountData) =>
        api.post('/smtp-accounts', accountData),

    // 删除 SMTP 发件账号
    deleteAccount: (accountId) =>
        api.delete(`/smtp-accounts/${accountId}`),

    // 发送邮件
    sendEmail: (accountId, emailData) =>
        api.post(`/smtp-accounts/${accountId}/send`, emailData)
}

export default smtpAccountsAPI
