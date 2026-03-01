<template>
  <div class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
    <div class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
      <!-- 头部 -->
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-medium text-black">
          {{ plugin?.is_installed && hasExpiredLicense ? '更新授权码' : '输入授权码' }}
        </h3>
        <button
          @click="$emit('close')"
          class="text-gray-400 hover:text-black"
        >
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- 插件信息 -->
      <div class="mb-6">
        <div class="flex items-center space-x-3 p-3 bg-gray-50 rounded-lg">
          <div class="w-10 h-10 bg-gradient-to-br from-primary-600 to-primary-700 rounded-lg flex items-center justify-center">
            <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
            </svg>
          </div>
          <div>
            <h4 class="font-medium text-black">{{ plugin?.name }}</h4>
            <p class="text-sm text-black">需要授权</p>
            <p v-if="plugin?.license_expiry" class="text-sm text-red-500">
              授权到期: {{ formatDate(plugin.license_expiry) }}
            </p>
          </div>
        </div>
      </div>

      <!-- 授权码输入 -->
      <div class="mb-6">
        <label for="authCode" class="block text-sm font-medium text-black mb-2">
          授权码
        </label>
        <BaseInput
          id="authCode"
          v-model="authCode"
          type="text"
          placeholder="请输入授权码"
          @enter="handleConfirm"
        />
        <p class="mt-2 text-sm text-black">
          {{ plugin?.is_installed && hasExpiredLicense 
            ? '授权码已过期，请输入新的授权码' 
            : '请输入有效的授权码来获取此插件' }}
        </p>
      </div>

      <!-- 授权状态提示 -->
      <div v-if="plugin?.license_expiry" class="mb-6">
        <div v-if="hasExpiredLicense" class="p-3 bg-red-50 border border-red-200 rounded-lg">
          <div class="flex">
            <div class="flex-shrink-0">
              <svg class="h-5 w-5 text-red-400" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
              </svg>
            </div>
            <div class="ml-3">
              <h3 class="text-sm font-medium text-red-800">授权已过期</h3>
              <p class="mt-1 text-sm text-red-700">
                您的插件授权已于 {{ formatDate(plugin.license_expiry) }} 过期，请输入新的授权码以继续使用。
              </p>
            </div>
          </div>
        </div>
        <div v-else class="p-3 bg-success-50 border border-success-200 rounded-lg">
          <div class="flex">
            <div class="flex-shrink-0">
              <svg class="h-5 w-5 text-success-400" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
              </svg>
            </div>
            <div class="ml-3">
              <h3 class="text-sm font-medium text-success-800">授权有效</h3>
              <p class="mt-1 text-sm text-success-700">
                您的插件授权将于 {{ formatDate(plugin.license_expiry) }} 到期。
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- 授权说明 -->
      <div class="mb-6 p-3 bg-primary-50 rounded-lg">
        <div class="flex">
          <div class="flex-shrink-0">
            <svg class="h-5 w-5 text-success-600" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
            </svg>
          </div>
          <div class="ml-3">
            <h3 class="text-sm font-medium text-success-800">
              授权说明
            </h3>
            <div class="mt-2 text-sm text-success-700">
              <ul class="list-disc list-inside space-y-1">
                <li>授权码激活后立即生效</li>
                <li>插件获取后即可使用</li>
                <li>支持后续版本更新</li>
                <li>如有问题请联系客服</li>
              </ul>
            </div>
          </div>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="flex justify-end space-x-3">
        <button
          @click="$emit('close')"
          class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-black hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
        >
          取消
        </button>
        <button
          @click="handleConfirm"
          :disabled="!authCode.trim() || props.loading"
          class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {{ props.loading ? '处理中...' : (hasExpiredLicense ? '更新授权' : '确认获取') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import BaseInput from '@/components/BaseInput/index.vue'

const props = defineProps({
  plugin: {
    type: Object,
    required: true
  },
  loading: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['close', 'confirm'])

const authCode = ref('')

// 检查授权是否过期
const hasExpiredLicense = computed(() => {
  if (!props.plugin?.license_expiry) return false
  return new Date(props.plugin.license_expiry) < new Date()
})

// 格式化日期
const formatDate = (dateString) => {
  if (!dateString) return ''
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  })
}

const handleConfirm = () => {
  if (!authCode.value.trim()) return
  
  emit('confirm', {
    authCode: authCode.value.trim(),
    isUpdate: hasExpiredLicense.value
  })
}
</script>
