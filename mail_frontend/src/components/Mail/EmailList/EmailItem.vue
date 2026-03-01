<template>
  <div
    class="group p-3 bg-gray-50 rounded-lg hover:bg-primary-100 transition-colors cursor-pointer relative"
    :class="{ 'bg-primary-100': isSelected || isChecked }"
    @click="handleClick"
  >
    <div class="flex items-start justify-between">
      <div class="flex items-start flex-1 min-w-0">
        <!-- 未读标记 -->
        <span
          v-if="!isRead && !batchMode"
          class="w-2 h-2 bg-primary-500 rounded-full flex-shrink-0 mt-1.5 mr-2"
          title="未读邮件"
        ></span>
        
        <!-- 复选框（批量模式） -->
        <input
          v-if="batchMode"
          type="checkbox"
          :checked="isChecked"
          @click.stop="$emit('toggle-check')"
          class="w-4 h-4 cursor-pointer mr-3 mt-1 flex-shrink-0"
        />
        
        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-black truncate" :class="{ 'font-bold': !isRead }">
            {{ subject || '(无主题)' }}
          </p>
          <p class="text-sm text-gray-600 truncate mt-1">
            发件人：{{ email.from_addr }}
          </p>
          <p class="text-xs text-gray-400 mt-1">
            {{ formatDate(date) }}
          </p>
        </div>
      </div>
      
      <!-- 操作按钮 -->
      <div class="ml-2 opacity-0 group-hover:opacity-100 transition-opacity" v-if="!batchMode">
        <slot name="actions"></slot>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatTimestamp } from '@/utils/timeUtils'

interface Props {
  subject: string
  sender: string
  date: number
  isRead?: boolean
  isSelected?: boolean
  isChecked?: boolean
  batchMode?: boolean
}

withDefaults(defineProps<Props>(), {
  isRead: false,
  isSelected: false,
  isChecked: false,
  batchMode: false
})

defineEmits<{
  'click': []
  'toggle-check': []
}>()

const formatDate = (timestamp: number) => {
  if (!timestamp) return ''
  const date = new Date(timestamp)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))
  
  if (days === 0) {
    return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
  } else if (days === 1) {
    return '昨天'
  } else if (days < 7) {
    return `${days}天前`
  } else {
    return date.toLocaleDateString('zh-CN')
  }
}

const handleClick = () => {
  // 点击事件由父组件处理
}
</script>
