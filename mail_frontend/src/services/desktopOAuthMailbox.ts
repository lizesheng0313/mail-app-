import { batchLoginAPI } from '@/api/batchLogin'

type OAuthAccessTokenPayload = {
  access_token: string
  email: string
  imap_host: string
  imap_port: number
  provider?: string
}

type OAuthActionOptions = {
  suppressErrorMessage?: boolean
}

const OAUTH_RETRY_KEYWORDS = [
  'oauth2',
  'xoauth2',
  'auth',
  'token',
  'credential',
  'login',
  '重新授权',
  '授权',
  '选择收件箱失败',
  'bad response: command',
  'command error',
  'not authenticated',
  'invalid credentials'
]

const getRequestConfig = (suppressErrorMessage = true) => ({
  suppressErrorMessage
})

const getErrorMessage = (error: any) =>
  error?.response?.data?.message || error?.message || String(error || '')

const shouldSkipTokenForceRefresh = (error: any) => {
  const message = getErrorMessage(error).toLowerCase()
  return message.includes('refresh_token 不存在') || message.includes('等待桌面端自动恢复')
}

export const shouldRetryOAuthMailboxAction = (error: any) => {
  const message = getErrorMessage(error).toLowerCase()
  return OAUTH_RETRY_KEYWORDS.some((keyword) => message.includes(keyword))
}

export const getDesktopOAuthAccessToken = async (
  mailboxId: number,
  suppressErrorMessage = true
): Promise<OAuthAccessTokenPayload> => {
  const response: any = await batchLoginAPI.getOAuth2AccessToken(
    mailboxId,
    getRequestConfig(suppressErrorMessage)
  )
  if (response.code !== 0 || !response.data?.access_token) {
    throw new Error(response.message || '获取 token 失败')
  }
  return response.data
}

export const forceRefreshDesktopOAuthAccessToken = async (
  mailboxId: number,
  suppressErrorMessage = true
): Promise<OAuthAccessTokenPayload> => {
  const refreshResponse: any = await batchLoginAPI.refreshOAuth2Token(
    mailboxId,
    getRequestConfig(suppressErrorMessage)
  )
  if (refreshResponse.code !== 0) {
    throw new Error(refreshResponse.message || '刷新 token 失败')
  }
  return await getDesktopOAuthAccessToken(mailboxId, suppressErrorMessage)
}

export const runDesktopOAuthMailboxAction = async <T>(
  mailboxId: number,
  executor: (tokenPayload: OAuthAccessTokenPayload) => Promise<T>,
  options: OAuthActionOptions = {}
): Promise<T> => {
  const suppressErrorMessage = options.suppressErrorMessage ?? true

  try {
    const tokenPayload = await getDesktopOAuthAccessToken(mailboxId, suppressErrorMessage)
    return await executor(tokenPayload)
  } catch (error: any) {
    if (!shouldRetryOAuthMailboxAction(error)) {
      throw error
    }

    if (shouldSkipTokenForceRefresh(error)) {
      throw error
    }

    const tokenPayload = await forceRefreshDesktopOAuthAccessToken(mailboxId, suppressErrorMessage)
    return await executor(tokenPayload)
  }
}
