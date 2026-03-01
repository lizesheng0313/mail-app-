/**
 * 时间处理工具函数
 * 前端统一处理时间格式化和时区转换
 */

/**
 * 将毫秒时间戳转换为本地时间字符串
 * @param {number} timestamp - 毫秒时间戳
 * @param {string} format - 格式化选项 'datetime' | 'date' | 'time'
 * @returns {string} 格式化后的时间字符串
 */
export function formatTimestamp(timestamp, format = 'datetime') {
  if (!timestamp) return ''
  
  const date = new Date(timestamp)
  
  // 检查时间是否有效
  if (isNaN(date.getTime())) return ''
  
  switch (format) {
    case 'date':
      return date.toLocaleDateString()
    case 'time':
      return date.toLocaleTimeString()
    case 'datetime':
    default:
      return date.toLocaleString()
  }
}

/**
 * 将毫秒时间戳转换为相对时间字符串 (如：2分钟前)
 * @param {number} timestamp - 毫秒时间戳
 * @returns {string} 相对时间字符串
 */
export function formatRelativeTime(timestamp) {
  if (!timestamp) return ''
  
  const now = Date.now()
  const diff = now - timestamp
  
  if (diff < 0) return '未来时间'
  
  const seconds = Math.floor(diff / 1000)
  const minutes = Math.floor(seconds / 60)
  const hours = Math.floor(minutes / 60)
  const days = Math.floor(hours / 24)
  const weeks = Math.floor(days / 7)
  const months = Math.floor(days / 30)
  const years = Math.floor(days / 365)
  
  if (seconds < 60) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  if (hours < 24) return `${hours}小时前`
  if (days < 7) return `${days}天前`
  if (weeks < 4) return `${weeks}周前`
  if (months < 12) return `${months}个月前`
  return `${years}年前`
}

/**
 * 将本地时间转换为毫秒时间戳
 * @param {Date|string} date - 日期对象或日期字符串
 * @returns {number} 毫秒时间戳
 */
export function dateToTimestamp(date) {
  if (!date) return null
  
  const dateObj = date instanceof Date ? date : new Date(date)
  
  if (isNaN(dateObj.getTime())) return null
  
  return dateObj.getTime()
}

/**
 * 获取当前时区偏移量（分钟）
 * @returns {number} 时区偏移量（分钟）
 */
export function getTimezoneOffset() {
  return new Date().getTimezoneOffset()
}

/**
 * 获取当前时区名称
 * @returns {string} 时区名称
 */
export function getTimezoneName() {
  return Intl.DateTimeFormat().resolvedOptions().timeZone
}

/**
 * 格式化持续时间（毫秒）
 * @param {number} duration - 持续时间（毫秒）
 * @returns {string} 格式化后的持续时间
 */
export function formatDuration(duration) {
  if (!duration || duration === 0) return '0ms'
  
  if (duration < 1000) return `${duration}ms`
  if (duration < 60000) return `${(duration / 1000).toFixed(1)}s`
  if (duration < 3600000) return `${(duration / 60000).toFixed(1)}分钟`
  return `${(duration / 3600000).toFixed(1)}小时`
}

/**
 * 检查时间戳是否在指定范围内
 * @param {number} timestamp - 要检查的时间戳
 * @param {string} range - 范围 '1h' | '24h' | '7d' | '30d'
 * @returns {boolean} 是否在范围内
 */
export function isWithinTimeRange(timestamp, range) {
  if (!timestamp) return false
  
  const now = Date.now()
  const diff = now - timestamp
  
  switch (range) {
    case '1h':
      return diff <= 60 * 60 * 1000
    case '24h':
      return diff <= 24 * 60 * 60 * 1000
    case '7d':
      return diff <= 7 * 24 * 60 * 60 * 1000
    case '30d':
      return diff <= 30 * 24 * 60 * 60 * 1000
    default:
      return true
  }
}

/**
 * 将时间戳转换为日期范围的开始和结束时间
 * @param {string} range - 范围 'today' | 'yesterday' | 'week' | 'month'
 * @returns {object} {start: number, end: number} 开始和结束时间戳
 */
export function getTimeRangeBounds(range) {
  const now = new Date()
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate())
  
  switch (range) {
    case 'today':
      return {
        start: today.getTime(),
        end: today.getTime() + 24 * 60 * 60 * 1000 - 1
      }
    case 'yesterday':
      const yesterday = new Date(today.getTime() - 24 * 60 * 60 * 1000)
      return {
        start: yesterday.getTime(),
        end: yesterday.getTime() + 24 * 60 * 60 * 1000 - 1
      }
    case 'week':
      const weekStart = new Date(today.getTime() - (now.getDay() * 24 * 60 * 60 * 1000))
      return {
        start: weekStart.getTime(),
        end: weekStart.getTime() + 7 * 24 * 60 * 60 * 1000 - 1
      }
    case 'month':
      const monthStart = new Date(now.getFullYear(), now.getMonth(), 1)
      const nextMonth = new Date(now.getFullYear(), now.getMonth() + 1, 1)
      return {
        start: monthStart.getTime(),
        end: nextMonth.getTime() - 1
      }
    default:
      return {
        start: 0,
        end: Date.now()
      }
  }
}