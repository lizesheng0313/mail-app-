<template>
  <div v-if="visible" class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
    <div class="bg-white rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[80vh] flex flex-col">
      <!-- 标题 -->
      <div class="px-6 py-4 border-b">
        <h3 class="text-lg font-semibold">批量添加邮箱账号</h3>
      </div>

      <!-- 内容 -->
      <div class="px-6 py-4 flex-1 overflow-y-auto">
        <!-- 邮箱服务商选择 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-2">邮箱服务商</label>
          <CustomSelect
            v-model="selectedProvider"
            :options="providerOptions"
            placeholder="请选择邮箱服务商"
          />
          <p class="text-xs text-gray-500 mt-1">
            {{ selectedProvider === 'custom' ? '请在下方填写自定义POP3服务器配置' : selectedProvider ? `将为所有账号使用 ${selectedProvider} 的配置` : '系统会根据邮箱后缀自动识别服务商' }}
          </p>
        </div>

        <!-- 自定义POP3配置 -->
        <div v-if="selectedProvider === 'custom'" class="mb-4 p-4 bg-gray-50 rounded-lg">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">POP3服务器地址</label>
              <input
                v-model="customPop3.host"
                type="text"
                placeholder="例如: pop.example.com"
                class="w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">端口号</label>
              <input
                v-model="customPop3.port"
                type="number"
                placeholder="例如: 995"
                class="w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
              />
            </div>
          </div>
        </div>

        <div class="mb-4">
          <p class="text-sm text-gray-600 mb-2">批量导入账号</p>
          <p class="text-xs text-gray-500">每行一个账号，格式：<strong>邮箱,授权码</strong> 或 <strong>用户名,授权码</strong>（使用上方选择的服务商）</p>
          <p class="text-xs text-orange-600 mt-1">⚠️ 注意：请输入授权码，不是登录密码！授权码需要在邮箱设置中开启POP3服务后获取。</p>
        </div>
        
        <textarea
          v-model="accountsText"
          :placeholder="selectedProvider 
            ? `user1,授权码1\nuser2,授权码2\n（将自动添加 @${selectedProvider}）` 
            : `user1@example.com,授权码1\nuser2@example.com,授权码2`"
          class="w-full h-64 px-3 py-2 border rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
        />

        <div v-if="parsedAccounts.length > 0" class="mt-4">
          <p class="text-sm font-medium mb-2">将添加 {{ parsedAccounts.length }} 个账号：</p>
          <div class="max-h-32 overflow-y-auto bg-gray-50 rounded p-2">
            <div v-for="(acc, idx) in parsedAccounts" :key="idx" class="text-xs text-gray-600">
              {{ idx + 1 }}. {{ acc.email }}
            </div>
          </div>
        </div>

        <div v-if="errors.length > 0" class="mt-4">
          <p class="text-sm font-medium text-red-600 mb-2">格式错误：</p>
          <div class="max-h-24 overflow-y-auto bg-red-50 rounded p-2">
            <div v-for="(error, idx) in errors" :key="idx" class="text-xs text-red-600">
              {{ error }}
            </div>
          </div>
        </div>
      </div>

      <!-- 按钮 -->
      <div class="px-6 py-4 border-t flex justify-end gap-3">
        <button
          @click="$emit('close')"
          class="px-4 py-2 text-sm text-gray-700 bg-gray-100 hover:bg-gray-200 rounded"
        >
          取消
        </button>
        <button
          @click="handleSubmit"
          :disabled="parsedAccounts.length === 0 || errors.length > 0"
          class="px-4 py-2 text-sm text-white bg-primary-600 hover:bg-primary-700 rounded disabled:opacity-50 disabled:cursor-not-allowed"
        >
          添加 {{ parsedAccounts.length }} 个账号
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import CustomSelect from '@/components/CustomSelect/index.vue'

interface Props {
  visible: boolean
}

const props = defineProps<Props>()
const emit = defineEmits(['close', 'submit'])

const accountsText = ref('')
const selectedProvider = ref('')
const customPop3 = ref({
  host: '',
  port: 995
})
const errors = ref<string[]>([])

const providerOptions = [
  { label: '自动识别（根据邮箱后缀）', value: '' },
  { label: 'QQ邮箱 (@qq.com) - pop.qq.com:995', value: 'qq.com' },
  { label: '网易163 (@163.com) - pop.163.com:995', value: '163.com' },
  { label: '网易126 (@126.com) - pop.126.com:995', value: '126.com' },
  { label: 'Gmail (@gmail.com) - pop.gmail.com:995', value: 'gmail.com' },
  { label: 'Outlook (@outlook.com) - outlook.office365.com:995', value: 'outlook.com' },
  { label: 'Hotmail (@hotmail.com) - outlook.office365.com:995', value: 'hotmail.com' },
  { label: 'Yahoo (@yahoo.com) - pop.mail.yahoo.com:995', value: 'yahoo.com' },
  { label: '自定义POP3服务器', value: 'custom' }
]

const parsedAccounts = computed(() => {
  const lines = accountsText.value.split('\n').filter(line => line.trim())
  const accounts: { email: string; password: string }[] = []
  errors.value = []

  lines.forEach((line, idx) => {
    const parts = line.split(',').map(p => p.trim())
    if (parts.length !== 2) {
      errors.value.push(`行 ${idx + 1}: 格式错误，应为"邮箱,授权码" 或 "用户名,授权码"`)
      return
    }
    
    let [email, password] = parts
    if (!email || !password) {
      errors.value.push(`行 ${idx + 1}: 邮箱或授权码为空`)
      return
    }
    
    // 如果选择了服务商且邮箱不包含@，自动添加后缀
    if (selectedProvider.value && !email.includes('@')) {
      email = `${email}@${selectedProvider.value}`
    }
    
    // 验证邮箱格式
    if (!email.includes('@')) {
      errors.value.push(`行 ${idx + 1}: 邮箱格式错误（缺少@），请选择邮箱服务商或使用完整邮箱地址`)
      return
    }

    accounts.push({ email, password })
  })

  return accounts
})

const handleSubmit = () => {
  if (parsedAccounts.value.length > 0 && errors.value.length === 0) {
    // 如果是自定义POP3，需要验证配置
    if (selectedProvider.value === 'custom') {
      if (!customPop3.value.host || !customPop3.value.port) {
        errors.value.push('请填写完整的POP3服务器配置')
        return
      }
      // 添加自定义POP3信息到每个账号
      const accountsWithCustomPop3 = parsedAccounts.value.map(acc => ({
        ...acc,
        pop3_host: customPop3.value.host,
        pop3_port: customPop3.value.port
      }))
      emit('submit', accountsWithCustomPop3)
    } else {
      emit('submit', parsedAccounts.value)
    }
    accountsText.value = ''
  }
}

watch(() => props.visible, (visible) => {
  if (!visible) {
    accountsText.value = ''
    selectedProvider.value = ''
    customPop3.value = { host: '', port: 995 }
    errors.value = []
  }
})
</script>
