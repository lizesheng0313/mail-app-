<template>
  <el-dialog
    title="发布工作流到市场"
    :visible.sync="dialogVisible"
    width="700px"
    :close-on-click-modal="false"
    @close="handleClose"
  >
    <el-form
      ref="form"
      :model="formData"
      :rules="rules"
      label-width="120px"
    >
      <el-form-item label="工作流名称">
        <el-input :value="workflowName" disabled />
      </el-form-item>

      <el-form-item label="分类" prop="category">
        <el-select v-model="formData.category" placeholder="请选择分类">
          <el-option label="自动化" value="automation" />
          <el-option label="数据处理" value="data" />
          <el-option label="通知提醒" value="notification" />
          <el-option label="集成对接" value="integration" />
          <el-option label="其他" value="other" />
        </el-select>
      </el-form-item>

      <el-form-item label="标签">
        <el-tag
          v-for="tag in formData.tags"
          :key="tag"
          closable
          :disable-transitions="false"
          @close="handleRemoveTag(tag)"
          style="margin-right: 8px"
        >
          {{ tag }}
        </el-tag>
        <el-input
          v-if="tagInputVisible"
          ref="tagInput"
          v-model="tagInputValue"
          class="tag-input"
          size="small"
          @keyup.enter.native="handleAddTag"
          @blur="handleAddTag"
        />
        <el-button
          v-else
          size="small"
          @click="showTagInput"
        >
          + 添加标签
        </el-button>
      </el-form-item>

      <el-form-item label="定价模式" prop="pricingModel">
        <el-radio-group v-model="formData.pricingModel">
          <el-radio label="free">免费</el-radio>
          <el-radio label="per_use">按次付费</el-radio>
          <el-radio label="one_time">一次性购买</el-radio>
          <el-radio label="subscription">订阅制</el-radio>
        </el-radio-group>
      </el-form-item>

      <el-form-item
        v-if="formData.pricingModel !== 'free'"
        label="奶币价格"
        prop="milkCoinPrice"
      >
        <el-input-number
          v-model="formData.milkCoinPrice"
          :min="0"
          :step="1"
          :precision="2"
        />
        <span class="price-hint">（1奶币 = 1元）</span>
      </el-form-item>

      <el-form-item label="图标URL">
        <el-input v-model="formData.iconUrl" placeholder="请输入图标URL" />
      </el-form-item>

      <el-form-item label="截图URL">
        <el-input
          v-model="screenshotInput"
          placeholder="请输入截图URL，按回车添加"
          @keyup.enter.native="handleAddScreenshot"
        >
          <el-button slot="append" @click="handleAddScreenshot">添加</el-button>
        </el-input>
        <div v-if="formData.screenshots.length > 0" class="screenshots-list">
          <el-tag
            v-for="(url, index) in formData.screenshots"
            :key="index"
            closable
            @close="handleRemoveScreenshot(index)"
            style="margin: 4px"
          >
            截图{{ index + 1 }}
          </el-tag>
        </div>
      </el-form-item>

      <el-form-item label="详细说明">
        <el-button
          v-if="!formData.longDescription"
          type="text"
          @click="showDescriptionEditor"
        >
          点击编辑详细说明
        </el-button>
        <div v-else class="description-preview">
          <div class="preview-text">{{ descriptionPreview }}</div>
          <el-button type="text" size="small" @click="showDescriptionEditor">
            编辑
          </el-button>
        </div>
      </el-form-item>
    </el-form>

    <div slot="footer">
      <el-button @click="handleClose">取消</el-button>
      <el-button
        type="primary"
        :loading="publishing"
        @click="handlePublish"
      >
        提交审核
      </el-button>
    </div>

    <!-- Markdown编辑器弹窗 -->
    <el-dialog
      title="编辑详细说明"
      :visible.sync="editorVisible"
      width="90%"
      append-to-body
      :close-on-click-modal="false"
    >
      <markdown-editor
        ref="editor"
        :content="formData.longDescription"
        :show-ai-button="true"
        @save="handleSaveDescription"
        @cancel="editorVisible = false"
        @generate-ai="handleGenerateAI"
      />
    </el-dialog>
  </el-dialog>
</template>

<script>
import MarkdownEditor from '@/components/MarkdownEditor/index.vue'
import { workflowApi } from '@/api/workflow'

export default {
  name: 'PublishWorkflowDialog',
  components: {
    MarkdownEditor
  },
  props: {
    visible: {
      type: Boolean,
      default: false
    },
    workflowId: {
      type: String,
      required: true
    },
    workflowName: {
      type: String,
      default: ''
    }
  },
  data() {
    return {
      dialogVisible: this.visible,
      editorVisible: false,
      publishing: false,
      tagInputVisible: false,
      tagInputValue: '',
      screenshotInput: '',
      formData: {
        category: 'automation',
        tags: [],
        pricingModel: 'free',
        milkCoinPrice: 0,
        iconUrl: '',
        screenshots: [],
        longDescription: ''
      },
      rules: {
        category: [
          { required: true, message: '请选择分类', trigger: 'change' }
        ],
        pricingModel: [
          { required: true, message: '请选择定价模式', trigger: 'change' }
        ],
        milkCoinPrice: [
          {
            validator: (rule, value, callback) => {
              if (this.formData.pricingModel !== 'free' && value <= 0) {
                callback(new Error('付费工作流的价格必须大于0'))
              } else {
                callback()
              }
            },
            trigger: 'blur'
          }
        ]
      }
    }
  },
  computed: {
    descriptionPreview() {
      if (!this.formData.longDescription) return ''
      return this.formData.longDescription.substring(0, 100) + '...'
    }
  },
  watch: {
    visible(val) {
      this.dialogVisible = val
    },
    dialogVisible(val) {
      this.$emit('update:visible', val)
    }
  },
  methods: {
    showTagInput() {
      this.tagInputVisible = true
      this.$nextTick(() => {
        this.$refs.tagInput.$refs.input.focus()
      })
    },

    handleAddTag() {
      const tag = this.tagInputValue.trim()
      if (tag && !this.formData.tags.includes(tag)) {
        this.formData.tags.push(tag)
      }
      this.tagInputVisible = false
      this.tagInputValue = ''
    },

    handleRemoveTag(tag) {
      this.formData.tags = this.formData.tags.filter(t => t !== tag)
    },

    handleAddScreenshot() {
      const url = this.screenshotInput.trim()
      if (url && !this.formData.screenshots.includes(url)) {
        this.formData.screenshots.push(url)
        this.screenshotInput = ''
      }
    },

    handleRemoveScreenshot(index) {
      this.formData.screenshots.splice(index, 1)
    },

    showDescriptionEditor() {
      this.editorVisible = true
    },

    handleSaveDescription(content) {
      this.formData.longDescription = content
      this.editorVisible = false
      this.$message.success('详情说明已保存')
    },

    async handleGenerateAI() {
      if (!this.$refs.editor) return

      try {
        this.$refs.editor.setGenerating(true)
        const res = await workflowApi.generateWorkflowDescription(this.workflowId)

        if (res.code === 200 && res.data?.long_description) {
          this.formData.longDescription = res.data.long_description
          this.$refs.editor.setContent(res.data.long_description)
          this.$message.success('AI生成成功')
        } else {
          this.$message.error(res.message || 'AI生成失败')
        }
      } catch (error) {
        this.$message.error('AI生成失败: ' + (error.message || '未知错误'))
      } finally {
        this.$refs.editor.setGenerating(false)
      }
    },

    async handlePublish() {
      if (!this.formData.longDescription) {
        this.$message.warning('请先添加详细说明')
        return
      }

      this.$refs.form.validate(async (valid) => {
        if (!valid) return

        try {
          this.publishing = true
          const res = await workflowApi.publishWorkflowToMarket(this.workflowId, {
            category: this.formData.category,
            tags: this.formData.tags,
            milk_coin_price: this.formData.milkCoinPrice,
            pricing_model: this.formData.pricingModel,
            icon_url: this.formData.iconUrl || null,
            screenshots: this.formData.screenshots,
            long_description: this.formData.longDescription
          })

          if (res.code === 200) {
            this.$message.success('工作流已提交到市场，等待审核')
            this.$emit('success')
            this.handleClose()
          } else {
            this.$message.error(res.message || '发布失败')
          }
        } catch (error) {
          this.$message.error('发布失败: ' + (error.message || '未知错误'))
        } finally {
          this.publishing = false
        }
      })
    },

    handleClose() {
      this.$refs.form.resetFields()
      this.formData = {
        category: 'automation',
        tags: [],
        pricingModel: 'free',
        milkCoinPrice: 0,
        iconUrl: '',
        screenshots: [],
        longDescription: ''
      }
      this.dialogVisible = false
    }
  }
}
</script>

<style scoped>
.tag-input {
  width: 120px;
}

.price-hint {
  margin-left: 8px;
  color: #909399;
  font-size: 12px;
}

.screenshots-list {
  margin-top: 8px;
}

.description-preview {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 12px;
  background: #f5f7fa;
  border-radius: 4px;
}

.description-preview .preview-text {
  flex: 1;
  color: #606266;
  font-size: 13px;
  line-height: 1.5;
}
</style>
