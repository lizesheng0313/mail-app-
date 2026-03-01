import * as echarts from 'echarts'

export interface UserStatsData {
  daily_new_users: Array<{
    date: string
    new_users: number
  }>
}

// 更新用户趋势图表
export const updateUserChart = (container: HTMLDivElement, userStats: UserStatsData, userChart: echarts.ECharts | null) => {
  if (!container || !userStats.daily_new_users || container.clientWidth === 0) return null

  // 销毁旧图表
  if (userChart) {
    userChart.dispose()
  }

  userChart = echarts.init(container)

  const data = userStats.daily_new_users.slice().reverse()

  const option = {
    grid: {
      left: '10%',      // 使用百分比，适应不同容器大小
      right: '5%',      // 使用百分比
      top: '10%',       // 使用百分比
      bottom: '15%'     // 使用百分比，为X轴标签留足空间
    },
    tooltip: {
      trigger: 'axis',
      formatter: '{b}<br/>{a}: {c}人'
    },
    xAxis: {
      type: 'category',
      data: data.map((item: any) => item.date),
      axisLine: {
        lineStyle: {
          color: '#e5e7eb'
        }
      },
      axisLabel: {
        rotate: 0,  // 保持水平显示
        margin: 10
      }
    },
    yAxis: {
      type: 'value',
      minInterval: 1, // 确保只显示整数
      axisLine: {
        lineStyle: {
          color: '#e5e7eb'
        }
      },
      axisLabel: {
        margin: 10   // 增加标签与轴线的距离
      }
    },
    series: [{
      name: '新用户',
      type: 'line',
      data: data.map((item: any) => item.new_users),
      smooth: true,
      lineStyle: {
        color: '#3b82f6'
      },
      areaStyle: {
        color: {
          type: 'linear',
          x: 0,
          y: 0,
          x2: 0,
          y2: 1,
          colorStops: [{
            offset: 0, color: 'rgba(59, 130, 246, 0.2)'
          }, {
            offset: 1, color: 'rgba(59, 130, 246, 0.05)'
          }]
        }
      }
    }]
  }

  userChart.setOption(option)

  // 确保图表自适应容器大小
  setTimeout(() => {
    if (userChart) {
      userChart.resize()
    }
  }, 100)

  return userChart
}
