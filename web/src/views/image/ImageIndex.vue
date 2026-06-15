<script setup>
const features = [
  { title: '灰度化',          path: '/image/grayscale',   desc: '6 种灰度算法实时切换',         status: 'done' },
  { title: '反色 / 负片',     path: '/image/invert',      desc: 'RGB 通道 255 反向，生成底片风格', status: 'done' },
  { title: '亮度 / 对比度',   path: '/image/brightness',  desc: '滑块控制实时调色',              status: 'done' },
  { title: '二值化阈值',      path: '/image/threshold',   desc: '设定阈值只保留黑白两色',         status: 'done' },
  { title: '单色滤镜',        path: '/image/color-filter', desc: '复古泛黄、冷色调等 7 种滤镜',   status: 'done' },
]
</script>

<template>
  <div class="index-page">
    <h2>🎨 第一部分：单像素画笔</h2>
    <p class="subtitle"><strong>给单像素上色</strong>——从灰度化到单色滤镜，最基础的图像处理工具集。</p>

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
.index-page {
  padding: 40px;
}
h2 { margin-bottom: 8px; }
.subtitle {
  color: #888;
  margin-bottom: 30px;
}
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
.card:hover {
  border-color: #3b5bdb;
  transform: translateY(-2px);
}
.card.is-todo {
  opacity: 0.5;
  pointer-events: none;
}
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}
.title {
  font-size: 16px;
  font-weight: 600;
}
.badge {
  font-size: 12px;
  padding: 2px 8px;
  border-radius: 4px;
}
.badge.done {
  background: #2f9e44;
  color: #fff;
}
.badge.todo {
  background: #495057;
  color: #ccc;
}
.desc {
  color: #888;
  font-size: 13px;
  line-height: 1.5;
}
</style>
