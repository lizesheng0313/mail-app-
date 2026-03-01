/**
 * 批量选择逻辑（可复用）
 * 用于邮箱列表、邮件列表的批量操作
 */

import { ref, computed, nextTick, type Ref, type ComputedRef } from 'vue'

export function useBatchSelection<T extends { id: number }>(itemsRef: Ref<T[]> | ComputedRef<T[]>) {
  const isBatchMode = ref(false)
  const selectedIds = ref<number[]>([])

  // 计算属性
  const selectedCount = computed(() => selectedIds.value.length)
  const isAllSelected = computed(() => 
    itemsRef.value.length > 0 && itemsRef.value.every(item => selectedIds.value.includes(item.id))
  )

  // 开始批量模式
  const startBatchMode = () => {
    isBatchMode.value = true
    selectedIds.value = []
  }

  // 取消批量模式
  const cancelBatchMode = () => {
    isBatchMode.value = false
    selectedIds.value = []
  }

  // 切换单个选择
  const toggleSelection = (id: number) => {
    console.log('🟡 toggleSelection 被调用, id:', id)
    console.log('🟡 当前 selectedIds:', selectedIds.value)
    const currentIds = [...selectedIds.value]
    const index = currentIds.indexOf(id)
    if (index > -1) {
      // 已选中，取消选中
      console.log('🟡 取消选中 id:', id)
      currentIds.splice(index, 1)
    } else {
      // 未选中，添加选中
      console.log('🟡 添加选中 id:', id)
      currentIds.push(id)
    }
    // 替换整个数组触发响应式
    selectedIds.value = currentIds
    console.log('🟡 更新后的 selectedIds:', selectedIds.value)
  }

  // 全选/取消全选
  const toggleSelectAll = () => {
    if (isAllSelected.value) {
      selectedIds.value = []
    } else {
      selectedIds.value = itemsRef.value.map(item => item.id)
    }
  }

  // 获取选中的ID列表
  const getSelectedIds = () => selectedIds.value

  // 清空选中项（但保持批量模式）
  const clearSelected = () => {
    selectedIds.value = []
  }

  return {
    isBatchMode,
    selectedIds,
    selectedCount,
    isAllSelected,
    startBatchMode,
    cancelBatchMode,
    toggleSelection,
    toggleSelectAll,
    getSelectedIds,
    clearSelected
  }
}
