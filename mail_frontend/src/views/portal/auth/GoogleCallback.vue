<template>
  <div class="min-h-screen bg-gray-50 flex items-center justify-center">
    <div class="max-w-md w-full text-center">
      <!-- Loading 状态 -->
      <div v-if="loading" class="bg-white rounded-lg shadow-lg p-8">
        <div class="inline-flex items-center justify-center w-16 h-16 bg-primary-100 rounded-full mb-4">
          <svg class="animate-spin h-8 w-8 text-primary-600" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
        </div>
        <h2 class="text-xl font-semibold text-gray-900 mb-2">正在登录...</h2>
        <p class="text-gray-600">请稍候，我们正在处理您的 Google 登录</p>
      </div>
      
      <!-- 错误状态 -->
      <div v-else-if="error" class="bg-white rounded-lg shadow-lg p-8">
        <div class="inline-flex items-center justify-center w-16 h-16 bg-red-100 rounded-full mb-4">
          <svg class="h-8 w-8 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </div>
        <h2 class="text-xl font-semibold text-gray-900 mb-2">登录失败</h2>
        <p class="text-gray-600 mb-6">{{ error }}</p>
        <button
          @click="router.push('/login')"
          class="inline-flex items-center px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white rounded-lg transition-colors"
        >
          返回登录页
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useUserStore } from '@/stores/user'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore()

const loading = ref(true)
const error = ref('')

const handleGoogleCallback = async () => {
  try {
    // 获取URL参数
    const code = route.query.code as string
    const state = route.query.state as string
    
    if (!code) {
      error.value = '未获取到授权码，请重试'
      loading.value = false
      return
    }
    
    // 调用后端处理Google回调
    const response = await fetch(`/mail-api/v1/auth/google/callback?code=${encodeURIComponent(code)}&state=${encodeURIComponent(state)}`)
    const result = await response.json()
    
    if (result.code === 0 && result.data) {
      // 保存token到store
      userStore.setToken(result.data.token)
      userStore.setUser(result.data.user)
      
      // 跳转到首页
      setTimeout(() => {
        router.push('/')
      }, 500)
    } else {
      error.value = result.message || 'Google登录失败'
      loading.value = false
    }
  } catch (err: any) {
    console.error('Google callback error:', err)
    error.value = '网络错误，请稍后重试'
    loading.value = false
  }
}

onMounted(() => {
  handleGoogleCallback()
})
</script>
