<template>
  <div class="p-6">
    <!-- 操作栏 -->
    <div class="bg-white rounded-lg shadow p-4 mb-4">
      <div class="flex items-center gap-4">
        <!-- 搜索框 -->
        <div class="flex-1">
          <input
            v-model="searchKeyword"
            type="text"
            placeholder="搜索公告标题或内容..."
            class="w-full px-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            @keyup.enter="handleSearch"
          />
        </div>

        <!-- 按钮组 -->
        <div class="flex gap-2">
          <button
            @click="handleSearch"
            class="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-md hover:bg-primary-700"
          >
            查询
          </button>
          <button
            @click="handleReset"
            class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
          >
            重置
          </button>
          <button
            @click="showCreateModal = true"
            class="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-md hover:bg-primary-700"
          >
            + 发布公告
          </button>
        </div>
      </div>
    </div>

    <!-- 公告列表 -->
    <AdminDataTable
      :loading="loading"
      :pagination="{
        page: page,
        pages: totalPages,
        total: total,
        limit: pageSize
      }"
      :column-count="6"
      @page-change="handlePageChange"
      @page-size-change="handlePageSizeChange"
    >
      <template #thead>
        <tr>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">标题</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">类型</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">内容</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">发布时间</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">发布者</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">操作</th>
        </tr>
      </template>

      <template #tbody>
        <tr v-if="announcements.length === 0">
          <td colspan="6" class="px-6 py-12 text-center text-gray-500">
            暂无公告
          </td>
        </tr>
        <tr v-for="item in announcements" :key="item.id" class="hover:bg-gray-50">
          <td class="px-6 py-4">
            <div class="text-sm font-medium text-gray-900">{{ item.title }}</div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap">
            <span
              :class="[
                'px-2 py-1 text-xs rounded-full',
                item.type === 'info' ? 'bg-blue-100 text-blue-700' :
                item.type === 'warning' ? 'bg-yellow-100 text-yellow-700' :
                item.type === 'success' ? 'bg-green-100 text-green-700' :
                'bg-red-100 text-red-700'
              ]"
            >
              {{ getTypeName(item.type) }}
            </span>
          </td>
          <td class="px-6 py-4">
            <div class="text-sm text-gray-600 line-clamp-2">{{ item.content }}</div>
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
            {{ formatDate(item.created_at) }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
            {{ item.created_by_email || '-' }}
          </td>
          <td class="px-6 py-4 whitespace-nowrap">
            <ActionButton
              :actions="[
                { label: '编辑', onClick: () => editAnnouncement(item) },
                { label: '删除', onClick: () => confirmDelete(item), type: 'danger' }
              ]"
            />
          </td>
        </tr>
      </template>
    </AdminDataTable>

    <!-- 创建/编辑公告弹窗 -->
    <div v-if="showCreateModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg shadow-xl max-w-2xl w-full mx-4">
        <div class="flex items-center justify-between px-6 py-4 border-b">
          <h3 class="text-lg font-semibold">{{ editingItem ? '编辑公告' : '发布公告' }}</h3>
          <button @click="closeModal" class="text-gray-400 hover:text-gray-600">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div class="p-6 space-y-4">
          <!-- 标题 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">标题</label>
            <input
              v-model="formData.title"
              type="text"
              placeholder="请输入公告标题"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
            />
          </div>

          <!-- 类型 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">类型</label>
            <CustomSelect
              v-model="formData.type"
              :options="typeOptions"
              placeholder="选择公告类型"
            />
          </div>

          <!-- 内容 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">内容</label>
            <textarea
              v-model="formData.content"
              rows="5"
              placeholder="请输入公告内容"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500"
            ></textarea>
          </div>
        </div>

        <div class="flex gap-2 px-6 py-4 border-t bg-gray-50">
          <button
            @click="handleSave"
            :disabled="saving"
            class="px-4 py-2 bg-primary-600 text-white rounded-md hover:bg-primary-700 disabled:opacity-50"
          >
            {{ saving ? '保存中...' : '保存' }}
          </button>
          <button
            @click="closeModal"
            class="px-4 py-2 bg-white border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50"
          >
            取消
          </button>
        </div>
      </div>
    </div>

    <!-- 删除确认对话框 -->
    <ConfirmDialog
      :visible="showDeleteConfirm"
      title="确认删除"
      message="确定要删除这条公告吗？"
      type="danger"
      @confirm="handleDelete"
      @cancel="showDeleteConfirm = false"
    />
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { showMessage } from '@/utils/message'
import AdminDataTable from '@/components/AdminDataTable/index.vue'
import ConfirmDialog from '@/components/ConfirmDialog/index.vue'
import CustomSelect from '@/components/CustomSelect/index.vue'
import ActionButton from '@/components/ActionButton/index.vue'

const loading = ref(false)
const saving = ref(false)
const searchKeyword = ref('')
const showCreateModal = ref(false)
const showDeleteConfirm = ref(false)
const editingItem = ref(null)
const deleteTarget = ref(null)

const page = ref(1)
const pageSize = ref(20)
const total = ref(0)
const announcements = ref([])

const formData = ref({
  title: '',
  type: 'info',
  content: ''
})

// 公告类型选项
const typeOptions = [
  { value: 'info', label: '信息' },
  { value: 'warning', label: '警告' },
  { value: 'success', label: '成功' },
  { value: 'error', label: '错误' }
]

const totalPages = computed(() => Math.ceil(total.value / pageSize.value))

const getTypeName = (type) => {
  const typeMap = {
    'info': '信息',
    'warning': '警告',
    'success': '成功',
    'error': '错误'
  }
  return typeMap[type] || type
}

const formatDate = (timestamp) => {
  if (!timestamp) return '-'
  const date = new Date(timestamp)
  return date.toLocaleString('zh-CN')
}

const loadAnnouncements = async () => {
  loading.value = true
  try {
    const token = localStorage.getItem('token')
    const response = await fetch(`/mail-api/v1/api/announcements/?page=${page.value}&page_size=${pageSize.value}`, {
      headers: {
        'Authorization': `Bearer ${token}`
      }
    })

    const result = await response.json()
    if (result.code === 0) {
      announcements.value = result.data.items || []
      total.value = result.data.total || 0

      // 调试：打印公告数据
      console.log('管理后台公告列表:', result.data.items.map(item => ({
        id: item.id,
        title: item.title,
        created_at: item.created_at,
        created_at_type: typeof item.created_at,
        formatted: formatDate(item.created_at),
        date_object: new Date(item.created_at)
      })))
    }
  } catch (error) {
    console.error('加载公告失败:', error)
    showMessage('加载公告失败', 'error')
  } finally {
    loading.value = false
  }
}

const handlePageChange = (newPage) => {
  page.value = newPage
  loadAnnouncements()
}

const handlePageSizeChange = (newSize) => {
  pageSize.value = newSize
  page.value = 1
  loadAnnouncements()
}

const editAnnouncement = (item) => {
  editingItem.value = item
  formData.value = {
    title: item.title,
    type: item.type,
    content: item.content
  }
  showCreateModal.value = true
}

const closeModal = () => {
  showCreateModal.value = false
  editingItem.value = null
  formData.value = {
    title: '',
    type: 'info',
    content: ''
  }
}

const handleSave = async () => {
  if (!formData.value.title.trim()) {
    showMessage('请输入标题', 'warning')
    return
  }
  if (!formData.value.content.trim()) {
    showMessage('请输入内容', 'warning')
    return
  }

  saving.value = true
  try {
    const token = localStorage.getItem('token')
    const url = editingItem.value
      ? `/mail-api/v1/api/announcements/${editingItem.value.id}`
      : '/mail-api/v1/api/announcements/'

    const response = await fetch(url, {
      method: editingItem.value ? 'PUT' : 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`
      },
      body: JSON.stringify(formData.value)
    })

    const result = await response.json()
    if (result.code === 0) {
      showMessage(editingItem.value ? '更新成功' : '发布成功', 'success')
      closeModal()
      loadAnnouncements()
    } else {
      showMessage(result.message || '操作失败', 'error')
    }
  } catch (error) {
    console.error('保存失败:', error)
    showMessage('保存失败', 'error')
  } finally {
    saving.value = false
  }
}

const confirmDelete = (item) => {
  deleteTarget.value = item
  showDeleteConfirm.value = true
}

const handleSearch = () => {
  page.value = 1
  loadAnnouncements()
}

const handleReset = () => {
  searchKeyword.value = ''
  page.value = 1
  loadAnnouncements()
}

const handleDelete = async () => {
  try {
    const token = localStorage.getItem('token')
    const response = await fetch(`/mail-api/v1/api/announcements/${deleteTarget.value.id}`, {
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${token}`
      }
    })

    const result = await response.json()
    if (result.code === 0) {
      showMessage('删除成功', 'success')
      loadAnnouncements()
    } else {
      showMessage(result.message || '删除失败', 'error')
    }
  } catch (error) {
    console.error('删除失败:', error)
    showMessage('删除失败', 'error')
  } finally {
    showDeleteConfirm.value = false
    deleteTarget.value = null
  }
}

onMounted(() => {
  loadAnnouncements()
})
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
