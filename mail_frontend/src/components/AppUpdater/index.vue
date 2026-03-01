<template>
  <div v-if="showModal" class="update-overlay">
    <div class="update-modal">
      <!-- 提示更新 -->
      <template v-if="phase === 'confirm'">
        <div class="update-icon">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5M16.5 12L12 16.5m0 0L7.5 12m4.5 4.5V3" />
          </svg>
        </div>
        <h3 class="update-title">发现新版本</h3>
        <p class="update-desc">v{{ version }} 已发布，建议立即更新以获得最佳体验</p>
        <div class="update-actions">
          <button class="btn-later" @click="dismiss">稍后再说</button>
          <button class="btn-update" @click="startUpdate">立即更新</button>
        </div>
      </template>

      <!-- 下载进度 -->
      <template v-if="phase === 'downloading'">
        <h3 class="update-title">正在更新</h3>
        <div class="progress-wrap">
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: percent + '%' }"></div>
          </div>
          <span class="progress-text">{{ percent }}%</span>
        </div>
        <p class="update-hint">{{ hint }}</p>
      </template>

      <!-- 更新失败 -->
      <template v-if="phase === 'error'">
        <div class="update-icon error">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z" />
          </svg>
        </div>
        <h3 class="update-title">更新失败</h3>
        <p class="update-desc">{{ errorMsg }}</p>
        <div class="update-actions">
          <button class="btn-update" @click="dismiss">我知道了</button>
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { isTauri } from '@/services/api'

const showModal = ref(false)
const phase = ref<'confirm' | 'downloading' | 'error'>('confirm')
const version = ref('')
const percent = ref(0)
const hint = ref('准备下载...')
const errorMsg = ref('')
let pendingUpdate: any = null

function dismiss() {
  showModal.value = false
  pendingUpdate = null
}

async function startUpdate() {
  if (!pendingUpdate) return
  phase.value = 'downloading'
  percent.value = 0
  hint.value = '准备下载...'

  try {
    let downloaded = 0
    let total = 0
    await pendingUpdate.downloadAndInstall((event: any) => {
      if (event.event === 'Started') {
        total = event.data.contentLength || 0
        hint.value = '正在下载...'
      } else if (event.event === 'Progress') {
        downloaded += event.data.chunkLength || 0
        percent.value = total > 0 ? Math.round((downloaded / total) * 100) : 0
        const mb = (downloaded / 1024 / 1024).toFixed(1)
        const totalMb = (total / 1024 / 1024).toFixed(1)
        hint.value = `${mb} MB / ${totalMb} MB`
      } else if (event.event === 'Finished') {
        percent.value = 100
        hint.value = '下载完成，正在安装...'
      }
    })
    hint.value = '安装完成，正在重启...'
    const { relaunch } = await import('@tauri-apps/plugin-process')
    await relaunch()
  } catch (e: any) {
    phase.value = 'error'
    errorMsg.value = String(e)
  }
}

async function checkForUpdates() {
  if (!isTauri()) return
  try {
    const { check } = await import('@tauri-apps/plugin-updater')
    const update = await check()
    if (update) {
      pendingUpdate = update
      version.value = update.version
      phase.value = 'confirm'
      showModal.value = true
    }
  } catch (e) {
    console.log('检查更新:', e)
  }
}

defineExpose({ checkForUpdates })
</script>

<style scoped>
.update-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 99999;
}

.update-modal {
  background: #fff;
  border-radius: 16px;
  padding: 32px;
  width: 380px;
  text-align: center;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.15);
}

.update-icon {
  color: rgb(var(--color-primary-500));
  margin-bottom: 12px;
}

.update-icon.error {
  color: #ef4444;
}

.update-title {
  font-size: 18px;
  font-weight: 600;
  color: #111827;
  margin: 0 0 8px;
}

.update-desc {
  font-size: 14px;
  color: #6b7280;
  margin: 0 0 24px;
  line-height: 1.5;
}

.update-actions {
  display: flex;
  gap: 12px;
}

.btn-later {
  flex: 1;
  padding: 10px 0;
  border: 1px solid #d1d5db;
  border-radius: 10px;
  background: #fff;
  color: #374151;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-later:hover {
  background: #f3f4f6;
}

.btn-update {
  flex: 1;
  padding: 10px 0;
  border: none;
  border-radius: 10px;
  background: rgb(var(--color-primary-500));
  color: #fff;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-update:hover {
  background: rgb(var(--color-primary-600));
}

.progress-wrap {
  display: flex;
  align-items: center;
  gap: 12px;
  margin: 20px 0 12px;
}

.progress-bar {
  flex: 1;
  height: 8px;
  background: #e5e7eb;
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: rgb(var(--color-primary-500));
  border-radius: 4px;
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 14px;
  font-weight: 600;
  color: rgb(var(--color-primary-600));
  min-width: 40px;
}

.update-hint {
  font-size: 13px;
  color: #9ca3af;
  margin: 0;
}
</style>
