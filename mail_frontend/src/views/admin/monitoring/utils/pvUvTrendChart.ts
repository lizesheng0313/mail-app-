import * as echarts from 'echarts'

export interface PageAnalyticsData {
  daily_stats: Array<{
    date: string
    pv: number
    uv: number
  }>
}

// 更新PV/UV趋势图
export const updatePvUvChart = (container: HTMLDivElement, pageAnalytics: PageAnalyticsData, pvUvChart: echarts.ECharts | null) => {
  if (!container || !pageAnalytics.daily_stats || container.clientWidth === 0) return null

  // 销毁旧图表
  if (pvUvChart) {
    pvUvChart.dispose()
  }

  pvUvChart = echarts.init(container)

  const data = pageAnalytics.daily_stats

  const option = {
    title: {
      text: 'PV/UV趋势',
      left: 'center',
      textStyle: {
        fontSize: 14,
        fontWeight: 'normal'
      }
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'cross'
      }
    },
    legend: {
      data: ['PV', 'UV'],
      bottom: 0
    },
    grid: {
      left: '10%',
      right: '5%',
      top: '15%',
      bottom: '20%',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      data: data.map((item: any) => item.date),
      axisLabel: {
        rotate: 0
      }
    },
    yAxis: {
      type: 'value'
    },
    series: [
      {
        name: 'PV',
        type: 'line',
        data: data.map((item: any) => item.pv),
        smooth: true,
        lineStyle: {
          color: '#3b82f6'
        }
      },
      {
        name: 'UV',
        type: 'line',
        data: data.map((item: any) => item.uv),
        smooth: true,
        lineStyle: {
          color: '#10b981'
        }
      }
    ]
  }

  pvUvChart.setOption(option)
  
  // 确保图表自适应容器大小
  setTimeout(() => {
    if (pvUvChart) {
      pvUvChart.resize()
    }
  }, 100)

  return pvUvChart
}
