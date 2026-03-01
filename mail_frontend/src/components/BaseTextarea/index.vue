<template>
  <div class="relative">
    <!-- 标签 -->
    <label 
      v-if="label" 
      :for="textareaId" 
      class="block text-sm font-medium text-black mb-2"
    >
      <span class="flex items-center">
        <slot name="label-icon"></slot>
        {{ label }}
        <span v-if="required" class="text-red-500 ml-1">*</span>
      </span>
    </label>

    <!-- 文本域容器 -->
    <div class="relative">
      <textarea
        :id="textareaId"
        :value="modelValue"
        :placeholder="placeholder"
        :required="required"
        :disabled="disabled"
        :readonly="readonly"
        :rows="rows"
        :maxlength="maxlength"
        :class="textareaClasses"
        @input="handleInput"
        @blur="handleBlur"
        @focus="handleFocus"
        v-bind="$attrs"
      ></textarea>

      <!-- 字符计数 -->
      <div 
        v-if="showCount && maxlength" 
        class="absolute bottom-2 right-3 text-xs text-gray-400 bg-white px-1"
      >
        {{ currentLength }}/{{ maxlength }}
      </div>
    </div>

    <!-- 帮助文本/错误信息 -->
    <div v-if="helpText || errorMessage" class="mt-2">
      <p v-if="errorMessage" class="text-sm text-red-600">{{ errorMessage }}</p>
      <p v-else-if="helpText" class="text-sm text-black">{{ helpText }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, useAttrs } from 'vue'

interface Props {
  modelValue?: string
  label?: string
  placeholder?: string
  required?: boolean
  disabled?: boolean
  readonly?: boolean
  rows?: number
  maxlength?: number
  showCount?: boolean
  helpText?: string
  errorMessage?: string
  size?: 'sm' | 'md' | 'lg'
  resize?: 'none' | 'vertical' | 'horizontal' | 'both'
}

const props = withDefaults(defineProps<Props>(), {
  rows: 4,
  size: 'md',
  resize: 'vertical',
  showCount: false
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
  'blur': [event: FocusEvent]
  'focus': [event: FocusEvent]
}>()

// 禁用属性继承到根元素
defineOptions({
  inheritAttrs: false
})

const attrs = useAttrs()

// 生成唯一ID
const textareaId = computed(() => {
  return attrs.id as string || `textarea-${Math.random().toString(36).substr(2, 9)}`
})

// 当前字符长度
const currentLength = computed(() => {
  return (props.modelValue || '').length
})

// 计算文本域样式类
const textareaClasses = computed(() => {
  const baseClasses = [
    'block w-full border border-gray-300 rounded-lg',
    'bg-white text-black placeholder-gray-500',
    'focus:outline-none focus:border-primary-500',
    'transition-all duration-200',
    'disabled:bg-gray-50 disabled:text-black disabled:cursor-not-allowed',
    'readonly:bg-gray-50 readonly:cursor-default'
  ]

  // 尺寸相关的padding
  const sizeClasses = {
    sm: 'px-3 py-2 text-sm',
    md: 'px-4 py-3 text-sm',
    lg: 'px-4 py-4 text-base'
  }

  // 调整大小样式
  const resizeClasses = {
    none: 'resize-none',
    vertical: 'resize-y',
    horizontal: 'resize-x',
    both: 'resize'
  }

  // 错误状态样式
  const errorClasses = props.errorMessage
    ? ['border-red-300 focus:border-red-500']
    : []

  return [
    ...baseClasses,
    sizeClasses[props.size],
    resizeClasses[props.resize],
    ...errorClasses
  ].join(' ')
})

// 事件处理
const handleInput = (event: Event) => {
  const target = event.target as HTMLTextAreaElement
  emit('update:modelValue', target.value)
}

const handleBlur = (event: FocusEvent) => {
  emit('blur', event)
}

const handleFocus = (event: FocusEvent) => {
  emit('focus', event)
}
</script>
