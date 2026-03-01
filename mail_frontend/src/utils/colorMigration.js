/**
 * 颜色迁移工具 - 批量替换绿色为新配色
 * 使用说明：
 * 1. bg-green-xxx → bg-primary-xxx（主色）
 * 2. text-green-xxx → text-primary-xxx
 * 3. border-green-xxx → border-primary-xxx
 * 4. ring-green-xxx → ring-primary-xxx
 * 5. 需要强调的按钮（如 CTA）→ bg-accent-xxx（橙色）
 */

export const colorMapping = {
  // 背景色映射
  'bg-green-50': 'bg-primary-50',
  'bg-green-100': 'bg-primary-100',
  'bg-green-500': 'bg-primary-500',
  'bg-green-600': 'bg-primary-600',
  'bg-green-700': 'bg-primary-700',
  
  // CTA 按钮使用强调色
  'bg-green-600 hover:bg-green-700': 'bg-accent-500 hover:bg-accent-600',
  
  // 文字色映射
  'text-green-50': 'text-primary-50',
  'text-green-100': 'text-primary-100',
  'text-green-500': 'text-primary-500',
  'text-green-600': 'text-primary-600',
  'text-green-700': 'text-primary-700',
  'text-green-800': 'text-primary-800',
  
  // 边框色映射
  'border-green-200': 'border-primary-200',
  'border-green-300': 'border-primary-300',
  'border-green-500': 'border-primary-500',
  'border-green-600': 'border-primary-600',
  
  // Ring映射
  'ring-green-500': 'ring-primary-500',
  'ring-green-200': 'ring-primary-200',
  
  // 特殊组合
  'from-green-600 to-emerald-600': 'from-primary-600 to-primary-700',
  'from-green-500 to-emerald-600': 'from-primary-500 to-primary-600',
  'from-green-400 to-emerald-600': 'from-primary-400 to-primary-600',
}

/**
 * 自动替换文本中的颜色类
 */
export function migrateColors(text) {
  let result = text
  
  for (const [oldColor, newColor] of Object.entries(colorMapping)) {
    result = result.replace(new RegExp(oldColor, 'g'), newColor)
  }
  
  return result
}

/**
 * 颜色使用指南
 */
export const colorGuide = {
  // 主色（深蓝灰）- 用于主要UI元素
  primary: {
    description: '主色 - 深蓝灰色，用于主要按钮、链接、重要文字',
    usage: [
      'bg-primary-600 - 主按钮背景',
      'text-primary-900 - 主要文字',
      'border-primary-200 - 边框'
    ]
  },
  
  // 强调色（橙色）- 用于CTA和重要操作
  accent: {
    description: '强调色 - 橙色，用于 CTA 按钮、重要通知、强调元素',
    usage: [
      'bg-accent-500 - CTA 按钮',
      'text-accent-600 - 重要提示文字',
      'border-accent-400 - 强调边框'
    ]
  },
  
  // 成功色（青色）
  success: {
    description: '成功色 - 青色，用于成功状态、正面反馈',
    usage: [
      'bg-success-600 - 成功按钮',
      'text-success-600 - 成功文字',
      'border-success-400 - 成功边框'
    ]
  },
  
  // 警告色（琥珀色）
  warning: {
    description: '警告色 - 琥珀色，用于警告信息、需要注意的内容',
    usage: [
      'bg-warning-500 - 警告按钮',
      'text-warning-600 - 警告文字',
      'border-warning-400 - 警告边框'
    ]
  },
  
  // 危险色（红色）
  danger: {
    description: '危险色 - 红色，用于错误、删除等危险操作',
    usage: [
      'bg-danger-600 - 危险按钮',
      'text-danger-600 - 错误文字',
      'border-danger-400 - 错误边框'
    ]
  }
}

console.log('🎨 颜色系统已加载')
console.log('主色：深蓝灰（Slate）')
console.log('强调色：橙色（Orange）')
console.log('成功色：青色（Cyan）')
console.log('警告色：琥珀色（Amber）')
console.log('危险色：红色（Red）')
