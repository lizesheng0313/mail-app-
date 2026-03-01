<template>
  <div class="h-full">
    <div class="h-full flex flex-col">
      <!-- 操作栏 -->
      <div class="bg-white rounded-lg shadow-sm border p-6 mb-8">
        <div class="flex justify-between items-center">
          <div>
            <h2 class="text-xl font-semibold text-gray-900">视频激励配置</h2>
            <p class="text-sm text-gray-600 mt-1">配置小程序工作流和视频奖励规则</p>
          </div>
          <button
            @click="openAddModal"
            class="px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white rounded-md text-sm flex items-center"
          >
            <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
            </svg>
            添加配置
          </button>
        </div>
      </div>

      <!-- 配置列表 -->
      <AdminDataTable
        :loading="loading"
        :column-count="8"
      >
        <template #thead>
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">ID</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">小程序</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">工作流</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">奖励类型</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">所需视频数</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">奖励数量</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">状态</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">操作</th>
          </tr>
        </template>
        <template #tbody>
          <tr v-for="item in configs" :key="item.id" class="hover:bg-gray-50">
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{{ item.id }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{{ item.miniapp_name }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
              <div>{{ item.workflow_name }}</div>
              <div class="text-xs text-gray-500">ID: {{ item.workflow_id }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span :class="getRewardTypeClass(item.reward_type)" class="px-2 py-1 text-xs rounded-full">
                {{ getRewardTypeName(item.reward_type) }}
              </span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{{ item.videos_required }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{{ item.reward_amount }}</td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span :class="item.enabled ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'" class="px-2 py-1 text-xs rounded-full">
                {{ item.enabled ? '启用' : '禁用' }}
              </span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm">
              <div class="flex items-center space-x-3">
                <ActionButton
                  icon="edit"
                  tooltip="编辑"
                  @click.stop="openEditModal(item)"
                />
                <ActionButton
                  icon="delete"
                  tooltip="删除"
                  variant="danger"
                  @click.stop="confirmDelete(item)"
                />
              </div>
            </td>
          </tr>
          <tr v-if="configs.length === 0">
            <td colspan="8" class="px-6 py-8 text-center text-gray-500">暂无配置</td>
          </tr>
        </template>
      </AdminDataTable>
    </div>

    <!-- 添加/编辑弹窗 -->
    <BaseModal 
      v-model="showModal" 
      :title="isEdit ? '编辑配置' : '添加配置'"
      :show-footer="false"
      @close="closeModal"
    >
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">选择小程序 *</label>
          <CustomSelect
            v-model="formData.miniapp_id"
            :options="miniappOptions"
            placeholder="请选择"
          />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">选择工作流 *</label>
          <CustomSelect
            v-model="formData.workflow_id"
            :options="workflowOptions"
            placeholder="请选择"
          />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">奖励类型 *</label>
          <CustomSelect
            v-model="formData.reward_type"
            :options="rewardTypeOptions"
            placeholder="请选择"
          />
          <p class="text-xs text-gray-500 mt-1">
            <span v-if="formData.reward_type === 'quota'">用户可用于创建临时邮箱的配额</span>
            <span v-if="formData.reward_type === 'mailbox'">直接兑换工作流生成的邮箱账号</span>
            <span v-if="formData.reward_type === 'milk_coin'">平台虚拟货币</span>
          </p>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">所需视频数 *</label>
            <BaseInput
              v-model.number="formData.videos_required"
              type="number"
              min="1"
              placeholder="例如：10"
            />
            <p class="text-xs text-gray-500 mt-1">观看多少个视频可兑换</p>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">奖励数量 *</label>
            <BaseInput
              v-model.number="formData.reward_amount"
              type="number"
              min="1"
              placeholder="例如：1"
            />
            <p class="text-xs text-gray-500 mt-1">
              <span v-if="formData.reward_type === 'quota'">获得的配额数量</span>
              <span v-if="formData.reward_type === 'mailbox'">获得的邮箱数量</span>
              <span v-if="formData.reward_type === 'milk_coin'">获得的奶片数量</span>
            </p>
          </div>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">描述</label>
          <BaseTextarea
            v-model="formData.description"
            :rows="2"
            placeholder="配置说明"
          />
        </div>

        <div class="flex items-center">
          <input
            v-model="formData.enabled"
            type="checkbox"
            id="enabled"
            class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
          />
          <label for="enabled" class="ml-2 text-sm text-gray-700">启用此配置</label>
        </div>
      </div>

      <div class="flex justify-end space-x-3 mt-6">
        <button
          @click="closeModal"
          class="px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50"
        >
          取消
        </button>
        <button
          @click="saveItem"
          :disabled="loading"
          class="px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white rounded-md disabled:opacity-50"
        >
          {{ loading ? '保存中...' : '保存' }}
        </button>
      </div>
    </BaseModal>

    <!-- 删除确认对话框 -->
    <ConfirmDialog
      :visible="showDeleteDialog"
      title="删除配置"
      :message="`确定要删除配置【${deleteTarget?.miniapp_name} - ${deleteTarget?.workflow_name}】吗？`"
      :loading="deleting"
      @confirm="handleDelete"
      @cancel="showDeleteDialog = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { showMessage } from '@/utils/message'
import api from '@/services/api'
import AdminDataTable from '@/components/AdminDataTable/index.vue'
import ActionButton from '@/components/ActionButton/index.vue'
import BaseModal from '@/components/BaseModal/index.vue'
import BaseInput from '@/components/BaseInput/index.vue'
import BaseTextarea from '@/components/BaseTextarea/index.vue'
import CustomSelect from '@/components/CustomSelect/index.vue'
import ConfirmDialog from '@/components/ConfirmDialog/index.vue'

interface RewardConfig {
  id: number
  miniapp_id: number
  miniapp_name: string
  workflow_id: number
  workflow_name: string
  reward_type: string
  videos_required: number
  reward_amount: number
  description: string
  enabled: boolean
}

const configs = ref<RewardConfig[]>([])
const miniapps = ref<any[]>([])
const workflows = ref<any[]>([])
const loading = ref(false)
const showModal = ref(false)
const isEdit = ref(false)
const showDeleteDialog = ref(false)
const deleteTarget = ref<RewardConfig | null>(null)
const deleting = ref(false)
const formData = ref({
  id: 0,
  miniapp_id: '',
  workflow_id: '',
  reward_type: 'quota',
  videos_required: 1,
  reward_amount: 1,
  description: '',
  enabled: true
})

const rewardTypeOptions = [
  { value: 'quota', label: '邮箱配额' },
  { value: 'mailbox', label: '邮箱账号' },
  { value: 'milk_coin', label: '奶片' }
]

const miniappOptions = computed(() => 
  miniapps.value.map(app => ({ value: app.id.toString(), label: app.title }))
)

const workflowOptions = computed(() => 
  workflows.value.map(wf => ({ value: wf.id.toString(), label: `${wf.name} (ID: ${wf.id})` }))
)

onMounted(() => {
  fetchList()
  fetchMiniapps()
  
  // 延迟加载工作流列表
  setTimeout(() => {
    fetchWorkflows()
  }, 500)
})

const fetchList = async () => {
  try {
    const response = await api.get('/admin/miniapp-reward/list')
    if (response.code === 0) {
      configs.value = response.data || []
    } else {
      showMessage(response.message || '获取列表失败', 'error')
    }
  } catch (error: any) {
    console.error('获取列表失败:', error)
    showMessage(error.response?.data?.message || '获取列表失败', 'error')
  }
}

const fetchMiniapps = async () => {
  try {
    const response = await api.get('/admin/miniapp/list')
    if (response.code === 0) {
      miniapps.value = (response.data || []).filter((item: any) => item.enabled)
    } else {
      showMessage(response.message || '获取小程序列表失败', 'error')
    }
  } catch (error: any) {
    console.error('获取小程序列表失败:', error)
    showMessage(error.response?.data?.message || '获取小程序列表失败', 'error')
  }
}

const fetchWorkflows = async () => {
  try {
    const response = await api.get('/workflows/', { params: { all: true } })
    if (response.code === 0) {
      workflows.value = response.data?.workflows || []
    }
  } catch (error: any) {
    // 静默失败，不影响页面
    workflows.value = []
  }
}

const openAddModal = () => {
  isEdit.value = false
  formData.value = {
    id: 0,
    miniapp_id: '',
    workflow_id: '',
    reward_type: 'quota',
    videos_required: 1,
    reward_amount: 1,
    description: '',
    enabled: true
  }
  showModal.value = true
}

const openEditModal = (item: RewardConfig) => {
  isEdit.value = true
  formData.value = {
    id: item.id,
    miniapp_id: item.miniapp_id.toString(),
    workflow_id: item.workflow_id.toString(),
    reward_type: item.reward_type,
    videos_required: item.videos_required,
    reward_amount: item.reward_amount,
    description: item.description,
    enabled: item.enabled
  }
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
}

const saveItem = async () => {
  if (!formData.value.miniapp_id || !formData.value.workflow_id) {
    showMessage('请选择小程序和工作流', 'error')
    return
  }

  if (formData.value.videos_required < 1 || formData.value.reward_amount < 1) {
    showMessage('视频数和奖励数量必须大于0', 'error')
    return
  }

  loading.value = true
  try {
    const payload = {
      miniapp_id: parseInt(formData.value.miniapp_id),
      workflow_id: parseInt(formData.value.workflow_id),
      reward_type: formData.value.reward_type,
      videos_required: formData.value.videos_required,
      reward_amount: formData.value.reward_amount,
      description: formData.value.description,
      enabled: formData.value.enabled
    }

    const url = isEdit.value ? `/admin/miniapp-reward/${formData.value.id}` : '/admin/miniapp-reward'
    const response = isEdit.value 
      ? await api.put(url, payload)
      : await api.post(url, payload)

    if (response.code === 0) {
      showMessage(isEdit.value ? '更新成功' : '添加成功', 'success')
      closeModal()
      await fetchList()
    } else {
      showMessage(response.message || '保存失败', 'error')
    }
  } catch (error: any) {
    console.error('保存失败:', error)
    showMessage(error.response?.data?.message || '保存失败', 'error')
  } finally {
    loading.value = false
  }
}

const toggleStatus = async (item: RewardConfig) => {
  try {
    const response = await api.put(`/admin/miniapp-reward/${item.id}`, {
      enabled: !item.enabled
    })
    if (response.code === 0) {
      showMessage(item.enabled ? '已禁用' : '已启用', 'success')
      await fetchList()
    } else {
      showMessage(response.message || '操作失败', 'error')
    }
  } catch (error: any) {
    console.error('操作失败:', error)
    showMessage(error.response?.data?.message || '操作失败', 'error')
  }
}

const confirmDelete = (item: RewardConfig) => {
  deleteTarget.value = item
  showDeleteDialog.value = true
}

const handleDelete = async () => {
  if (!deleteTarget.value) return

  deleting.value = true
  try {
    const response = await api.delete(`/admin/miniapp-reward/${deleteTarget.value.id}`)
    if (response.code === 0) {
      showMessage('删除成功', 'success')
      showDeleteDialog.value = false
      deleteTarget.value = null
      await fetchList()
    } else {
      showMessage(response.message || '删除失败', 'error')
    }
  } catch (error: any) {
    console.error('删除失败:', error)
    showMessage(error.response?.data?.message || '删除失败', 'error')
  } finally {
    deleting.value = false
  }
}

const getRewardTypeName = (type: string) => {
  const names: Record<string, string> = {
    quota: '邮箱配额',
    mailbox: '邮箱账号',
    milk_coin: '奶片'
  }
  return names[type] || type
}

const getRewardTypeClass = (type: string) => {
  const classes: Record<string, string> = {
    quota: 'bg-blue-100 text-blue-800',
    mailbox: 'bg-green-100 text-green-800',
    milk_coin: 'bg-purple-100 text-purple-800'
  }
  return classes[type] || 'bg-gray-100 text-gray-800'
}
</script>
