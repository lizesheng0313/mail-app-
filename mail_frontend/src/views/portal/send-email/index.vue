<template>
  <div class="min-h-screen bg-gray-50">
    <!-- 公共头部 -->
    <PageHeader />

    <main class="max-w-5xl mx-auto px-4 py-6">
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div class="lg:col-span-1 h-[calc(100vh-120px)] bg-white rounded-lg shadow flex flex-col overflow-hidden p-4">
          <MailboxList
            title="发件账号"
            :mailboxes="smtpAccounts"
            :selectedId="selectedAccountId"
            :showPagination="false"
            @select="handleSelectAccount"
          >
            <!-- 隐藏默认的批量操作按钮 -->
            <template #header-actions>
              <span></span>
            </template>

            <template #content="{ mailboxes, selectedId, onSelect }">
              <div
                v-for="account in mailboxes"
                :key="account.id"
                class="group p-3 bg-gray-50 rounded-lg hover:bg-primary-100 cursor-pointer mb-2"
                :class="{ 'bg-primary-100': selectedId === account.id }"
                @click="onSelect(account)"
              >
                <div class="flex items-center justify-between">
                  <div class="flex items-center flex-1 min-w-0">
                    <div class="flex-1">
                      <div class="flex items-center gap-2">
                        <span 
                          v-if="account.status === 'active'"
                          class="w-2 h-2 rounded-full flex-shrink-0 bg-green-500"
                          title="🟢 可用"
                        ></span>
                        <span 
                          v-else
                          class="w-2 h-2 rounded-full flex-shrink-0 bg-red-500"
                          :title="'🔴 不可用' + (account.last_error ? ': ' + account.last_error : '')"
                        ></span>
                        <code class="text-sm truncate font-medium" :class="account.status === 'failed' ? 'text-red-600' : 'text-gray-900'">
                          {{ account.email }}
                        </code>
                      </div>
                      <p class="text-xs mt-1 text-gray-500">
                        <span v-if="account.status === 'failed' && account.last_error" class="text-red-500">
                          ❌ {{ account.last_error }}
                        </span>
                        <span v-else>
                          已发：{{ account.send_count || 0 }} 封
                        </span>
                      </p>
                    </div>
                  </div>
                  <div class="ml-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200 flex gap-1 flex-shrink-0">
                    <ActionButton
                      icon="copy"
                      variant="copy"
                      tooltip="复制邮箱"
                      @click.stop="copy(account.email)"
                    />
                    <ActionButton
                      icon="delete"
                      variant="delete"
                      tooltip="删除账号"
                      @click.stop="deleteAccount(account)"
                    />
                  </div>
                </div>
              </div>
            </template>
          </MailboxList>
        </div>

        <!-- 右侧：写邮件 -->
        <div class="lg:col-span-2">
          <div class="bg-white rounded-lg shadow p-6">
            <!-- 发件人显示 -->
            <div class="mb-4 pb-4 border-b border-gray-100">
              <div v-if="selectedAccount" class="flex items-center gap-2 text-sm">
                <span class="text-gray-500">发件人:</span>
                <span class="px-2 py-1 bg-primary-50 text-primary-700 rounded font-medium">
                  {{ selectedAccount.email }}
                </span>
                <span v-if="selectedAccount.id === null" class="text-xs text-red-500 bg-red-50 px-2 py-1 rounded">
                  ⚠️ 该邮箱未配置发送服务，请重新添加
                </span>
              </div>
              <div v-else class="text-sm text-amber-600">
                ← 请先从左侧选择发件账号
              </div>
            </div>

            <div class="space-y-4">
              <!-- 收件人 -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">
                  收件人 <span class="text-red-500">*</span>
                </label>
                <input
                  v-model="form.to"
                  type="text"
                  placeholder="多个收件人用逗号分隔"
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                />
              </div>

              <!-- 抄送/密送（折叠） -->
              <div v-if="showCcBcc" class="grid grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">抄送</label>
                  <input
                    v-model="form.cc"
                    type="text"
                    placeholder="CC"
                    class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">密送</label>
                  <input
                    v-model="form.bcc"
                    type="text"
                    placeholder="BCC"
                    class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                  />
                </div>
              </div>
              <button 
                v-else
                @click="showCcBcc = true" 
                class="text-sm text-gray-500 hover:text-gray-700"
              >
                + 添加抄送/密送
              </button>

              <!-- 主题 -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">
                  主题 <span class="text-red-500">*</span>
                </label>
                <input
                  v-model="form.subject"
                  type="text"
                  placeholder="请输入邮件主题"
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                />
              </div>

              <!-- 正文 -->
              <div>
                <div class="flex items-center justify-between mb-1">
                  <label class="block text-sm font-medium text-gray-700">
                    正文 <span class="text-red-500">*</span>
                  </label>
                  <!-- AI 润色按钮 -->
                  <button
                    @click="polishContent"
                    :disabled="polishing || !form.content.trim()"
                    :class="[
                      'px-3 py-1 text-xs rounded-full flex items-center gap-1 transition-colors',
                      form.content.trim() && !polishing
                        ? 'bg-gradient-to-r from-purple-500 to-pink-500 text-white hover:from-purple-600 hover:to-pink-600'
                        : 'bg-gray-200 text-gray-400 cursor-not-allowed'
                    ]"
                  >
                    {{ polishing ? 'AI润色中...' : 'AI润色' }}
                  </button>
                </div>
                <textarea
                  v-model="form.content"
                  rows="12"
                  placeholder="请输入邮件正文..."
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 resize-none"
                ></textarea>
              </div>

              <!-- 发送按钮 -->
              <div class="flex justify-end">
                <button
                  @click="sendEmail"
                  :disabled="sending || !canSend"
                  :class="[
                    'px-8 py-2.5 rounded-lg font-medium transition-colors flex items-center gap-2',
                    canSend && !sending
                      ? 'bg-primary-600 text-white hover:bg-primary-700'
                      : 'bg-gray-300 text-gray-500 cursor-not-allowed'
                  ]"
                >
                  <svg v-if="sending" class="w-4 h-4 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                  </svg>
                  <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"/>
                  </svg>
                  {{ sending ? '发送中...' : '发送邮件' }}
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- 添加 SMTP 账号弹窗 -->
    <BaseModal
      v-model="showAddSmtpModal"
      title="添加发件账号"
      size="md"
      :confirmLoading="addingSmtp"
      confirmText="添加"
      @confirm="addSmtpAccount"
    >
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            邮箱地址 <span class="text-red-500">*</span>
          </label>
          <input
            v-model="smtpForm.email"
            type="email"
            placeholder="example@qq.com"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
          />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            SMTP 授权码 <span class="text-red-500">*</span>
          </label>
          <input
            v-model="smtpForm.password"
            type="password"
            placeholder="邮箱授权码（非登录密码）"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
          />
        </div>

        <div class="bg-blue-50 border border-blue-200 rounded-lg p-3">
          <p class="text-sm text-blue-700">
            💡 支持 QQ、163、126、Gmail、Outlook 等常见邮箱，自动识别服务器配置
          </p>
        </div>
      </div>
    </BaseModal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import PageHeader from '@/components/PageHeader/index.vue'
import MailboxList from '@/components/Mail/MailboxList/MailboxList.vue'
import ActionButton from '@/components/ActionButton/index.vue'
import BaseModal from '@/components/BaseModal/index.vue'
import smtpAccountsAPI from '@/api/smtpAccounts'
import { showMessage } from '@/utils/message'
import api from '@/services/api'

interface SmtpAccount {
  id: number
  email: string
  provider: string
  status: string
  send_count: number
}

// 状态
const loading = ref(false)
const smtpAccounts = ref<SmtpAccount[]>([])
const selectedAccountId = ref<number | null>(null)
const selectedAccount = ref<SmtpAccount | { email: string, id: null, is_virtual: boolean } | null>(null)
const sending = ref(false)
const showCcBcc = ref(false)
const polishing = ref(false)


// 添加 SMTP 弹窗
const showAddSmtpModal = ref(false)
const addingSmtp = ref(false)
const smtpForm = ref({
  email: '',
  password: ''
})

// 邮件表单
const form = ref({
  to: '',
  cc: '',
  bcc: '',
  subject: '',
  content: ''
})

// 计算属性
const canSend = computed(() => {
  return selectedAccount.value && 
         selectedAccount.value.id !== null && // 确保是已绑定的发件账号
         form.value.to.trim() && 
         form.value.subject.trim() && 
         form.value.content.trim()
})

// 监听 SMTP 账号列表加载，加载完后尝试自动匹配
const loadSmtpAccounts = async () => {
  loading.value = true
  try {
    const res = await smtpAccountsAPI.getAccounts()
    if (res.code === 0) {
      smtpAccounts.value = res.data.accounts || []
    }
  } catch (e) {
    console.error('获取SMTP账号失败', e)
  } finally {
    loading.value = false
  }
}

// 处理从列表选择邮箱
const handleSelectAccount = (account: any) => {
  selectedAccountId.value = account.id
  selectedAccount.value = account
}

const copy = (text: string) => {
  navigator.clipboard.writeText(text)
  showMessage('已复制', 'success')
}

// 删除账号
const deleteAccount = async (account: any) => {
  if (!confirm(`确定要删除 ${account.email} 吗？`)) return
  
  try {
    const response = await smtpAccountsAPI.deleteAccount(account.id)
    if (response.code === 0) {
      showMessage('删除成功', 'success')
      await loadSmtpAccounts() // 重新加载 SMTP 账号列表
      // 如果删除的是当前选中的账号，则清空选中
      if (selectedAccountId.value === account.id) {
        selectedAccountId.value = null
        selectedAccount.value = null
      }
    }
  } catch (error: any) {
    showMessage('删除失败', 'error')
  }
}



// AI 润色
const polishContent = async () => {
  if (!form.value.content.trim() || polishing.value) return
  
  polishing.value = true
  try {
    const response: any = await api.post('/ai/polish-email', {
      content: form.value.content,
      subject: form.value.subject || null
    })
    
    if (response.code === 0 && response.data?.content) {
      form.value.content = response.data.content
      if (response.data.subject) {
        form.value.subject = response.data.subject
      }
      showMessage('✨ AI 润色完成', 'success')
    } else {
      showMessage(response.message || 'AI 润色失败', 'error')
    }
  } catch (error: any) {
    console.error('AI 润色失败:', error)
    showMessage('AI 润色失败，请稍后重试', 'error')
  } finally {
    polishing.value = false
  }
}

// 添加 SMTP 账号
const addSmtpAccount = async () => {
  if (!smtpForm.value.email || !smtpForm.value.password) {
    showMessage('请填写邮箱地址和授权码', 'error')
    return
  }

  addingSmtp.value = true
  try {
    const response = await smtpAccountsAPI.addAccount({
      email: smtpForm.value.email,
      password: smtpForm.value.password
    })
    
    if (response.code === 0) {
      showMessage('添加成功', 'success')
      showAddSmtpModal.value = false
      smtpForm.value = { email: '', password: '' }
      await loadSmtpAccounts()
      // 自动选中新添加的
      if (response.data?.id) {
        selectedAccountId.value = response.data.id
      }
    }
  } catch (error: any) {
    showMessage('添加失败: ' + (error.response?.data?.message || error.message), 'error')
  } finally {
    addingSmtp.value = false
  }
}

// 发送邮件
const sendEmail = async () => {
  if (!canSend.value || !selectedAccount.value) return

  sending.value = true
  try {
    const response = await smtpAccountsAPI.sendEmail(selectedAccount.value.id, {
      to: form.value.to,
      subject: form.value.subject,
      content: form.value.content,
      cc: form.value.cc || null,
      bcc: form.value.bcc || null
    })

    if (response.code === 0) {
      showMessage('🎉 邮件发送成功', 'success')
      // 清空表单
      form.value = {
        to: '',
        cc: '',
        bcc: '',
        subject: '',
        content: ''
      }
      showCcBcc.value = false
      // 刷新账号（更新发送次数）
      loadSmtpAccounts()
    }
  } catch (error: any) {
    const msg = error.response?.data?.message || error.message || '发送失败'
    showMessage('发送失败: ' + msg, 'error')
  } finally {
    sending.value = false
  }
}

onMounted(() => {
  loadSmtpAccounts()
})
</script>
