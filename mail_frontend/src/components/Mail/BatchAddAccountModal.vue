<template>
  <Teleport to="body">
    <div v-if="visible" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div class="bg-white rounded-lg shadow-2xl w-full max-w-6xl" style="max-height: 90vh;">
        <!-- 标题栏 -->
        <div class="px-6 py-4 border-b border-gray-200 flex items-center justify-between">
          <h3 class="text-lg font-semibold text-gray-800">批量添加第三方邮箱</h3>
          <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600 transition">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
        </div>

        <div class="p-6">
          <div class="grid grid-cols-5 gap-4">
            <!-- 左侧：输入框 (3/5) -->
            <div class="col-span-3">
              <div class="text-sm text-gray-600 mb-2 h-6 flex items-center">
                输入账号（每行一个）
              </div>
              <textarea
                v-model="accountsText"
                placeholder="每行一个，支持格式：&#10;邮箱----授权码&#10;邮箱 授权码&#10;邮箱----任意----授权码&#10;&#10;示例：&#10;user@163.com----abc123&#10;user@qq.com pwd456"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 font-mono resize-none"
                style="height: 360px; line-height: 20px; font-size: 14px;"
              ></textarea>
            </div>
            
            <!-- 右侧：结果状态 (2/5) -->
            <div class="col-span-2">
              <div class="text-sm text-gray-600 mb-2 h-6 flex items-center">
                添加结果
              </div>
              <div class="border border-gray-200 rounded-lg p-2 bg-gray-50 overflow-y-auto" style="height: 360px;">
                <div v-if="results.length === 0" class="text-sm text-gray-400 text-center py-20">
                  添加结果将显示在这里
                </div>
                <div v-else class="space-y-0">
                  <div v-for="(result, idx) in results" :key="idx" 
                       class="flex items-center gap-2 px-2 rounded"
                       :class="result.status === 'error' ? 'bg-red-50' : result.status === 'success' ? 'bg-green-50' : ''"
                       style="height: 20px; line-height: 20px; font-size: 14px;">
                    <span v-if="result.status === 'success'" class="text-green-600 font-bold w-3">✓</span>
                    <span v-else-if="result.status === 'error'" class="text-red-600 font-bold w-3">✗</span>
                    <span v-else class="text-gray-400 w-3">⋯</span>
                    <div class="flex-1 min-w-0 font-mono truncate" :class="result.status === 'error' ? 'text-red-700' : 'text-gray-700'">
                      {{ result.email }}
                    </div>
                    <div v-if="result.message" class="text-gray-500 flex-shrink-0" :title="result.message">
                      {{ getShortMessage(result.message) }}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 协议选择和自定义服务器 -->
          <div class="mt-4 pt-4 border-t border-gray-200">
            <div class="flex items-center gap-6">
              <!-- 协议选择 -->
              <div class="flex items-center gap-3">
                <span class="text-sm text-gray-700 font-medium">协议：</span>
                <label class="flex items-center text-sm cursor-pointer hover:text-primary-600 transition">
                  <input type="radio" v-model="protocol" value="auto" class="mr-1.5 w-4 h-4 text-primary-600">
                  自动
                </label>
                <label class="flex items-center text-sm cursor-pointer hover:text-primary-600 transition">
                  <input type="radio" v-model="protocol" value="imap" class="mr-1.5 w-4 h-4 text-primary-600">
                  IMAP
                </label>
                <label class="flex items-center text-sm cursor-pointer hover:text-primary-600 transition">
                  <input type="radio" v-model="protocol" value="pop3" class="mr-1.5 w-4 h-4 text-primary-600">
                  POP3
                </label>
              </div>
              
              <!-- 自定义服务器开关 -->
              <label class="flex items-center text-sm text-gray-700 cursor-pointer hover:text-primary-600 transition select-none">
                <input type="checkbox" v-model="showCustomServer" class="mr-2 w-4 h-4 rounded text-primary-600">
                自定义服务器
              </label>
            </div>
            
            <!-- 自定义服务器输入 -->
            <div v-if="showCustomServer" class="mt-3 flex items-center gap-3">
              <span class="text-sm text-gray-600 w-16">服务器：</span>
              <input
                v-model="customHost"
                type="text"
                :placeholder="protocol === 'imap' ? 'imap.example.com' : 'pop.example.com'"
                class="flex-1 px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-primary-500 focus:border-transparent"
              />
              <span class="text-sm text-gray-600">端口：</span>
              <input
                v-model.number="customPort"
                type="number"
                :placeholder="protocol === 'imap' ? '993' : '995'"
                class="w-24 px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-primary-500 focus:border-transparent"
              />
            </div>
          </div>
        </div>
        
        <!-- 底部按钮 -->
        <div class="px-6 py-4 border-t border-gray-200 flex justify-end gap-3 bg-gray-50">
          <button
            @click="$emit('close')"
            class="px-5 py-2 text-sm text-gray-700 bg-white border border-gray-300 hover:bg-gray-50 rounded-lg transition"
          >
            取消
          </button>
          <button
            @click="handleSubmit"
            :disabled="!accountsText.trim()"
            class="px-5 py-2 text-sm text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition disabled:opacity-50 disabled:cursor-not-allowed"
          >
            开始添加
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{ visible: boolean }>()
const emit = defineEmits(['close', 'submit'])

const accountsText = ref('')
const protocol = ref('auto')
const showCustomServer = ref(false)
const customHost = ref('')
const customPort = ref(995)
const results = ref<Array<{ email: string, status: 'pending' | 'success' | 'error', message?: string }>>([])

// 监听弹窗关闭，清空结果
watch(() => props.visible, (visible) => {
  if (!visible) {
    results.value = []
  }
})

// 暴露方法给父组件更新结果
defineExpose({
  updateResult: (email: string, status: 'success' | 'error', message?: string) => {
    const index = results.value.findIndex(r => r.email === email)
    if (index !== -1) {
      results.value[index] = { email, status, message }
    }
  },
  initResults: (emails: string[]) => {
    results.value = emails.map(email => ({ email, status: 'pending' as const }))
  },
  clearResults: () => {
    results.value = []
  }
})

const parseAccounts = () => {
  const lines = accountsText.value.trim().split('\n')
  const accounts: any[] = []

  for (const line of lines) {
    if (!line.trim()) continue

    let processedLine = line.trim()

    // 去掉 "卡号：" 或 "卡号:" 前缀
    processedLine = processedLine.replace(/^卡号[：:]\s*/i, '')

    // 支持：空格、----、——（中文破折号）、—、Tab
    const parts = processedLine.split(/[\s]+|[-]{2,}|[—–]+/).filter(p => p.trim())

    if (parts.length >= 2) {
      // 如果有3列或更多，第3列是授权码；否则第2列是授权码
      const email = parts[0].trim()
      const password = parts.length >= 3 ? parts[2].trim() : parts[1].trim()

      const account: any = {
        email,
        password,
        protocol: protocol.value
      }

      // 自定义服务器
      if (showCustomServer.value && customHost.value) {
        if (protocol.value === 'imap') {
          account.imap_host = customHost.value
          account.imap_port = customPort.value || 993
        } else {
          account.pop3_host = customHost.value
          account.pop3_port = customPort.value || 995
        }
      }

      accounts.push(account)
    }
  }

  return accounts
}

// 缩短错误信息
const getShortMessage = (msg: string) => {
  if (!msg) return ''
  if (msg.includes('无法连接') || msg.includes('连接失败') || msg.includes('Connection')) return '连接失败'
  if (msg.includes('授权') || msg.includes('auth') || msg.includes('password')) return '授权失败'
  if (msg.includes('超时') || msg.includes('timeout')) return '超时'
  if (msg.length > 10) return msg.substring(0, 10) + '...'
  return msg
}

const handleSubmit = () => {
  const accounts = parseAccounts()
  if (accounts.length > 0) {
    emit('submit', accounts)
    // 不清空输入框，保留用户输入
  }
}
</script>
