<template>
  <div
    class="group p-3 bg-gray-50 rounded-lg cursor-pointer"
    :class="{
      'bg-primary-50': isSelected || isChecked,
      'opacity-75': isLoading
    }"
    @click="handleClick"
  >
    <div class="flex items-center justify-between">
      <div class="flex items-center flex-1 min-w-0">
        <!-- 复选框（批量模式） -->
        <input
          v-if="batchMode"
          type="checkbox"
          :checked="isChecked"
          @click.stop="$emit('toggle-check')"
          class="w-4 h-4 cursor-pointer mr-3 flex-shrink-0"
        />
        
        <div class="flex-1 min-w-0">
          <div class="flex items-center">
            <!-- 状态图标 -->
            <slot name="status-icon"></slot>
            
            <!-- 邮箱地址 -->
            <code 
              :class="statusClass"
              class="text-sm truncate"
              :title="email"
              style="max-width: 200px;"
            >{{ email }}</code>
            
            <!-- 状态标签 -->
            <slot name="status-badge"></slot>
          </div>
          
          <!-- 底部信息 -->
          <div class="text-xs text-gray-600 mt-1">
            <slot name="bottom-info"></slot>
          </div>
        </div>
        
        <!-- Loading 状态 -->
        <div v-if="isLoading" class="ml-2">
          <svg class="animate-spin h-4 w-4 text-primary-500" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
        </div>
      </div>
      
      <!-- 操作按钮 -->
      <div class="ml-4 flex items-center space-x-2">
        <div class="opacity-0 group-hover:opacity-100 transition-opacity flex space-x-1" v-if="!batchMode">
          <slot name="actions"></slot>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  email: string
  isSelected?: boolean
  isChecked?: boolean
  isLoading?: boolean
  batchMode?: boolean
  statusClass?: string
}

withDefaults(defineProps<Props>(), {
  isSelected: false,
  isChecked: false,
  isLoading: false,
  batchMode: false,
  statusClass: 'text-black'
})

defineEmits<{
  'click': []
  'toggle-check': []
}>()

const handleClick = () => {
  // 只在非批量模式下触发点击
}
</script>
