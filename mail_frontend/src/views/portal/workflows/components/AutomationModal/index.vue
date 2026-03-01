<template>
  <div class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
    <div class="relative top-10 mx-auto p-0 border max-w-3xl shadow-lg rounded-lg bg-white">
      <!-- 头部 -->
      <div class="flex items-center justify-between p-6 border-b">
        <h3 class="text-xl font-semibold text-black">
          快速自动化
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
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label for="name" class="block text-sm font-medium text-black mb-2">
                  自动化名称 *
                </label>
                <BaseInput
                  id="name"
                  v-model="form.name"
                  type="text"
                  required
                  placeholder="输入自动化名称"
                />
              </div>
              <div>
                <label for="type" class="block text-sm font-medium text-black mb-2">
                  自动化类型 *
                </label>
                <CustomSelect
                  v-model="form.type"
                  :options="automationTypeOptions"
                  placeholder="选择自动化类型"
                  @update:modelValue="handleTypeChange"
                />
              </div>
            </div>
            <div class="mt-4">
              <label for="description" class="block text-sm font-medium text-black mb-2">
                描述
              </label>
              <BaseTextarea
                id="description"
                v-model="form.description"
                rows="3"
                placeholder="输入自动化描述"
              />
            </div>
          </div>

          <!-- 类型说明 -->
          <div v-if="selectedTypeInfo" class="mb-6 p-4 bg-primary-50 rounded-lg">
            <h4 class="text-sm font-medium text-blue-900 mb-2">{{ selectedTypeInfo.name }}</h4>
            <p class="text-sm text-primary-700">{{ selectedTypeInfo.description }}</p>
          </div>

          <!-- 邮件处理配置 -->
          <div v-if="form.type === 'email_processing'" class="mb-6">
            <h4 class="text-lg font-medium text-black mb-4">邮件处理配置</h4>
            
            <!-- 获取邮件配置 -->
            <div class="mb-4">
              <h5 class="text-sm font-medium text-black mb-3">获取邮件设置</h5>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div>
                  <label class="block text-sm font-medium text-black mb-1">
                    邮箱文件夹
                  </label>
                  <BaseInput
                    v-model="form.config.fetch_params.folder"
                    type="text"
                    size="sm"
                    placeholder="INBOX"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-black mb-1">
                    获取数量
                  </label>
                  <BaseInput
                    v-model="form.config.fetch_params.limit"
                    type="number"
                    min="1"
                    max="100"
                    size="sm"
                    placeholder="10"
                  />
                </div>
                <div class="flex items-center">
                  <input
                    id="unread_only"
                    v-model="form.config.fetch_params.unread_only"
                    type="checkbox"
                    class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                  >
                  <label for="unread_only" class="ml-2 block text-sm text-black">
                    仅未读邮件
                  </label>
                </div>
              </div>
            </div>

            <!-- 处理规则配置 -->
            <div class="mb-4">
              <h5 class="text-sm font-medium text-black mb-3">处理规则</h5>
              <div class="space-y-3">
                <div
                  v-for="(rule, index) in form.config.process_params.rules"
                  :key="index"
                  class="flex items-center space-x-3 p-3 border border-gray-200 rounded-md"
                >
                  <input
                    v-model="rule.condition"
                    type="text"
                    placeholder="条件 (如: subject contains '重要')"
                    class="flex-1 px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500 text-sm"
                  >
                  <input
                    v-model="rule.action"
                    type="text"
                    placeholder="动作 (如: move_to_folder('重要')"
                    class="flex-1 px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500 text-sm"
                  >
                  <button
                    type="button"
                    @click="removeRule(index)"
                    class="text-red-600 hover:text-red-800"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                </div>
                <button
                  type="button"
                  @click="addRule"
                  class="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-black bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                >
                  <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                  </svg>
                  添加规则
                </button>
              </div>
            </div>
          </div>

          <!-- 网页爬取配置 -->
          <div v-if="form.type === 'web_crawling'" class="mb-6">
            <h4 class="text-lg font-medium text-black mb-4">网页爬取配置</h4>
            
            <!-- 爬取设置 -->
            <div class="mb-4">
              <h5 class="text-sm font-medium text-black mb-3">爬取设置</h5>
              <div class="grid grid-cols-1 gap-4">
                <div>
                  <label class="block text-sm font-medium text-black mb-1">
                    目标URL列表 (每行一个)
                  </label>
                  <textarea
                    v-model="urlsText"
                    rows="4"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500 text-sm"
                    placeholder="https://example.com&#10;https://example2.com"
                  ></textarea>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <label class="block text-sm font-medium text-black mb-1">
                      超时时间(秒)
                    </label>
                    <input
                      v-model.number="form.config.crawl_params.timeout"
                      type="number"
                      min="1"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500 text-sm"
                      placeholder="30"
                    >
                  </div>
                  <div class="flex items-center">
                    <input
                      id="use_proxy"
                      v-model="form.config.use_proxy"
                      type="checkbox"
                      class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                    >
                    <label for="use_proxy" class="ml-2 block text-sm text-black">
                      使用代理
                    </label>
                  </div>
                  <div v-if="form.config.use_proxy">
                    <label class="block text-sm font-medium text-black mb-1">
                      代理池
                    </label>
                    <CustomSelect
                      v-model="form.config.proxy_pool_id"
                      :options="proxyPoolOptions"
                      placeholder="选择代理池"
                    />
                  </div>
                </div>
              </div>
            </div>

            <!-- 数据处理设置 -->
            <div class="mb-4">
              <h5 class="text-sm font-medium text-black mb-3">数据处理设置</h5>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-black mb-1">
                    输出格式
                  </label>
                  <CustomSelect
                    v-model="form.config.process_params.output_format"
                    :options="outputFormatOptions"
                    placeholder="选择输出格式"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-black mb-1">
                    提取规则 (JSON格式)
                  </label>
                  <textarea
                    v-model="extractRulesText"
                    rows="3"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500 text-sm font-mono"
                    placeholder='[{"selector": "h1", "field": "title"}]'
                  ></textarea>
                </div>
              </div>
            </div>
          </div>

          <!-- 文件处理配置 -->
          <div v-if="form.type === 'file_processing'" class="mb-6">
            <h4 class="text-lg font-medium text-black mb-4">文件处理配置</h4>
            
            <div class="grid grid-cols-1 gap-4">
              <div>
                <label class="block text-sm font-medium text-black mb-1">
                  文件路径列表 (每行一个)
                </label>
                <textarea
                  v-model="filePathsText"
                  rows="4"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500 text-sm"
                  placeholder="/path/to/file1.txt&#10;/path/to/file2.txt"
                ></textarea>
              </div>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-black mb-1">
                    文件类型过滤
                  </label>
                  <input
                    v-model="fileTypesText"
                    type="text"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500 text-sm"
                    placeholder="txt,pdf,doc (用逗号分隔)"
                  >
                </div>
                <div>
                  <label class="block text-sm font-medium text-black mb-1">
                    输出格式
                  </label>
                  <CustomSelect
                    v-model="form.config.process_params.output_format"
                    :options="outputFormatOptions"
                    placeholder="选择输出格式"
                  />
                </div>
              </div>
            </div>
          </div>

          <!-- 操作按钮 -->
          <div class="flex justify-end space-x-3 pt-4 border-t">
            <button
              type="button"
              @click="$emit('close')"
              class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-black hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
            >
              取消
            </button>
            <button
              type="submit"
              :disabled="loading"
              class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {{ loading ? '创建中...' : '创建自动化' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import BaseInput from '@/components/BaseInput/index.vue'
import BaseTextarea from '@/components/BaseTextarea/index.vue'
import CustomSelect from '@/components/CustomSelect/index.vue'
import { workflowApi } from '@/api/workflow'
import { proxyApi } from '@/api/proxy'
import { showMessage } from '@/utils/message'

const emit = defineEmits(['close', 'created'])

// 响应式数据
const loading = ref(false)
const automationTypes = ref({})
const proxyPools = ref([])
const form = ref({
  name: '',
  description: '',
  type: '',
  config: {
    fetch_params: {
      folder: 'INBOX',
      limit: 10,
      unread_only: true
    },
    process_params: {
      rules: [],
      output_format: 'json'
    },
    crawl_params: {
      urls: [],
      headers: {},
      timeout: 30
    },
    use_proxy: false,
    proxy_pool_id: null,
    proxy_strategy: {
      ip_reuse_policy: 'time_based',
      reuse_interval_hours: 72,
      min_success_rate: 80.0
    }
  }
})

// 辅助文本字段
const urlsText = ref('')
const extractRulesText = ref('[]')
const filePathsText = ref('')
const fileTypesText = ref('')

// 计算属性
const automationTypeOptions = computed(() => {
  return Object.keys(automationTypes.value).map(key => ({
    label: automationTypes.value[key].name,
    value: key
  }))
})

const selectedTypeInfo = computed(() => {
  return form.value.type ? automationTypes.value[form.value.type] : null
})

const outputFormatOptions = [
  { label: 'JSON', value: 'json' },
  { label: 'CSV', value: 'csv' },
  { label: 'XML', value: 'xml' },
  { label: 'TXT', value: 'txt' }
]

const proxyPoolOptions = computed(() => {
  return proxyPools.value.map(pool => ({
    label: `${pool.pool_name} (${pool.pool_type.toUpperCase()})`,
    value: pool.id
  }))
})

const reuseOptions = [
  { label: '永不重复', value: 'no_reuse' },
  { label: '基于时间间隔', value: 'time_based' },
  { label: '基于使用次数', value: 'count_based' }
]

// 方法
const fetchAutomationTypes = async () => {
  try {
    const response = await workflowApi.getAutomationTypes()
    automationTypes.value = response.data || {}
  } catch (error) {
    showMessage('获取自动化类型失败', 'error')
  }
}

const fetchProxyPools = async () => {
  try {
    const response = await proxyApi.getProxyPools()
    proxyPools.value = response.data?.pools || []
  } catch (error) {
    // 静默处理代理池获取失败
  }
}

const handleTypeChange = (type) => {
  // 重置配置
  if (type === 'email_processing') {
    form.value.config.process_params.rules = []
  } else if (type === 'web_crawling') {
    urlsText.value = ''
    extractRulesText.value = '[]'
  } else if (type === 'file_processing') {
    filePathsText.value = ''
    fileTypesText.value = ''
  }
}

const addRule = () => {
  form.value.config.process_params.rules.push({
    condition: '',
    action: ''
  })
}

const removeRule = (index) => {
  form.value.config.process_params.rules.splice(index, 1)
}

const handleSubmit = async () => {
  try {
    loading.value = true

    // 处理不同类型的配置
    const config = { ...form.value.config }

    if (form.value.type === 'web_crawling') {
      // 处理URL列表
      config.crawl_params.urls = urlsText.value
        .split('\n')
        .map(url => url.trim())
        .filter(url => url)

      // 处理提取规则
      try {
        config.process_params.extract_rules = JSON.parse(extractRulesText.value || '[]')
      } catch (e) {
        throw new Error('提取规则格式不正确')
      }
    } else if (form.value.type === 'file_processing') {
      // 处理文件路径列表
      config.read_params = {
        file_paths: filePathsText.value
          .split('\n')
          .map(path => path.trim())
          .filter(path => path),
        file_types: fileTypesText.value
          .split(',')
          .map(type => type.trim())
          .filter(type => type)
      }
    }

    const data = {
      name: form.value.name,
      description: form.value.description,
      type: form.value.type,
      config
    }

    await workflowApi.createAutomation(data)
    showMessage('自动化工作流创建成功', 'success')
    emit('created')

  } catch (error) {
    showMessage(error.message || '创建自动化失败', 'error')
  } finally {
    loading.value = false
  }
}

// 生命周期
onMounted(() => {
  fetchAutomationTypes()
  fetchProxyPools()
})
</script>
