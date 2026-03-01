import * as echarts from 'echarts'

export interface MailboxStatsData {
  daily_mailbox_stats: Array<{
    date: string
    new_mailboxes: number
    user_mailboxes: number
    temp_mailboxes: number
  }>
}

// 更新邮箱申请趋势图表
export const updateMailboxChart = (container: HTMLDivElement, userStats: MailboxStatsData, mailboxChart: echarts.ECharts | null) => {
  if (!container || !userStats.daily_mailbox_stats || container.clientWidth === 0) return null

  // 销毁旧图表
  if (mailboxChart) {
    mailboxChart.dispose()
  }

  mailboxChart = echarts.init(container)
  
  const data = userStats.daily_mailbox_stats.slice().reverse()
  
  const option = {
    grid: {
      left: '10%',
      right: '5%',
      top: '20%',      // 为图例留更多空间
      bottom: '15%'
    },
    tooltip: {
      trigger: 'axis',
      formatter: function(params: any) {
        let result = params[0].name + '<br/>'
        params.forEach((param: any) => {
          result += param.marker + param.seriesName + ': ' + param.value + '个<br/>'
        })
        return result
      }
    },
    legend: {
      data: ['总申请', '用户邮箱', '临时邮箱'],
      top: 10
    },
    xAxis: {
      type: 'category',
      data: data.map((item: any) => item.date),
      axisLine: {
        lineStyle: {
          color: '#e5e7eb'
        }
      }
    },
    yAxis: {
      type: 'value',
      minInterval: 1, // 确保只显示整数
      axisLine: {
        lineStyle: {
          color: '#e5e7eb'
        }
      }
    },
    series: [
      {
        name: '总申请',
        type: 'line',
        data: data.map((item: any) => item.new_mailboxes),
        smooth: true,
        lineStyle: {
          color: '#10b981'
        }
      },
      {
        name: '用户邮箱',
        type: 'line',
        data: data.map((item: any) => item.user_mailboxes),
        smooth: true,
        lineStyle: {
          color: '#8b4513'
        }
      },
      {
        name: '临时邮箱',
        type: 'line',
        data: data.map((item: any) => item.temp_mailboxes),
        smooth: true,
        lineStyle: {
          color: '#6b7280'
        }
      }
    ]
  }
  
  mailboxChart.setOption(option)
  
  // 确保图表自适应容器大小
  setTimeout(() => {
    if (mailboxChart) {
      mailboxChart.resize()
    }
  }, 100)

  return mailboxChart
}
