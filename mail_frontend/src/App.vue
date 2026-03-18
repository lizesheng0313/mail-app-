<template>
  <div id="app">
    <RouterView />
    <FloatingFeedback />
    <SystemMaintenance ref="maintenanceRef" />
    <AppUpdater ref="updaterRef" />
  </div>
</template>

<script setup lang="ts">
import { RouterView, useRouter } from 'vue-router'
import { onBeforeUnmount, onMounted, ref } from 'vue'
import { useUserStore } from '@/stores/user'
import { usePageTracking } from '@/composables/usePageTracking'
import FloatingFeedback from '@/components/FloatingFeedback/index.vue'
import SystemMaintenance from '@/components/SystemMaintenance/index.vue'
import AppUpdater from '@/components/AppUpdater/index.vue'
import { registerMaintenanceCallback, isTauri } from '@/services/api'
import { showMessage } from '@/utils/message'

const router = useRouter()
const userStore = useUserStore()
const maintenanceRef = ref<InstanceType<typeof SystemMaintenance>>()
const updaterRef = ref<InstanceType<typeof AppUpdater>>()
let unlistenOAuthCallback: null | (() => void) = null

// 启用页面访问统计
const { recordPageView } = usePageTracking()

// 应用启动时检查认证状态
onMounted(async () => {
  // 桌面端启动时检查更新
  updaterRef.value?.checkForUpdates()

  // Windows 桌面端：在标题栏显示版本号
  if (isTauri() && navigator.platform.toUpperCase().includes('WIN')) {
    try {
      const { getVersion } = await import('@tauri-apps/api/app')
      const { getCurrentWindow } = await import('@tauri-apps/api/window')
      const ver = await getVersion()
      await getCurrentWindow().setTitle(`肥猫猫 v${ver}`)
    } catch (e) {
      console.error('设置窗口标题失败:', e)
    }
  }

  if (isTauri()) {
    try {
      const { listen } = await import('@tauri-apps/api/event')
      unlistenOAuthCallback = await listen('oauth-callback', async (event: any) => {
        const payload = event.payload || {}
        const path = payload.path || '/'
        const query = payload.query || {}

        if (path === '/oauth2/callback') {
          window.dispatchEvent(new CustomEvent('oauth2-callback', { detail: query }))
          if (query.oauth2_success === '1') {
            showMessage(`邮箱授权成功${query.email ? `: ${query.email}` : ''}`, 'success')
          } else if (query.oauth2_error) {
            showMessage(query.oauth2_error, 'error')
          }
          return
        }

        const search = new URLSearchParams(query).toString()
        const target = search ? `${path}?${search}` : path
        await router.replace(target)
      })
    } catch (e) {
      console.error('注册桌面端 OAuth 回调监听失败:', e)
    }
  }

  try {
    await userStore.checkAuth()
  } catch (e) {
    console.error('checkAuth failed:', e)
  }

  registerMaintenanceCallback(() => {
    maintenanceRef.value?.show()
  })
})

onBeforeUnmount(() => {
  unlistenOAuthCallback?.()
})
</script>
