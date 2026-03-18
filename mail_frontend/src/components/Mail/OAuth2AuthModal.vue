<template>
  <Teleport to="body">
    <div v-if="visible" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-5">
      <div class="bg-white rounded-lg shadow-2xl w-full max-w-lg">
        <!-- 标题栏 -->
        <div class="px-5 py-3 border-b border-gray-200 flex items-center justify-between">
          <h3 class="text-lg font-semibold text-gray-800">OAuth2 授权登录</h3>
          <button @click="handleClose" :disabled="isAuthorizing" class="text-gray-400 hover:text-gray-600 transition disabled:opacity-50">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
        </div>

        <div class="p-5">
          <!-- 说明 -->
          <div class="mb-4 p-3 bg-amber-50 border border-amber-200 rounded-lg text-sm text-amber-700">
            <p>以下邮箱需要 OAuth2 授权登录。点击"开始授权"后，将逐个打开授权页面，请在弹出的页面中完成登录授权。</p>
          </div>

          <!-- 邮箱列表 -->
          <div class="border border-gray-200 rounded-lg overflow-hidden max-h-64 overflow-y-auto">
            <div v-for="(item, idx) in accounts" :key="item.email" 
                 class="flex items-center gap-3 px-4 py-2 border-b border-gray-100 last:border-b-0"
                 :class="getRowClass(item.status)">
              <!-- 序号 -->
              <span class="text-gray-400 text-sm w-6">{{ idx + 1 }}.</span>
              
              <!-- 状态图标 -->
              <span class="w-5 flex-shrink-0">
                <svg v-if="item.status === 'success'" class="w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                </svg>
                <svg v-else-if="item.status === 'error'" class="w-5 h-5 text-red-500" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/>
                </svg>
                <svg v-else-if="item.status === 'authorizing'" class="w-5 h-5 text-blue-500 animate-spin" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                <span v-else class="w-5 h-5 rounded-full border-2 border-gray-300 block"></span>
              </span>
              
              <!-- 邮箱地址 -->
              <span class="flex-1 font-mono text-sm truncate" :class="item.status === 'error' ? 'text-red-600' : 'text-gray-700'">
                {{ item.email }}
              </span>
              
              <!-- 平台标签 -->
              <span class="text-xs px-2 py-0.5 rounded" :class="item.provider === 'google' ? 'bg-red-100 text-red-600' : 'bg-blue-100 text-blue-600'">
                {{ item.provider === 'google' ? 'Gmail' : 'Outlook' }}
              </span>
              
              <!-- 状态文字 -->
              <span class="text-xs text-gray-500 w-20 text-right">
                {{ getStatusText(item.status) }}
              </span>
            </div>
          </div>

          <!-- 当前授权进度 -->
          <div v-if="isAuthorizing" class="mt-4 p-3 bg-blue-50 border border-blue-200 rounded-lg">
            <div class="flex items-center gap-2 text-blue-700 text-sm">
              <svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              <span>正在授权 {{ currentIndex + 1 }}/{{ accounts.length }}: {{ currentEmail }}</span>
            </div>
            <p class="text-xs text-blue-600 mt-1">请在弹出的窗口中完成授权，授权完成后窗口会自动关闭</p>
          </div>

          <!-- 错误提示 -->
          <div v-if="errorMessage" class="mt-4 p-3 bg-red-50 border border-red-200 rounded-lg text-sm text-red-700">
            {{ errorMessage }}
          </div>
        </div>

        <!-- 底部按钮 -->
        <div class="px-5 py-3 border-t border-gray-200 flex justify-between items-center bg-gray-50">
          <div class="text-sm text-gray-500">
            共 {{ accounts.length }} 个邮箱，成功 {{ successCount }} 个，失败 {{ failCount }} 个
          </div>
          <div class="flex gap-3">
            <button
              @click="handleClose"
              :disabled="isAuthorizing"
              class="px-4 py-2 text-sm text-gray-700 bg-white border border-gray-300 hover:bg-gray-50 rounded-lg transition disabled:opacity-50"
            >
              {{ isFinished ? '关闭' : '取消' }}
            </button>
            <button
              v-if="!isFinished"
              @click="startAuthorization"
              :disabled="isAuthorizing || accounts.length === 0"
              class="px-4 py-2 text-sm text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition disabled:opacity-50 inline-flex items-center gap-2"
            >
              <svg v-if="isAuthorizing" class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              {{ isAuthorizing ? '授权中...' : '开始授权' }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import batchLoginAPI from '@/api/batchLogin'
import { isTauri } from '@/services/api'

// 动态获取 Tauri shell API
async function openExternal(url: string): Promise<boolean> {
  if (isTauri()) {
    try {
      const { open } = await import('@tauri-apps/plugin-shell')
      await open(url)
      return true
    } catch (e) {
      console.error('[OAuth2] Tauri shell.open 失败:', e)
      // 降级尝试 window.open
      const win = window.open(url, '_blank')
      return !!win
    }
  } else {
    const win = window.open(url, '_blank', 'width=600,height=700,scrollbars=yes')
    return !!win
  }
}

interface OAuthAccount {
  email: string
  provider: 'google' | 'microsoft'
  status: 'pending' | 'authorizing' | 'success' | 'error'
}

const props = defineProps<{
  visible: boolean
  pendingAccounts: Array<{ email: string, provider: string }>
}>()

const emit = defineEmits(['close', 'complete'])

const accounts = ref<OAuthAccount[]>([])
const isAuthorizing = ref(false)
const currentIndex = ref(-1)

const currentEmail = computed(() => {
  if (currentIndex.value >= 0 && currentIndex.value < accounts.value.length) {
    return accounts.value[currentIndex.value].email
  }
  return ''
})

const successCount = computed(() => accounts.value.filter(a => a.status === 'success').length)
const failCount = computed(() => accounts.value.filter(a => a.status === 'error').length)
const isFinished = computed(() => {
  return accounts.value.length > 0 && accounts.value.every(a => a.status === 'success' || a.status === 'error')
})

watch(() => props.visible, (visible) => {
  if (visible) {
    // 初始化账号列表
    accounts.value = props.pendingAccounts.map(a => ({
      email: a.email,
      provider: a.provider as 'google' | 'microsoft',
      status: 'pending' as const
    }))
    currentIndex.value = -1
    isAuthorizing.value = false
  }
})

const getRowClass = (status: string) => {
  switch (status) {
    case 'success': return 'bg-green-50'
    case 'error': return 'bg-red-50'
    case 'authorizing': return 'bg-blue-50'
    default: return ''
  }
}

const getStatusText = (status: string) => {
  switch (status) {
    case 'success': return '授权成功'
    case 'error': return '授权失败'
    case 'authorizing': return '授权中...'
    default: return '待授权'
  }
}

const errorMessage = ref('')

const waitForMailboxAuthorized = async (email: string, maxWait = 10000, interval = 1000) => {
  const startTime = Date.now()

  while (Date.now() - startTime < maxWait) {
    const accountsRes = await batchLoginAPI.getAccounts(1, 100)
    const existingAccount = accountsRes.data?.accounts?.find((a: any) =>
      a.email === email && a.auth_type === 'oauth2'
    )

    if (existingAccount) {
      return true
    }

    await new Promise(resolve => setTimeout(resolve, interval))
  }

  return false
}

const waitForDesktopOAuthCallback = (expectedEmail: string, timeoutMs: number) =>
  new Promise<{ success: boolean, error?: string } | null>((resolve) => {
    const handler = (event: Event) => {
      const detail = (event as CustomEvent).detail || {}

      if (detail.oauth2_success === '1') {
        if (detail.email && detail.email !== expectedEmail) {
          cleanup()
          resolve({ success: false, error: `授权返回的邮箱与当前账号不一致: ${detail.email}` })
          return
        }

        cleanup()
        resolve({ success: true })
        return
      }

      if (detail.oauth2_error) {
        cleanup()
        resolve({ success: false, error: detail.oauth2_error })
      }
    }

    const cleanup = () => {
      window.clearTimeout(timer)
      window.removeEventListener('oauth2-callback', handler as EventListener)
    }

    const timer = window.setTimeout(() => {
      cleanup()
      resolve(null)
    }, timeoutMs)

    window.addEventListener('oauth2-callback', handler as EventListener)
  })

const startAuthorization = async () => {
  isAuthorizing.value = true
  errorMessage.value = ''
  
  for (let i = 0; i < accounts.value.length; i++) {
    const account = accounts.value[i]
    
    // 跳过已完成的
    if (account.status === 'success' || account.status === 'error') {
      continue
    }
    
    currentIndex.value = i
    account.status = 'authorizing'
    
    try {
      // 获取授权 URL（桌面端传 isDesktop=true）
      const isDesktop = isTauri()
      console.log('[OAuth2] 获取授权 URL:', account.provider, 'isDesktop:', isDesktop)
      const res = await batchLoginAPI.getOAuth2AuthUrl(account.provider, isDesktop)
      console.log('[OAuth2] API 返回:', res)
      
      // 处理不同的返回格式
      const authUrl = res.data?.auth_url || res.auth_url
      
      if (!authUrl) {
        console.error('[OAuth2] 没有获取到授权 URL:', res)
        errorMessage.value = res.message || res.data?.message || '获取授权链接失败'
        account.status = 'error'
        continue
      }
      
      console.log('[OAuth2] 打开授权页面:', authUrl)
      
      // 打开授权页面（桌面端用系统浏览器，Web端用弹窗）
      const opened = await openExternal(authUrl)
      
      if (!opened) {
        console.error('[OAuth2] 打开授权页面失败')
        errorMessage.value = '无法打开授权页面，请手动复制链接到浏览器'
        account.status = 'error'
        continue
      }
      
      const maxWait = 5 * 60 * 1000
      let authorized = false

      if (isDesktop) {
        const callbackResult = await waitForDesktopOAuthCallback(account.email, maxWait)
        if (callbackResult?.success === false) {
          account.status = 'error'
          errorMessage.value = callbackResult.error || '授权失败'
          continue
        }

        if (callbackResult?.success) {
          authorized = await waitForMailboxAuthorized(account.email)
        }
      } else {
        authorized = await waitForMailboxAuthorized(account.email, maxWait, 3000)
      }
      
      if (authorized) {
        account.status = 'success'
      } else {
        account.status = 'error'
        errorMessage.value = '授权超时或未完成，请重试'
      }
      
    } catch (e: any) {
      console.error('[OAuth2] 授权失败:', e)
      errorMessage.value = e.message || '授权失败'
      account.status = 'error'
    }
  }
  
  isAuthorizing.value = false
  currentIndex.value = -1
  
  // 通知父组件授权完成
  emit('complete', {
    successCount: successCount.value,
    failCount: failCount.value,
    accounts: accounts.value
  })
}

const handleClose = () => {
  if (isAuthorizing.value) {
    // 授权中不允许关闭
    return
  }
  emit('close')
}
</script>
