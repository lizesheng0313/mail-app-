<template>
  <div class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
    <div class="relative top-10 mx-auto p-0 border max-w-4xl shadow-lg rounded-lg bg-white">
      <!-- 头部 -->
      <div class="flex items-center justify-between p-6 border-b">
        <h3 class="text-xl font-semibold text-black">
          {{ isEdit ? '编辑触发器' : '创建触发器' }}
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
        <form @submit.prevent="handleSubmit">
          <!-- 基本信息 -->
          <div class="mb-6">
            <h4 class="text-lg font-medium text-black mb-4">基本信息</h4>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label for="name" class="block text-sm font-medium text-black mb-2">
                  触发器名称 *
                </label>
                <BaseInput
                  id="name"
                  v-model="form.name"
                  type="text"
                  required
                  placeholder="输入触发器名称"
                  :error-message="errors.name"
                  @input="clearError('name')"
                />
              </div>
              <div>
                <label for="trigger_type" class="block text-sm font-medium text-black mb-2">
                  触发器类型 *
                </label>
                <CustomSelect
                  :model-value="form.trigger_type || ''"
                  :options="triggerTypeOptions"
                  placeholder="选择触发器类型"
                  @update:modelValue="(value) => { form.trigger_type = value; handleTypeChange(value); clearError('trigger_type') }"
                />
                <p v-if="errors.trigger_type" class="mt-2 text-sm text-red-600">{{ errors.trigger_type }}</p>
              </div>
            </div>
          </div>

          <!-- 关联工作流 -->
          <div class="mb-6">
            <h4 class="text-lg font-medium text-black mb-4">关联工作流</h4>
            <div>
              <label for="workflow_id" class="block text-sm font-medium text-black mb-2">
                选择工作流 *
              </label>
              <CustomSelect
                :model-value="form.workflow_id || ''"
                :options="workflowOptions"
                placeholder="选择要关联的工作流"
                :loading="loadingWorkflows"
                @update:modelValue="(value) => { form.workflow_id = value; clearError('workflow_id') }"
              />
              <p v-if="errors.workflow_id" class="mt-2 text-sm text-red-600">{{ errors.workflow_id }}</p>
            </div>
          </div>

          <!-- 触发器配置 -->
          <div class="mb-6">
            <h4 class="text-lg font-medium text-black mb-4">触发器配置</h4>
            
            <!-- 邮件触发配置 -->
            <div v-if="form.trigger_type === 'email'" class="space-y-4">
              <!-- 配置方式选择 -->
              <div>
                <label class="block text-sm font-medium text-black mb-3">
                  配置方式
                </label>
                <div class="flex space-x-4">
                  <label class="flex items-center">
                    <input
                      type="radio"
                      value="simple"
                      v-model="configMode"
                      class="form-radio text-primary-600"
                    />
                    <span class="ml-2 text-sm text-black">简单匹配</span>
                  </label>
                  <label class="flex items-center">
                    <input
                      type="radio"
                      value="advanced"
                      v-model="configMode"
                      class="form-radio text-primary-600"
                    />
                    <span class="ml-2 text-sm text-black">高级规则</span>
                  </label>
                </div>
              </div>

              <!-- 简单匹配配置 -->
              <div v-if="configMode === 'simple'" class="space-y-4">
                <!-- 匹配类型选择 -->
                <div>
                  <label class="block text-sm font-medium text-black mb-2">
                    匹配条件 *
                  </label>
                  <CustomSelect
                    :model-value="form.trigger_config.match_type || ''"
                    :options="matchTypeOptions"
                    placeholder="选择匹配类型"
                    required
                    @update:modelValue="(value) => { form.trigger_config.match_type = value; clearError('match_type') }"
                  />
                  <p v-if="errors.match_type" class="mt-2 text-sm text-red-600">{{ errors.match_type }}</p>
                </div>

                <!-- 匹配内容 -->
                <div>
                  <label for="match_content" class="block text-sm font-medium text-black mb-2">
                    包含内容 *
                  </label>
                  <BaseInput
                    id="match_content"
                    v-model="form.trigger_config.match_content"
                    type="text"
                    required
                    :placeholder="getMatchPlaceholder()"
                    :error-message="errors.match_content"
                    @input="clearError('match_content')"
                  />
                  <p class="mt-1 text-sm text-black">
                    {{ getMatchDescription() }}
                  </p>
                </div>
              </div>

              <!-- 高级规则配置 -->
              <div v-if="configMode === 'advanced'" class="space-y-4">
                <RuleBuilder
                  v-model="form.trigger_config.rules"
                  @update:modelValue="clearError('rules')"
                />
                <p v-if="errors.rules" class="mt-2 text-sm text-red-600">{{ errors.rules }}</p>
              </div>
            </div>

            <!-- 定时触发配置 -->
            <div v-else-if="form.trigger_type === 'schedule'" class="space-y-4">
              <div>
                <label for="cron_expression" class="block text-sm font-medium text-black mb-2">
                  Cron表达式 *
                </label>
                <BaseInput
                  id="cron_expression"
                  v-model="form.trigger_config.cron_expression"
                  type="text"
                  required
                  placeholder="例如: 0 9 * * * (每天9点执行)"
                />
                <p class="mt-1 text-sm text-black">
                  使用标准Cron表达式格式
                </p>
              </div>
              <div>
                <label for="timezone" class="block text-sm font-medium text-black mb-2">
                  时区
                </label>
                <CustomSelect
                  v-model="form.trigger_config.timezone"
                  :options="timezoneOptions"
                  placeholder="选择时区"
                />
              </div>
            </div>

            <!-- Webhook触发配置 -->
            <div v-else-if="form.trigger_type === 'webhook'" class="space-y-4">
              <div>
                <label for="webhook_url" class="block text-sm font-medium text-black mb-2">
                  Webhook URL *
                </label>
                <BaseInput
                  id="webhook_url"
                  v-model="form.trigger_config.webhook_url"
                  type="url"
                  required
                  placeholder="输入Webhook URL"
                />
              </div>
              <div>
                <label for="secret" class="block text-sm font-medium text-black mb-2">
                  密钥
                </label>
                <BaseInput
                  id="secret"
                  v-model="form.trigger_config.secret"
                  type="text"
                  placeholder="输入验证密钥（可选）"
                />
              </div>
            </div>

            <!-- 手动触发无需配置 -->
            <div v-else-if="form.trigger_type === 'manual'" class="text-center py-8">
              <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 11.5V14m0-2.5v-6a1.5 1.5 0 113 0m-3 6a1.5 1.5 0 00-3 0v2a7.5 7.5 0 0015 0v-5a1.5 1.5 0 00-3 0m-6-3V11m0-5.5v-1a1.5 1.5 0 013 0v1m0 0V11m0-5.5a1.5 1.5 0 013 0v3.5M3 16.5h12" />
              </svg>
              <h3 class="mt-2 text-sm font-medium text-black">手动触发</h3>
              <p class="mt-1 text-sm text-black">手动触发器无需额外配置，可以通过界面手动执行</p>
            </div>
          </div>

          <!-- 按钮 -->
          <div class="flex justify-end space-x-3 pt-6 border-t">
            <button
              type="button"
              @click="$emit('close')"
              class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-black bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
            >
              取消
            </button>
            <button
              type="submit"
              :disabled="submitting"
              class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50"
            >
              {{ submitting ? '保存中...' : (isEdit ? '更新' : '创建') }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import BaseInput from '@/components/BaseInput/index.vue'
import CustomSelect from '@/components/CustomSelect/index.vue'
import RuleBuilder from '@/components/RuleBuilder/index.vue'
import { triggerApi } from '@/api/trigger'
import { workflowApi } from '@/api/workflow'
import { showMessage } from '@/utils/message'

const props = defineProps({
  trigger: {
    type: Object,
    default: null
  }
})

const emit = defineEmits(['close', 'created', 'updated'])

// 响应式数据
const submitting = ref(false)
const loadingWorkflows = ref(false)
const workflows = ref([])
const errors = ref({})

const form = ref({
  name: '',
  trigger_type: '',
  workflow_id: '',  // 改为空字符串而不是null
  trigger_config: {}
})

const configMode = ref('simple') // 'simple' | 'advanced'

// 计算属性
const isEdit = computed(() => !!props.trigger)

const triggerTypeOptions = [
  { label: '邮件触发', value: 'email' },
  { label: '定时触发', value: 'schedule' },
  { label: 'Webhook触发', value: 'webhook' },
  { label: '手动触发', value: 'manual' }
]

const matchTypeOptions = [
  { label: '发件人包含', value: 'sender_contains' },
  { label: '收件人包含', value: 'recipient_contains' },
  { label: '主题包含', value: 'subject_contains' },
  { label: '内容包含', value: 'content_contains' }
]

const workflowOptions = computed(() => {
  // 确保 workflows.value 是数组
  const workflowsArray = Array.isArray(workflows.value) ? workflows.value : []
  return workflowsArray.map(workflow => ({
    label: workflow.name,
    value: workflow.id  // 使用数据库主键 id 而不是 workflow_id
  }))
})

const timezoneOptions = [
  { label: 'Asia/Shanghai', value: 'Asia/Shanghai' },
  { label: 'UTC', value: 'UTC' },
  { label: 'America/New_York', value: 'America/New_York' },
  { label: 'Europe/London', value: 'Europe/London' }
]

// 方法
const fetchWorkflows = async () => {
  try {
    loadingWorkflows.value = true
    const response = await workflowApi.getWorkflows()
    if (response.code === 0) {
      workflows.value = Array.isArray(response.data.workflows) ? response.data.workflows : []
    } else {
      workflows.value = []
    }
  } catch (error) {
    workflows.value = []
  } finally {
    loadingWorkflows.value = false
  }
}

const handleTypeChange = (type) => {
  // 重置配置
  form.value.trigger_config = {}

  // 根据类型设置默认配置
  if (type === 'schedule') {
    form.value.trigger_config.timezone = 'Asia/Shanghai'
  } else if (type === 'email') {
    // 重置邮件配置模式
    configMode.value = 'simple'
    form.value.trigger_config.match_type = 'subject_contains'
    form.value.trigger_config.match_content = ''
  }
}

const getMatchPlaceholder = () => {
  const matchType = form.value.trigger_config?.match_type
  switch (matchType) {
    case 'sender_contains':
      return '例如：admin@example.com 或 example.com'
    case 'recipient_contains':
      return '例如：user@example.com 或 example.com'
    case 'subject_contains':
      return '例如：订单确认 或 验证码'
    case 'content_contains':
      return '例如：验证码 或 重要通知'
    default:
      return '请先选择匹配类型'
  }
}

const getMatchDescription = () => {
  const matchType = form.value.trigger_config?.match_type
  switch (matchType) {
    case 'sender_contains':
      return '当邮件发件人地址包含指定内容时触发'
    case 'recipient_contains':
      return '当邮件收件人地址包含指定内容时触发'
    case 'subject_contains':
      return '当邮件主题包含指定内容时触发'
    case 'content_contains':
      return '当邮件正文内容包含指定内容时触发'
    default:
      return '请选择匹配类型'
  }
}

const validateForm = () => {
  errors.value = {}

  if (!form.value.name?.trim()) {
    errors.value.name = '请输入触发器名称'
  }

  if (!form.value.trigger_type) {
    errors.value.trigger_type = '请选择触发器类型'
  }

  if (!form.value.workflow_id) {
    errors.value.workflow_id = '请选择关联的工作流'
  }

  if (form.value.trigger_type === 'email') {
    if (configMode.value === 'simple') {
      if (!form.value.trigger_config?.match_type) {
        errors.value.match_type = '请选择匹配条件'
      }
      if (!form.value.trigger_config?.match_content?.trim()) {
        errors.value.match_content = '请输入包含内容'
      }
    } else if (configMode.value === 'advanced') {
      if (!form.value.trigger_config?.rules) {
        errors.value.rules = '请配置触发规则'
      }
    }
  }

  return Object.keys(errors.value).length === 0
}

const clearError = (field) => {
  if (errors.value[field]) {
    delete errors.value[field]
  }
}

const handleSubmit = async () => {
  // 表单验证
  if (!validateForm()) {
    return
  }

  try {
    submitting.value = true

    const data = {
      name: form.value.name,
      trigger_type: form.value.trigger_type,
      workflow_id: parseInt(form.value.workflow_id),
      trigger_config: form.value.trigger_config
    }

    if (isEdit.value) {
      const response = await triggerApi.updateTrigger(props.trigger.id, data)
      if (response.code === 0) {
        showMessage('触发器更新成功', 'success')
        emit('updated')
      }
    } else {
      const response = await triggerApi.createTrigger(data)
      if (response.code === 0) {
        showMessage('触发器创建成功', 'success')
        emit('created')
      }
    }
  } catch (error) {
    // 处理后端验证错误
    if (error.response?.data?.detail) {
      const detail = error.response.data.detail
      if (Array.isArray(detail)) {
        // Pydantic 验证错误
        detail.forEach(err => {
          const field = err.loc[err.loc.length - 1]
          errors.value[field] = err.msg
        })
      } else {
        showMessage(detail, 'error')
      }
    } else {
      showMessage('保存触发器失败', 'error')
    }
  } finally {
    submitting.value = false
  }
}

// 监听器
watch(() => props.trigger, (trigger) => {
  if (trigger) {
    form.value = {
      name: trigger.name || '',
      trigger_type: trigger.trigger_type || '',
      workflow_id: trigger.workflow_id || '',  // 确保不是null
      trigger_config: trigger.trigger_config || {}
    }
    
    // 根据已有配置确定配置模式
    if (trigger.trigger_type === 'email') {
      if (trigger.trigger_config?.rules) {
        configMode.value = 'advanced'
      } else {
        configMode.value = 'simple'
      }
    }
  }
}, { immediate: true })

// 生命周期
onMounted(() => {
  fetchWorkflows()
})
</script>
