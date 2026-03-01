<template>
  <div>
    <!-- 顶部导航 -->
    <PageHeader />
    
    <div class="min-h-screen bg-gray-50 py-8">
    <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
      <!-- 页面标题 -->
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-black">问题建议</h1>
        <p class="mt-2 text-black">提交问题建议或查看处理进度</p>
      </div>

      <!-- 标签切换 -->
      <div class="mb-6">
        <div class="border-b border-gray-200">
          <nav class="-mb-px flex space-x-8">
            <button
              @click="activeTab = 'submit'"
              :class="[
                'py-2 px-1 border-b-2 font-medium text-sm',
                activeTab === 'submit'
                  ? 'border-primary-500 text-primary-600'
                  : 'border-transparent text-black hover:text-black hover:border-gray-300'
              ]"
            >
              提交反馈
            </button>
            <button
              @click="activeTab = 'history'"
              :class="[
                'py-2 px-1 border-b-2 font-medium text-sm',
                activeTab === 'history'
                  ? 'border-primary-500 text-primary-600'
                  : 'border-transparent text-black hover:text-black hover:border-gray-300'
              ]"
            >
              我的反馈
              <span v-if="feedbackCount > 0" class="ml-2 bg-primary-100 text-primary-600 py-1 px-2 rounded-full text-xs">
                {{ feedbackCount }}
              </span>
            </button>
          </nav>
        </div>
      </div>

      <!-- 提交反馈标签页 -->
      <div v-if="activeTab === 'submit'" class="bg-white rounded-lg shadow-sm border p-6">
        <h2 class="text-xl font-semibold text-black mb-6">提交问题建议</h2>
        
        <form @submit.prevent="submitFeedback" class="space-y-6">
          <!-- 反馈类型 -->
          <div>
            <label class="block text-sm font-medium text-black mb-2">
              反馈类型 <span class="text-red-500">*</span>
            </label>
            <CustomSelect
              v-model="feedbackForm.type"
              :options="typeOptions"
              placeholder="请选择反馈类型"
            />
            <div v-if="errors.type" class="mt-1 text-sm text-red-600">{{ errors.type }}</div>
          </div>

          <!-- 标题 -->
          <div>
            <label class="block text-sm font-medium text-black mb-2">
              标题 <span class="text-red-500">*</span>
            </label>
            <BaseInput
              v-model="feedbackForm.title"
              type="text"
              :error-message="errors.title"
              placeholder="请简要描述问题..."
            />
            <div v-if="errors.title" class="mt-1 text-sm text-red-600">{{ errors.title }}</div>
          </div>

          <!-- 详细描述 -->
          <div>
            <label class="block text-sm font-medium text-black mb-2">
              详细描述 <span class="text-red-500">*</span>
            </label>
            <BaseTextarea
              v-model="feedbackForm.content"
              rows="6"
              :error-message="errors.content"
              placeholder="请详细描述遇到的问题、建议的功能或其他反馈..."
            />
            <div v-if="errors.content" class="mt-1 text-sm text-red-600">{{ errors.content }}</div>
          </div>

          <!-- 联系方式 -->
          <div>
            <label class="block text-sm font-medium text-black mb-2">
              联系方式（可选）
            </label>
            <BaseInput
              v-model="feedbackForm.contact_info"
              type="text"
              placeholder="如需回复，请留下您的联系方式（邮箱、QQ等）"
            />
          </div>

          <!-- 提交按钮 -->
          <div class="flex justify-end">
            <button
              type="submit"
              :disabled="submitting"
              class="px-6 py-2 bg-primary-600 hover:bg-primary-700 text-white font-medium rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <span v-if="submitting">提交中...</span>
              <span v-else>提交反馈</span>
            </button>
          </div>
        </form>
      </div>

      <!-- 我的反馈标签页 -->
      <div v-if="activeTab === 'history'">
        <div v-if="loadingHistory" class="flex justify-center py-12">
          <div class="inline-flex items-center">
            <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-black" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            加载中...
          </div>
        </div>

        <div v-else-if="feedbacks.length === 0" class="text-center py-12 bg-white rounded-lg shadow-sm">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
          </svg>
          <h3 class="mt-2 text-sm font-medium text-black">暂无反馈记录</h3>
          <p class="mt-1 text-sm text-black">您还没有提交过任何反馈</p>
          <div class="mt-6">
            <button
              @click="activeTab = 'submit'"
              class="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700"
            >
              提交反馈
            </button>
          </div>
        </div>

        <div v-else class="space-y-6">
          <!-- 反馈列表 -->
          <div v-for="feedback in feedbacks" :key="feedback.id" class="bg-white shadow rounded-lg overflow-hidden">
            <div class="px-6 py-4 border-b border-gray-200">
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3">
                  <span :class="getTypeClass(feedback.type)" class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium">
                    {{ getTypeText(feedback.type) }}
                  </span>
                  <span :class="getStatusClass(feedback.status)" class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium">
                    {{ getStatusText(feedback.status) }}
                  </span>
                </div>
                <span class="text-sm text-black">{{ formatDate(feedback.created_at) }}</span>
              </div>
            </div>

            <div class="px-6 py-4">
              <h3 class="text-lg font-medium text-black mb-3">{{ feedback.title }}</h3>
              <p class="text-black whitespace-pre-wrap mb-4">{{ feedback.content }}</p>
              
              <div v-if="feedback.contact_info" class="text-sm text-black mb-4">
                <span class="font-medium">联系方式:</span> {{ feedback.contact_info }}
              </div>

              <!-- 管理员回复 -->
              <div v-if="feedback.admin_reply" class="bg-primary-50 border-l-4 border-blue-400 p-4 rounded">
                <div class="flex items-start">
                  <div class="flex-shrink-0">
                    <svg class="h-5 w-5 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                    </svg>
                  </div>
                  <div class="ml-3">
                    <p class="text-sm font-medium text-primary-700">
                      管理员回复 {{ feedback.admin_name ? `(${feedback.admin_name})` : '' }}
                    </p>
                    <p class="text-sm text-primary-600 mt-1 whitespace-pre-wrap">{{ feedback.admin_reply }}</p>
                    <p v-if="feedback.updated_at" class="text-xs text-primary-500 mt-2">
                      {{ formatDate(feedback.updated_at) }}
                    </p>
                  </div>
                </div>
              </div>

              <!-- 未回复状态 -->
              <div v-else-if="feedback.status === 'pending'" class="bg-yellow-50 border-l-4 border-yellow-400 p-4 rounded">
                <div class="flex">
                  <div class="flex-shrink-0">
                    <svg class="h-5 w-5 text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                  </div>
                  <div class="ml-3">
                    <p class="text-sm text-yellow-700">您的反馈正在等待处理，我们会尽快回复您。</p>
                  </div>
                </div>
              </div>

              <div v-else-if="feedback.status === 'processing'" class="bg-orange-50 border-l-4 border-orange-400 p-4 rounded">
                <div class="flex">
                  <div class="flex-shrink-0">
                    <svg class="h-5 w-5 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                  </div>
                  <div class="ml-3">
                    <p class="text-sm text-orange-700">您的反馈正在处理中，请耐心等待。</p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 分页 -->
          <div v-if="pagination.total_pages > 1" class="bg-white px-6 py-4 border-t border-gray-200 rounded-lg">
            <div class="flex items-center justify-between">
              <div class="text-sm text-black">
                共 {{ pagination.total }} 条，第 {{ pagination.page }} / {{ pagination.total_pages }} 页
              </div>
              <div class="flex space-x-2">
                <button
                  :disabled="pagination.page <= 1"
                  @click="loadFeedbacks(pagination.page - 1)"
                  class="px-3 py-1 border border-gray-300 rounded-md text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
                >
                  上一页
                </button>
                <button
                  :disabled="pagination.page >= pagination.total_pages"
                  @click="loadFeedbacks(pagination.page + 1)"
                  class="px-3 py-1 border border-gray-300 rounded-md text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
                >
                  下一页
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>


    </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import PageHeader from '@/components/PageHeader/index.vue'
import BaseInput from '@/components/BaseInput/index.vue'
import BaseTextarea from '@/components/BaseTextarea/index.vue'
import { feedbackAPI } from '@/api/feedback'
import CustomSelect from '@/components/CustomSelect/index.vue'
import { showMessage } from '@/utils/message'

// 标签页状态
const activeTab = ref('submit')

// 提交反馈相关
const submitting = ref(false)
const feedbackForm = reactive({
  type: '',
  title: '',
  content: '',
  contact_info: ''
})

// 表单验证错误
const errors = reactive({
  type: '',
  title: '',
  content: ''
})

// 我的反馈相关
const loadingHistory = ref(false)
const feedbacks = ref<any[]>([])
const feedbackCount = computed(() => feedbacks.value.length)

// 分页信息
const pagination = reactive({
  page: 1,
  page_size: 10,
  total: 0,
  total_pages: 0
})



// 反馈类型选项
const typeOptions = [
  { label: '功能建议', value: 'suggestion' },
  { label: '错误报告', value: 'bug' },
  { label: '使用问题', value: 'question' },
  { label: '其他', value: 'other' }
]

// 表单验证
const validateForm = (): boolean => {
  // 重置错误
  errors.type = ''
  errors.title = ''
  errors.content = ''

  let isValid = true

  if (!feedbackForm.type) {
    errors.type = '请选择反馈类型'
    isValid = false
  }

  if (!feedbackForm.title.trim()) {
    errors.title = '请填写标题'
    isValid = false
  } else if (feedbackForm.title.trim().length < 5) {
    errors.title = '标题至少需要5个字符'
    isValid = false
  }

  if (!feedbackForm.content.trim()) {
    errors.content = '请填写详细描述'
    isValid = false
  } else if (feedbackForm.content.trim().length < 10) {
    errors.content = '内容至少需要10个字符'
    isValid = false
  }

  return isValid
}

// 提交反馈
const submitFeedback = async () => {
  if (!validateForm()) {
    return
  }

  submitting.value = true
  try {
    const response = await feedbackAPI.create(feedbackForm)
    if (response.code === 0) {
      showMessage('反馈提交成功，感谢您的建议！', 'success')
      // 重置表单
      feedbackForm.type = ''
      feedbackForm.title = ''
      feedbackForm.content = ''
      feedbackForm.contact_info = ''
      // 切换到历史记录并刷新
      activeTab.value = 'history'
      await loadFeedbacks()
    } else {
      showMessage(response.message || '提交失败，请稍后再试', 'error')
    }
  } catch (error: any) {
    console.error('提交反馈失败:', error)
    showMessage('提交失败，请稍后再试', 'error')
  } finally {
    submitting.value = false
  }
}

// 加载反馈历史
const loadFeedbacks = async (page = 1) => {
  loadingHistory.value = true
  try {
    const response = await feedbackAPI.getMyFeedbacks(page, pagination.page_size)
    if (response.code === 0) {
      feedbacks.value = response.data.feedbacks
      pagination.page = response.data.page
      pagination.total = response.data.total
      pagination.total_pages = response.data.total_pages
    }
  } catch (error: any) {
    console.error('加载反馈列表失败:', error)
  } finally {
    loadingHistory.value = false
  }
}



// 获取类型样式
const getTypeClass = (type: string) => {
  const classes = {
    suggestion: 'bg-primary-100 text-success-800',
    bug: 'bg-red-100 text-red-800',
    question: 'bg-primary-100 text-primary-800',
    other: 'bg-gray-100 text-black'
  }
  return classes[type as keyof typeof classes] || classes.other
}

// 获取类型文本
const getTypeText = (type: string) => {
  const texts = {
    suggestion: '功能建议',
    bug: '错误报告',
    question: '使用问题',
    other: '其他'
  }
  return texts[type as keyof typeof texts] || '其他'
}

// 获取状态样式
const getStatusClass = (status: string) => {
  const classes = {
    pending: 'bg-yellow-100 text-yellow-800',
    processing: 'bg-orange-100 text-orange-800',
    resolved: 'bg-primary-100 text-success-800',
    closed: 'bg-gray-100 text-black'
  }
  return classes[status as keyof typeof classes] || classes.pending
}

// 获取状态文本
const getStatusText = (status: string) => {
  const texts = {
    pending: '待处理',
    processing: '处理中',
    resolved: '已解决',
    closed: '已关闭'
  }
  return texts[status as keyof typeof texts] || '待处理'
}

// 格式化日期
const formatDate = (dateStr: string) => {
  if (!dateStr) return ''
  const date = new Date(dateStr)
  return date.toLocaleString('zh-CN')
}

// 初始化
onMounted(() => {
  // 加载历史记录
  loadFeedbacks()
})
</script>