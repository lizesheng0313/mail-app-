import { onUnmounted, ref } from 'vue'

import { batchLoginAPI } from '@/api/batchLogin'
import { getServerUrl, isTauri } from '@/services/api'
import { isAutoRecoveryLimitError, normalizeOAuthRecoveryErrorMessage } from '@/utils/oauthRecovery'

const KEEP_ALIVE_INTERVAL_MS = 5 * 60 * 1000
const KEEP_ALIVE_PAGE_SIZE = 100
const WAKE_REFRESH_MIN_GAP_MS = 30 * 1000

type OAuthMailboxAccount = {
  id: number
  email?: string
  password?: string
  auth_type?: string
  oauth_provider?: string
  status?: string
  error_message?: string
}

const AUTH_RECOVERY_KEYWORDS = ['auth', 'token', 'oauth', '授权', '失效', '重新授权', 'login', 'refresh_token']

const getErrorMessage = (error: any) =>
  error?.response?.data?.message || error?.message || String(error || '')

const shouldAttemptSilentRecovery = (message: string) => {
  const normalized = String(message || '').toLowerCase()
  return AUTH_RECOVERY_KEYWORDS.some((keyword) => normalized.includes(keyword))
}

const getTauriInvoke = async () => {
  if (!isTauri()) return null
  const { invoke } = await import('@tauri-apps/api/core')
  return invoke
}

export function useDesktopOAuthKeepAlive() {
  const isRunning = ref(false)
  const isRefreshing = ref(false)
  let timer: number | null = null
  let lastWakeRefreshAt = 0

  const scheduleNext = (delay = KEEP_ALIVE_INTERVAL_MS) => {
    if (!isRunning.value) return
    if (timer) {
      clearTimeout(timer)
    }
    timer = window.setTimeout(() => {
      void refreshAllOAuthTokens()
    }, delay)
  }

  const loadOAuthAccounts = async (): Promise<OAuthMailboxAccount[]> => {
    const response: any = await batchLoginAPI.getAllAccounts(KEEP_ALIVE_PAGE_SIZE, {
      suppressErrorMessage: true
    })
    if (response.code !== 0) {
      throw new Error(response.message || '获取外部邮箱列表失败')
    }

    const data = response.data || {}
    const list = Array.isArray(data) ? data : data.accounts || []
    return list.filter(
      (account: OAuthMailboxAccount) => String(account?.auth_type || '').toLowerCase() === 'oauth2'
    )
  }

  const refreshAllOAuthTokens = async () => {
    if (!isTauri() || !isRunning.value || isRefreshing.value) {
      return
    }

    const authToken = localStorage.getItem('token')
    if (!authToken) {
      scheduleNext()
      return
    }

    isRefreshing.value = true
    try {
      const oauthAccounts = await loadOAuthAccounts()
      const tauriInvoke = await getTauriInvoke()
      for (const account of oauthAccounts) {
        try {
          const response: any = await batchLoginAPI.getOAuth2AccessToken(account.id, {
            suppressErrorMessage: true
          })
          if (response.code !== 0) {
            const recovered = await trySilentRecover(account, authToken, response.message, tauriInvoke)
            if (recovered) {
              continue
            }
            console.warn('[DesktopOAuthKeepAlive] token keep-alive failed:', {
              mailboxId: account.id,
              email: account.email,
              provider: account.oauth_provider,
              message: response.message
            })
          } else if (
            account.status === 'failed' &&
            AUTH_RECOVERY_KEYWORDS.some((keyword) =>
              String(account.error_message || '').toLowerCase().includes(keyword)
            )
          ) {
            await batchLoginAPI.updateMailboxStatus(account.id, 'active')
          }
        } catch (error: any) {
          const errorMessage = getErrorMessage(error)
          const recovered = await trySilentRecover(account, authToken, errorMessage, tauriInvoke)
          if (recovered) {
            continue
          }
          console.warn('[DesktopOAuthKeepAlive] token keep-alive request failed:', {
            mailboxId: account.id,
            email: account.email,
            provider: account.oauth_provider,
            message: errorMessage
          })
        }
      }
    } catch (error: any) {
      console.warn(
        '[DesktopOAuthKeepAlive] load oauth accounts failed:',
        error?.response?.data?.message || error?.message || String(error || '')
      )
    } finally {
      isRefreshing.value = false
      scheduleNext()
    }
  }

  const trySilentRecover = async (
    account: OAuthMailboxAccount,
    authToken: string,
    rawMessage: string,
    tauriInvoke: any
  ) => {
    if (!tauriInvoke || !shouldAttemptSilentRecovery(rawMessage) || !String(account.password || '').trim()) {
      return false
    }

    try {
      await tauriInvoke('recover_external_mailbox_session', {
        mailboxId: account.id,
        token: authToken,
        serverUrl: getServerUrl()
      })
      window.dispatchEvent(
        new CustomEvent('external-mailbox-recovered', {
          detail: { mailboxId: account.id }
        })
      )
      return true
    } catch (recoveryError: any) {
      const rawMessage = getErrorMessage(recoveryError)
      const message = normalizeOAuthRecoveryErrorMessage(rawMessage)
      if (isAutoRecoveryLimitError(rawMessage)) {
        try {
          await batchLoginAPI.updateMailboxStatus(account.id, 'failed', message)
          window.dispatchEvent(
            new CustomEvent('external-mailbox-updated', {
              detail: { mailboxId: account.id }
            })
          )
        } catch (statusError) {
          console.warn('[DesktopOAuthKeepAlive] update failed status error:', statusError)
        }
      }
      console.warn('[DesktopOAuthKeepAlive] silent recovery failed:', {
        mailboxId: account.id,
        email: account.email,
        provider: account.oauth_provider,
        message: rawMessage
      })
      return false
    }
  }

  const triggerWakeRefresh = async () => {
    if (!isRunning.value || !isTauri()) return

    const now = Date.now()
    if (now - lastWakeRefreshAt < WAKE_REFRESH_MIN_GAP_MS) {
      return
    }
    lastWakeRefreshAt = now
    await refreshAllOAuthTokens()
  }

  const handleVisibilityChange = () => {
    if (document.visibilityState === 'visible') {
      void triggerWakeRefresh()
    }
  }

  const handleWindowFocus = () => {
    void triggerWakeRefresh()
  }

  const handleWindowOnline = () => {
    void triggerWakeRefresh()
  }

  const start = async (runImmediately = true) => {
    if (!isTauri() || isRunning.value) {
      return
    }

    isRunning.value = true
    window.addEventListener('focus', handleWindowFocus)
    window.addEventListener('online', handleWindowOnline)
    document.addEventListener('visibilitychange', handleVisibilityChange)

    if (runImmediately) {
      await refreshAllOAuthTokens()
      return
    }
    scheduleNext()
  }

  const stop = () => {
    if (timer) {
      clearTimeout(timer)
      timer = null
    }
    isRunning.value = false
    isRefreshing.value = false
    window.removeEventListener('focus', handleWindowFocus)
    window.removeEventListener('online', handleWindowOnline)
    document.removeEventListener('visibilitychange', handleVisibilityChange)
  }

  onUnmounted(() => {
    stop()
  })

  return {
    isRunning,
    isRefreshing,
    start,
    stop,
    refreshNow: refreshAllOAuthTokens
  }
}
