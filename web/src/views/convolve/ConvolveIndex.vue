<script setup>
const features = [
  { title: '均值模糊 & 中值模糊', path: '/image/blur',      desc: '3x3 邻域平均做平滑模糊；中值滤波做图片去噪', status: 'done' },
  { title: '马赛克',                path: '/image/mosaic',   desc: '分块取平均色实现打码像素风格',                status: 'done' },
  { title: '暗角 & 复古胶片',       path: '/image/vignette', desc: '四周亮度衰减，中心高亮模拟相机暗角',          status: 'done' },
  { title: 'Sobel 边缘检测',        path: '/image/sobel',    desc: '用卷积核矩阵提取图像轮廓线条',                status: 'done' },
  { title: '浮雕 / 雕刻滤镜',       path: '/image/emboss',   desc: '邻域像素差值生成凹凸浮雕效果',                status: 'done' },
  { title: 'RGB 通道偏移',          path: '/image/rgb-shift', desc: '故障风 RGB 错位偏移，模拟电子故障画面',     status: 'done' },
]
</script>

<template>
  <div class="index-page">
    <h2>🌀 第二部分：邻域卷积与图像特效</h2>
    <p class="subtitle"><strong>给图片加特效</strong>——模糊、马赛克、浮雕、故障风等视觉效果。</p>

    <div class="grid">
      <router-link
        v-for="item in features"
        :key="item.path"
        :to="item.path"
        class="card"
        :class="{ 'is-todo': item.status === 'todo' }"
      >
        <div class="card-header">
          <span class="title">{{ item.title }}</span>
          <span v-if="item.status === 'done'" class="badge done">✓ 已完成</span>
          <span v-else class="badge todo">规划中</span>
        </div>
        <div class="desc">{{ item.desc }}</div>
      </router-link>
    </div>
  </div>
</template>

<style scoped>
.index-page { padding: 40px; }
h2 { margin-bottom: 8px; }
.subtitle { color: #888; margin-bottom: 30px; }
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 16px;
}
.card {
  display: block;
  padding: 20px;
  background: #1a1a2e;
  border-radius: 8px;
  text-decoration: none;
  color: #fff;
  border: 1px solid transparent;
  transition: all 0.2s;
}
.card:hover { border-color: #3b5bdb; transform: translateY(-2px); }
.card.is-todo { opacity: 0.5; pointer-events: none; }
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}
.title { font-size: 16px; font-weight: 600; }
.badge { font-size: 12px; padding: 2px 8px; border-radius: 4px; }
.badge.done { background: #2f9e44; color: #fff; }
.badge.todo { background: #495057; color: #ccc; }
.desc { color: #888; font-size: 13px; line-height: 1.5; }
</style>
