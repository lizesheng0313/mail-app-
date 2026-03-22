<template>
  <BaseModal
    v-model="localVisible"
    title="批量发送邮件"
    size="lg"
    :confirmLoading="sending"
    :showConfirm="isDesktop"
    :confirmText="isDesktop ? '发送' : '请在桌面端使用'"
    @confirm="handleSend"
    @close="handleClose"
  >
    <div class="space-y-4">
      <!-- 发件邮箱选择 -->
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">发件邮箱</label>
        <select
          v-model="form.mailboxId"
          class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
          :disabled="!!presetMailboxId"
        >
          <option value="">请选择发件邮箱</option>
          <option 
            v-for="mailbox in availableMailboxes" 
            :key="mailbox.id" 
            :value="mailbox.id"
          >
            {{ mailbox.email }}
          </option>
        </select>
        <!-- 没有可用邮箱的提示 -->
        <p v-if="availableMailboxes.length === 0 && mailboxes.length > 0" class="mt-2 text-sm text-amber-600">
          ⚠️ 没有可用于发送的邮箱，请先配置可用的 SMTP 发件账号。
        </p>
        <!-- 显示不可用的邮箱 -->
        <div v-if="unavailableMailboxes.length > 0" class="mt-2">
          <p class="text-xs text-gray-500 mb-1">以下邮箱未配置可用 SMTP 发件账号，无法发送：</p>
          <div class="flex flex-wrap gap-1">
            <span 
              v-for="mailbox in unavailableMailboxes" 
              :key="mailbox.id"
              class="px-2 py-0.5 text-xs bg-gray-100 text-gray-500 rounded"
            >
              {{ mailbox.email }}
            </span>
          </div>
        </div>
      </div>

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

      <!-- 抄送 -->
      <div class="grid grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">抄送 (CC)</label>
          <input
            v-model="form.cc"
            type="text"
            placeholder="多个用逗号分隔"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">密送 (BCC)</label>
          <input
            v-model="form.bcc"
            type="text"
            placeholder="多个用逗号分隔"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
          />
        </div>
      </div>

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
        <label class="block text-sm font-medium text-gray-700 mb-1">
          正文 <span class="text-red-500">*</span>
        </label>
        <textarea
          v-model="form.content"
          rows="8"
          placeholder="请输入邮件正文..."
          class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 resize-none"
        ></textarea>
      </div>

      <!-- 提示信息 -->
      <div v-if="isDesktop" class="bg-blue-50 border border-blue-200 rounded-lg p-3">
        <div class="flex items-start">
          <svg class="w-5 h-5 text-blue-500 mt-0.5 mr-2 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
          </svg>
          <p class="text-sm text-blue-700">
            邮件将通过您选择的第三方邮箱 SMTP 服务器发送，请确保邮箱已正确配置授权码。
          </p>
        </div>
      </div>
      <div v-else class="mt-2 flex items-center text-xs text-amber-600 gap-2">
        <span>第三方发件功能仅支持桌面端，请下载桌面客户端使用。</span>
        <button
          type="button"
          class="px-2 py-0.5 text-xs rounded border border-primary-500 text-primary-600 hover:bg-primary-50"
          @click="downloadDesktop"
        >
          下载桌面端
        </button>
      </div>
    </div>
  </BaseModal>
</template>

<script setup>
import { ref, watch, computed } from 'vue'
import { isTauri } from '@/services/api'
import BaseModal from '@/components/BaseModal/index.vue'
import smtpAccountsAPI from '@/api/smtpAccounts'
import { showMessage } from '@/utils/message'
import { buildDesktopSendableSmtpAccountMap } from '@/utils/smtpCapability'

async function getTauriInvoke() {
  if (!isTauri()) return null
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    return invoke
  } catch {
    return null
  }
}

const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  },
  // 预设的邮箱ID（如果有的话）
  presetMailboxId: {
    type: Number,
    default: null
  },
  // 可用的邮箱列表
  mailboxes: {
    type: Array,
    default: () => []
  },
  // 回复邮件时预填充的收件人
  replyTo: {
    type: String,
    default: ''
  },
  // 回复邮件时预填充的主题
  replySubject: {
    type: String,
    default: ''
  }
})

const emit = defineEmits(['update:visible', 'sent'])

const isDesktop = computed(() => isTauri())

// 本地 visible 值，用于 v-model
const localVisible = computed({
  get: () => props.visible,
  set: (val) => emit('update:visible', val)
})

const smtpAccounts = ref([])

const activeSmtpAccountMap = computed(() => {
  return buildDesktopSendableSmtpAccountMap(smtpAccounts.value)
})

const availableMailboxes = computed(() => {
  return props.mailboxes.filter((mailbox) =>
    activeSmtpAccountMap.value.has(String(mailbox.email || '').toLowerCase())
  )
})

const unavailableMailboxes = computed(() => {
  return props.mailboxes.filter((mailbox) =>
    !activeSmtpAccountMap.value.has(String(mailbox.email || '').toLowerCase())
  )
})

const sending = ref(false)

const form = ref({
  mailboxId: null,
  to: '',
  cc: '',
  bcc: '',
  subject: '',
  content: ''
})

const loadSmtpAccounts = async () => {
  try {
    const response = await smtpAccountsAPI.getAccounts()
    if (response.code === 0) {
      smtpAccounts.value = response.data?.accounts || []
    } else {
      smtpAccounts.value = []
    }
  } catch (error) {
    console.error('获取 SMTP 账号列表失败:', error)
    smtpAccounts.value = []
  }
}

const getActiveSmtpAccount = (mailbox) => {
  return activeSmtpAccountMap.value.get(String(mailbox?.email || '').toLowerCase()) || null
}

// 监听 visible 变化，初始化表单
watch(() => props.visible, async (newVal) => {
  if (newVal) {
    await loadSmtpAccounts()
    const presetMailboxAvailable = props.presetMailboxId &&
      availableMailboxes.value.some((mailbox) => mailbox.id === props.presetMailboxId)

    form.value = {
      mailboxId: presetMailboxAvailable
        ? props.presetMailboxId
        : (availableMailboxes.value.length === 1 ? availableMailboxes.value[0].id : null),
      to: props.replyTo || '',
      cc: '',
      bcc: '',
      subject: props.replySubject ? `Re: ${props.replySubject}` : '',
      content: ''
    }
  }
})

const handleClose = () => {
  emit('update:visible', false)
}

const downloadDesktop = () => {
  window.location.href = 'https://zjkdongao.cn/download'
}

const handleSend = async () => {
  if (!isDesktop.value) {
    showMessage('第三方发件功能仅支持桌面端，请下载桌面客户端使用', 'warning')
    return
  }
  // 验证表单
  if (!form.value.mailboxId) {
    showMessage('请选择发件邮箱', 'error')
    return
  }
  if (!form.value.to.trim()) {
    showMessage('请输入收件人地址', 'error')
    return
  }
  if (!form.value.subject.trim()) {
    showMessage('请输入邮件主题', 'error')
    return
  }
  if (!form.value.content.trim()) {
    showMessage('请输入邮件正文', 'error')
    return
  }

  // 简单的邮箱格式验证
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  const recipients = form.value.to.split(',').map(e => e.trim()).filter(e => e)
  const invalidEmails = recipients.filter(e => !emailRegex.test(e))
  if (invalidEmails.length > 0) {
    showMessage(`收件人邮箱格式不正确: ${invalidEmails.join(', ')}`, 'error')
    return
  }

  sending.value = true

  try {
    const selectedMailbox = props.mailboxes.find((mailbox) => mailbox.id === form.value.mailboxId)
    if (!selectedMailbox) {
      showMessage('发件邮箱不存在', 'error')
      return
    }

    const smtpAccount = getActiveSmtpAccount(selectedMailbox)
    if (!smtpAccount) {
      showMessage('该邮箱未配置可用 SMTP 发件账号', 'error')
      return
    }

    const tauriInvoke = await getTauriInvoke()
    if (!tauriInvoke) {
      showMessage('第三方发件功能仅支持桌面端，请下载桌面客户端使用', 'warning')
      return
    }

    await tauriInvoke('send_smtp_email', {
      fromEmail: selectedMailbox.email,
      password: smtpAccount.password || selectedMailbox.password || '',
      smtpHost: smtpAccount.smtp_host || '',
      smtpPort: smtpAccount.smtp_port || 465,
      toEmail: form.value.to,
      subject: form.value.subject,
      content: form.value.content,
      cc: form.value.cc || null,
      bcc: form.value.bcc || null,
      attachments: []
    })

    try {
      const saveResponse = await smtpAccountsAPI.saveSentEmails({
        records: [{
          smtp_account_id: smtpAccount.id,
          external_mailbox_id: selectedMailbox.id,
          from_email: selectedMailbox.email,
          to_email: form.value.to,
          subject: form.value.subject,
          content: form.value.content
        }]
      })
      if (saveResponse.code !== 0) {
        showMessage('邮件已发送，但发送记录同步失败', 'warning')
      }
    } catch (error) {
      console.error('保存发送记录失败:', error)
      showMessage('邮件已发送，但发送记录同步失败', 'warning')
    }

    showMessage('邮件发送成功', 'success')
    emit('sent')
    handleClose()
  } catch (error) {
    console.error('发送失败:', error)
    const message = error?.message || error?.toString?.() || '发送失败'
    showMessage(message.startsWith('发送失败') ? message : `发送失败: ${message}`, 'error')
  } finally {
    sending.value = false
  }
}
</script>
