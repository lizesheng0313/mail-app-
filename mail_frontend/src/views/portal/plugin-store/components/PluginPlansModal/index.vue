<template>
  <div class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
    <div class="relative top-10 mx-auto p-0 border max-w-6xl shadow-lg rounded-lg bg-white">
      <!-- 头部 -->
      <div class="flex items-center justify-between p-6 border-b">
        <h3 class="text-xl font-semibold text-black">
          {{ plugin.name }} - 选择套餐
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

      <!-- 内容 -->
      <div class="p-6">
        <!-- 插件基本信息 -->
        <div class="flex items-start space-x-4 mb-8">
          <div class="w-16 h-16 bg-gradient-to-br from-primary-600 to-primary-700 rounded-lg flex items-center justify-center">
            <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
            </svg>
          </div>
          <div>
            <h2 class="text-2xl font-bold text-black mb-2">{{ plugin.name }}</h2>
            <p class="text-black">{{ plugin.description }}</p>
          </div>
        </div>

        <!-- 套餐选择 -->
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <div
            v-for="plan in plans"
            :key="plan.plan_id"
            :class="[
              'relative rounded-lg border-2 p-6 cursor-pointer transition-all hover:shadow-lg',
              plan.is_popular ? 'border-accent-500 bg-accent-50' : 'border-gray-200 hover:border-accent-300',
              selectedPlan?.plan_id === plan.plan_id ? 'ring-2 ring-accent-500 bg-accent-50' : ''
            ]"
            @click="selectPlan(plan)"
          >
            <!-- 热门标签 -->
            <div v-if="plan.is_popular" class="absolute -top-3 left-1/2 transform -translate-x-1/2">
              <span class="bg-accent-500 text-white px-3 py-1 rounded-full text-xs font-medium">
                热门推荐
              </span>
            </div>

            <!-- 已购买标签 -->
            <div v-if="plan.user_has_plan" class="absolute -top-3 right-4">
              <span class="bg-primary-600 text-white px-3 py-1 rounded-full text-xs font-medium">
                已购买
              </span>
            </div>

            <!-- 套餐名称和价格 -->
            <div class="text-center mb-4">
              <h3 class="text-lg font-semibold text-black mb-2">{{ plan.plan_name }}</h3>
              <div class="mb-2">
                <span v-if="plan.is_free" class="text-2xl font-bold text-accent-600">免费</span>
                <span v-else class="text-2xl font-bold text-black">{{ plan.price }}奶片</span>
                <span class="text-sm text-black">/ {{ plan.validity_days }}天</span>
              </div>
              <p class="text-sm text-black">{{ plan.description }}</p>
            </div>

            <!-- 功能特性 -->
            <div class="space-y-3 mb-6">
              <div class="flex items-center text-sm">
                <svg class="w-4 h-4 text-success-500 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                每月{{ plan.features.auto_workflow_triggers_per_month }}次自动工作流
              </div>
              <div class="flex items-center text-sm">
                <svg class="w-4 h-4 text-success-500 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                手动执行不限次数
              </div>
              <div class="flex items-center text-sm">
                <svg class="w-4 h-4 text-success-500 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                {{ plan.features.concurrent_workflows }}个并发工作流
              </div>
              <div class="flex items-center text-sm">
                <svg class="w-4 h-4 text-success-500 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                支持{{ plan.features.supported_browsers.join(', ') }}
              </div>
              <div v-if="plan.features.screenshot_support" class="flex items-center text-sm">
                <svg class="w-4 h-4 text-success-500 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                截图支持
              </div>
              <div v-if="plan.features.proxy_support" class="flex items-center text-sm">
                <svg class="w-4 h-4 text-success-500 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                代理支持
              </div>
              <div v-if="plan.features.priority_support" class="flex items-center text-sm">
                <svg class="w-4 h-4 text-success-500 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
                优先技术支持
              </div>
            </div>

            <!-- 套餐状态 -->
            <div class="text-center">
              <button
                v-if="plan.user_has_plan"
                disabled
                class="w-full bg-primary-100 text-primary-800 font-medium py-2 px-4 rounded-lg cursor-not-allowed text-sm"
              >
                已激活
                <div v-if="plan.expires_at" class="text-xs mt-1">
                  到期: {{ formatDate(plan.expires_at) }}
                </div>
              </button>
              <button
                v-else-if="selectedPlan?.plan_id === plan.plan_id"
                class="w-full bg-primary-600 text-white font-medium py-2 px-4 rounded-lg text-sm"
              >
                已选择
              </button>
              <button
                v-else
                class="w-full border border-gray-300 text-black font-medium py-2 px-4 rounded-lg hover:bg-gray-50 text-sm"
              >
                选择此套餐
              </button>
            </div>
          </div>
        </div>

        <!-- 授权码输入区域 -->
        <div v-if="selectedPlan && !selectedPlan.is_free && !selectedPlan.user_has_plan" class="mt-8 p-6 bg-gray-50 rounded-lg">
          <h4 class="text-lg font-semibold text-black mb-4">
            输入 {{ selectedPlan.plan_name }} 授权码
          </h4>
          <div class="flex space-x-4">
            <div class="flex-1">
              <BaseInput
                v-model="authCode"
                type="text"
                placeholder="请输入授权码"
                @enter="handlePurchase"
              />
            </div>
            <button
              @click="handlePurchase"
              :disabled="!authCode.trim() || purchasing"
              class="px-6 py-2 bg-primary-600 hover:bg-primary-700 disabled:bg-gray-400 text-white font-medium rounded-lg transition-colors"
            >
              {{ purchasing ? '激活中...' : '激活套餐' }}
            </button>
          </div>
          <p class="mt-2 text-sm text-black">
            激活后立即生效，有效期 {{ selectedPlan.validity_days }} 天
          </p>
        </div>

        <!-- 免费套餐激活 -->
        <div v-else-if="selectedPlan && selectedPlan.is_free && !selectedPlan.user_has_plan" class="mt-8 text-center">
          <button
            @click="handlePurchase"
            :disabled="purchasing"
            class="px-8 py-3 bg-primary-600 hover:bg-primary-700 disabled:bg-gray-400 text-white font-medium rounded-lg transition-colors"
          >
            {{ purchasing ? '激活中...' : '免费激活' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { showConfirm } from '@/utils/dialog'
import { showMessage } from '@/utils/message'
import BaseInput from '@/components/BaseInput/index.vue'

const router = useRouter()

const props = defineProps({
  plugin: {
    type: Object,
    required: true
  },
  plans: {
    type: Array,
    required: true
  }
})

const emit = defineEmits(['close', 'purchase'])

const selectedPlan = ref(null)
const authCode = ref('')
const purchasing = ref(false)

// 选择套餐
const selectPlan = (plan) => {
  if (!plan.user_has_plan) {
    selectedPlan.value = plan
    authCode.value = ''
  }
}

// 处理购买/激活
const handlePurchase = async () => {
  if (!selectedPlan.value) return
  
  if (!selectedPlan.value.is_free && !authCode.value.trim()) {
    showMessage('请输入授权码', 'warning')
    return
  }
  
  // 显示确认对话框
  const planName = selectedPlan.value.plan_name
  const isFree = selectedPlan.value.is_free
  const price = selectedPlan.value.price
  const validityDays = selectedPlan.value.validity_days
  
  let confirmMessage = ''
  if (isFree) {
    confirmMessage = `确认激活【${planName}】套餐？\n\n激活后立即生效，有效期 ${validityDays} 天`
  } else {
    confirmMessage = `确认购买【${planName}】套餐？\n\n• 价格：${price} 奶片\n• 有效期：${validityDays} 天\n• 授权码：${authCode.value.trim()}`
  }
  
  const confirmed = await showConfirm(confirmMessage, '购买确认')
  if (!confirmed) return
  
  purchasing.value = true
  
  try {
    await emit('purchase', {
      plugin_id: props.plugin.plugin_id,
      plan_id: selectedPlan.value.plan_id,
      auth_code: selectedPlan.value.is_free ? null : authCode.value.trim()
    })
    
    // 购买成功，跳转到插件商店
    showMessage('购买成功！', 'success')
    setTimeout(() => {
      router.push('/plugin-store')
    }, 1000)
  } catch (error) {
    console.error('购买失败:', error)
    showMessage(error.message || '购买失败', 'error')
  } finally {
    purchasing.value = false
  }
}

// 格式化日期
const formatDate = (timestamp) => {
  if (!timestamp) return ''
  const date = new Date(timestamp)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  })
}
</script>

<style scoped>
.grid {
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
}

@media (min-width: 1024px) {
  .grid {
    grid-template-columns: repeat(4, 1fr);
  }
}
</style>