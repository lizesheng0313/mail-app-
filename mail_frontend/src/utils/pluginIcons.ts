/**
 * 插件图标配置
 */

export interface PluginIconConfig {
  name: string
  description: string
  bgGradient: string
  bgSolid: string
  bgLight: string
}

// 插件图标配置映射
export const PLUGIN_ICONS: Record<string, PluginIconConfig> = {
  // 网页爬虫
  web_crawler: {
    name: '网页爬虫',
    description: '自动化网页数据采集',
    bgGradient: 'from-blue-500 to-cyan-600',
    bgSolid: 'bg-blue-500',
    bgLight: 'bg-blue-50 text-blue-600'
  },

  // 邮件内容提取
  email_content_extractor: {
    name: '内容提取',
    description: '提取邮件中的URL、邮箱、电话',
    bgGradient: 'from-purple-500 to-pink-600',
    bgSolid: 'bg-purple-500',
    bgLight: 'bg-purple-50 text-purple-600'
  },

  // 邮件转发器
  email_forwarder: {
    name: '邮件转发',
    description: '自动转发邮件到指定邮箱',
    bgGradient: 'from-green-500 to-emerald-600',
    bgSolid: 'bg-green-500',
    bgLight: 'bg-green-50 text-green-600'
  },

  // 邮件管理中心
  email_manager: {
    name: '邮件管理',
    description: '批量管理邮箱账号和邮件',
    bgGradient: 'from-indigo-500 to-blue-600',
    bgSolid: 'bg-indigo-500',
    bgLight: 'bg-indigo-50 text-indigo-600'
  },

  // 验证码提取器
  verification_code_extractor: {
    name: '验证码提取',
    description: '自动识别邮件中的验证码',
    bgGradient: 'from-red-500 to-orange-600',
    bgSolid: 'bg-red-500',
    bgLight: 'bg-red-50 text-red-600'
  }
}

/**
 * 获取插件图标配置
 */
export const getPluginIconConfig = (pluginId: string): PluginIconConfig => {
  return PLUGIN_ICONS[pluginId] || {
    name: '未知插件',
    description: '',
    bgGradient: 'from-gray-500 to-gray-600',
    bgSolid: 'bg-gray-500',
    bgLight: 'bg-gray-50 text-gray-600'
  }
}

/**
 * 获取插件名称
 */
export const getPluginName = (pluginId: string): string => {
  return PLUGIN_ICONS[pluginId]?.name || pluginId
}
