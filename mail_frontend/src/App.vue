<template>
  <div id="app">
    <RouterView />
    <FloatingFeedback />
    <SystemMaintenance ref="maintenanceRef" />
    <AppUpdater ref="updaterRef" />
  </div>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router'
import { onMounted, ref } from 'vue'
import { useUserStore } from '@/stores/user'
import { usePageTracking } from '@/composables/usePageTracking'
import FloatingFeedback from '@/components/FloatingFeedback/index.vue'
import SystemMaintenance from '@/components/SystemMaintenance/index.vue'
import AppUpdater from '@/components/AppUpdater/index.vue'
import { registerMaintenanceCallback, isTauri } from '@/services/api'

const userStore = useUserStore()
const maintenanceRef = ref<InstanceType<typeof SystemMaintenance>>()
const updaterRef = ref<InstanceType<typeof AppUpdater>>()

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

  try {
    await userStore.checkAuth()
  } catch (e) {
    console.error('checkAuth failed:', e)
  }

  registerMaintenanceCallback(() => {
    maintenanceRef.value?.show()
  })
})
</script>

