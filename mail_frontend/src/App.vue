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
import { registerMaintenanceCallback } from '@/services/api'

const userStore = useUserStore()
const maintenanceRef = ref<InstanceType<typeof SystemMaintenance>>()
const updaterRef = ref<InstanceType<typeof AppUpdater>>()

// 启用页面访问统计
const { recordPageView } = usePageTracking()

// 应用启动时检查认证状态
onMounted(async () => {
  // 桌面端启动时检查更新
  updaterRef.value?.checkForUpdates()

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
