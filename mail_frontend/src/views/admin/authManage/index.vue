<template>
  <div class="h-full">
    <div class=" h-full flex flex-col">

      <!-- 操作区域 -->
      <div class="bg-white rounded-lg shadow-sm border p-6 mb-8">
        <div class="flex justify-between items-center">
          <div class="flex items-center space-x-4">
            <CustomSelect
              v-model="queryForm.code_type"
              :options="queryCodeTypeOptions"
              placeholder="筛选类型"
            />
            <CustomSelect
              v-model="queryForm.status"
              :options="statusOptions"
              placeholder="筛选状态"
            />
            <BaseInput
              v-model="queryForm.keyword"
              placeholder="搜索卡密"
              class="w-48"
              size="sm"
            >
              <template #left-icon>
                <svg class="h-4 w-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
              </template>
            </BaseInput>
            <button
              @click="handleQuery"
              class="px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white rounded-md text-sm"
            >
              查询
            </button>
          </div>
          <div class="flex items-center space-x-2">
            <button 
              @click="showCreateModal = true"
              class="px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white rounded-md text-sm"
            >
              创建卡密
            </button>
          </div>
        </div>
      </div>

      <!-- 创建卡密弹窗 -->
      <div v-if="showCreateModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg p-6 w-full max-w-md">
          <h3 class="text-lg font-semibold mb-4">创建卡密</h3>

          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-black mb-2">卡密类型</label>
              <CustomSelect
                v-model="createForm.code_type"
                :options="codeTypeOptions"
                placeholder="选择卡密类型"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-black mb-2">数量</label>
              <BaseInput
                v-model="createForm.count"
                type="number"
                min="1"
                max="5000"
                placeholder="1-5000"
                size="sm"
              />
            </div>
          </div>

          <div class="flex justify-end space-x-3 mt-6">
            <button
              @click="cancelCreate"
              class="px-4 py-2 text-black hover:text-black"
            >
              取消
            </button>
            <button
              @click="createAuthCodes"
              :disabled="loading || !createForm.code_type || !createForm.count"
              class="px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white rounded-md disabled:opacity-50"
            >
              {{ loading ? '创建中...' : '创建卡密' }}
            </button>
          </div>
        </div>
      </div>

      <!-- 创建成功弹窗 -->
      <div v-if="showCreatedModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[80vh] overflow-hidden">
          <div class="p-6 border-b">
            <h3 class="text-lg font-semibold text-black">卡密创建成功</h3>
            <p class="text-sm text-black mt-1">
              已成功创建 {{ createdCodes.length }} 个 {{ getCodeTypeName(createdCodesType) }} 卡密
            </p>
          </div>

          <div class="p-6 max-h-96 overflow-y-auto">
            <div class="mb-4">
              <label class="block text-sm font-medium text-black mb-2">生成的卡密（每行一个）</label>
              <textarea
                ref="codesTextarea"
                :value="createdCodesText"
                readonly
                class="w-full h-48 px-3 py-2 border border-gray-300 rounded-md bg-gray-50 font-mono text-sm"
                placeholder="卡密将显示在这里..."
              ></textarea>
            </div>

            <div class="flex gap-3">
              <button
                @click="copyAllCodes"
                class="flex-1 bg-primary-600 hover:bg-primary-700 text-white px-4 py-2 rounded-md font-medium transition-colors"
              >
                {{ copyButtonText }}
              </button>
              <button
                @click="downloadCodes"
                class="flex-1 bg-primary-600 hover:bg-primary-700 text-white px-4 py-2 rounded-md font-medium transition-colors"
              >
                下载为文件
              </button>
            </div>
          </div>

          <div class="p-6 border-t bg-gray-50 flex justify-end">
            <button
              @click="closeCreatedModal"
              class="px-4 py-2 text-black hover:text-black font-medium"
            >
              关闭
            </button>
          </div>
        </div>
      </div>

      <!-- 卡密列表 -->
      <AdminDataTable
        title="卡密列表"
        :pagination="pagination"
        :loading="loading"
        :show-page-size-selector="true"
        :column-count="7"
        @page-change="changePage"
        @page-size-change="handlePageSizeChange"
      >
        <template #thead>
          <tr>
                  <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">卡密</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">类型</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">配额</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">状态</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">创建时间</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">使用时间</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-black uppercase tracking-wider">操作</th>
          </tr>
        </template>
        <template #tbody>
          <tr v-for="code in authCodes" :key="code.id" class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap">
                    <div class="flex items-center">
                      <code class="text-sm font-mono bg-gray-100 px-2 py-1 rounded">{{ code.code }}</code>
                      <button 
                        @click="copyToClipboard(code.code)"
                        class="ml-2 text-gray-400 hover:text-black"
                        title="复制卡密"
                      >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                        </svg>
                      </button>
                    </div>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-black">
                    {{ getCodeTypeName(code.code_type) }}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-black">
                    {{ code.mailbox_quota }}个邮箱
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span :class="code.is_used ? 'bg-red-100 text-red-800' : 'bg-primary-100 text-success-800'" 
                          class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full">
                      {{ code.is_used ? '已使用' : '未使用' }}
                    </span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-black">
                    {{ formatDate(code.created_at) }}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-black">
                    {{ code.used_at ? formatDate(code.used_at) : '-' }}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                    <div class="flex items-center space-x-2">
                      <ActionButton
                        icon="copy"
                        tooltip="复制"
                        variant="copy"
                        @click="copyToClipboard(code.code)"
                      />
                      <ActionButton
                        icon="delete"
                        tooltip="删除"
                        variant="delete"
                        @click="deleteAuthCode(code)"
                      />
                    </div>
                  </td>
          </tr>
          <tr v-if="authCodes.length === 0">
            <td colspan="7" class="px-6 py-12 text-center text-black">
              暂无卡密数据
            </td>
          </tr>
        </template>
      </AdminDataTable>
    </div>




  <!-- 删除确认对话框 -->
  <ConfirmDialog
    :visible="showDeleteConfirm"
    title="确认删除"
    :message="`确定要删除卡密 ${deletingCode?.code} 吗？`"
    type="danger"
    confirm-text="删除"
    :loading="deleting"
    :show-warning="true"
    @confirm="confirmDelete"
    @cancel="showDeleteConfirm = false"
  />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import BaseInput from '@/components/BaseInput/index.vue'
import CustomSelect from '@/components/CustomSelect/index.vue'
import ActionButton from '@/components/ActionButton/index.vue'
import { authCodeAPI } from '@/api/authCode'
import AdminDataTable from '@/components/AdminDataTable/index.vue'
import { showMessage } from '@/utils/message'
import ConfirmDialog from '@/components/ConfirmDialog/index.vue'
// 响应式数据
const loading = ref(false)
const authCodes = ref<any[]>([])

// 创建弹窗
const showCreateModal = ref(false)

// 删除确认对话框
const showDeleteConfirm = ref(false)
const deletingCode = ref(null)
const deleting = ref(false)

// 查询表单
const queryForm = ref({
  code_type: '',
  status: '',
  keyword: ''
})

// 查询类型选项（包含全部选项）
const queryCodeTypeOptions = [
  { label: '全部类型', value: '' },
  { label: '1邮箱卡密', value: 'MAILBOX_1' },
  { label: '10邮箱卡密', value: 'MAILBOX_10' },
  { label: '50邮箱卡密', value: 'MAILBOX_50' },
  { label: '100邮箱卡密', value: 'MAILBOX_100' },
  { label: '300邮箱卡密', value: 'MAILBOX_300' },
  { label: '500邮箱卡密', value: 'MAILBOX_500' },
  { label: '1000邮箱卡密', value: 'MAILBOX_1000' }
]

// 状态选项
const statusOptions = [
  { label: '全部状态', value: '' },
  { label: '未使用', value: 'unused' },
  { label: '已使用', value: 'used' }
]

// 卡密类型选项
const codeTypeOptions = [
  { label: '1邮箱卡密', value: 'MAILBOX_1' },
  { label: '10邮箱卡密', value: 'MAILBOX_10' },
  { label: '50邮箱卡密', value: 'MAILBOX_50' },
  { label: '100邮箱卡密', value: 'MAILBOX_100' },
  { label: '300邮箱卡密', value: 'MAILBOX_300' },
  { label: '500邮箱卡密', value: 'MAILBOX_500' },
  { label: '1000邮箱卡密', value: 'MAILBOX_1000' }
]

// 创建表单
const createForm = ref({
  code_type: 'MAILBOX_1',
  count: 1
})

// 分页信息
const pagination = ref({
  page: 1,
  limit: 20,
  total: 0,
  pages: 0
})

// 创建成功弹窗相关
const showCreatedModal = ref(false)
const createdCodes = ref<any[]>([])
const createdCodesType = ref('')
const copyButtonText = ref('复制所有卡密')
const codesTextarea = ref<HTMLTextAreaElement | null>(null)

// 计算属性
const createdCodesText = computed(() => {
  return createdCodes.value.map(code => code.code).join('\n')
})

// 页面加载时获取卡密列表
onMounted(() => {
  fetchAuthCodes()
})

// 获取卡密列表
const fetchAuthCodes = async () => {
  loading.value = true
  try {
    const filters = {
      code_type: queryForm.value.code_type || undefined,
      status: queryForm.value.status || undefined,
      keyword: queryForm.value.keyword || undefined
    }
    
    const response: any = await authCodeAPI.getAuthCodes(
      pagination.value.page, 
      pagination.value.limit, 
      filters
    )
    
    if (response.code === 0) {
      authCodes.value = response.data.codes
      pagination.value = {
        ...pagination.value,
        total: response.data.total,
        pages: response.data.pages
      }
    } else {
      showMessage(response.message || '获取卡密列表失败', 'error')
    }
  } catch (error: any) {
    showMessage(error.response?.data?.message || '获取卡密列表失败', 'error')
  } finally {
    loading.value = false
  }
}

// 处理查询
const handleQuery = () => {
  pagination.value.page = 1
  fetchAuthCodes()
}

// 创建卡密
const createAuthCodes = async () => {
  if (!createForm.value.code_type || !createForm.value.count) {
    showMessage('请填写完整信息', 'error')
    return
  }

  loading.value = true
  try {
    const response: any = await authCodeAPI.createAuthCodes(
      createForm.value.code_type,
      createForm.value.count
    )
    
    if (response.code === 0) {
      // 显示创建成功弹窗
      createdCodes.value = response.data.codes || []
      createdCodesType.value = createForm.value.code_type
      showCreatedModal.value = true
      showCreateModal.value = false

      // 重置表单
      createForm.value.count = 1
      // 刷新列表
      await fetchAuthCodes()
    } else {
      showMessage(response.message || '创建卡密失败', 'error')
    }
  } catch (error: any) {
    showMessage(error.response?.data?.message || '创建卡密失败', 'error')
  } finally {
    loading.value = false
  }
}

// 取消创建卡密
const cancelCreate = () => {
  showCreateModal.value = false
  createForm.value = {
    code_type: 'MAILBOX_1',
    count: 1
  }
}

// 删除卡密
const deleteAuthCode = (code) => {
  deletingCode.value = code
  showDeleteConfirm.value = true
}

const confirmDelete = async () => {
  deleting.value = true
  try {
    const response = await authCodeAPI.deleteAuthCode(deletingCode.value.id)
    if (response.code === 0) {
      showMessage('卡密删除成功', 'success')
      await fetchAuthCodes()
    } else {
      showMessage(response.message || '删除失败', 'error')
    }
  } catch (error) {
    showMessage('删除失败', 'error')
  } finally {
    deleting.value = false
    showDeleteConfirm.value = false
  }
}

// 分页切换
const changePage = (page: number) => {
  if (page >= 1 && page <= pagination.value.pages) {
    pagination.value.page = page
    fetchAuthCodes()
  }
}

// 改变每页显示数量
const handlePageSizeChange = (newPageSize: number) => {
  pagination.value.limit = newPageSize
  pagination.value.page = 1 // 重置到第一页
  fetchAuthCodes()
}



// 复制到剪贴板
const copyToClipboard = async (text: string) => {
  try {
    await navigator.clipboard.writeText(text)
    showMessage('卡密已复制到剪贴板', 'success')
  } catch (error) {
    showMessage('复制失败', 'error')
  }
}


// 获取卡密类型名称
const getCodeTypeName = (type: string) => {
  const typeMap: Record<string, string> = {
    'MAILBOX_1': '1邮箱',
    'MAILBOX_10': '10邮箱',
    'MAILBOX_50': '50邮箱',
    'MAILBOX_100': '100邮箱',
    'MAILBOX_300': '300邮箱',
    'MAILBOX_500': '500邮箱',
    'MAILBOX_1000': '1000邮箱'
  }
  return typeMap[type] || type
}

// 关闭创建成功弹窗
const closeCreatedModal = () => {
  showCreatedModal.value = false
  createdCodes.value = []
  createdCodesType.value = ''
  copyButtonText.value = '复制所有卡密'
}

// 复制所有卡密
const copyAllCodes = async () => {
  try {
    await navigator.clipboard.writeText(createdCodesText.value)
    copyButtonText.value = '已复制！'
    setTimeout(() => {
      copyButtonText.value = '复制所有卡密'
    }, 2000)
  } catch (error) {
    // 降级方案：使用传统方法
    if (codesTextarea.value) {
      codesTextarea.value.select()
      document.execCommand('copy')
      copyButtonText.value = '已复制！'
      setTimeout(() => {
        copyButtonText.value = '复制所有卡密'
      }, 2000)
    }
  }
}

// 下载卡密为文件
const downloadCodes = () => {
  const content = createdCodesText.value
  const blob = new Blob([content], { type: 'text/plain;charset=utf-8' })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download = `卡密_${getCodeTypeName(createdCodesType.value)}_${new Date().toISOString().slice(0, 10)}.txt`
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
  URL.revokeObjectURL(url)
}

// 格式化日期
const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleString('zh-CN')
}


</script>
