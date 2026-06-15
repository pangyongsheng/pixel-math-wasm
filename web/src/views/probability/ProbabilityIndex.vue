<script setup>
const features = [
  { title: '常见分布计算器',         path: '/probability/distribution',       desc: '正态分布、二项分布、泊松分布概率密度/累积概率计算', status: 'todo' },
  { title: '图片高斯/椒盐噪声生成', path: '/probability/gaussian-noise',   desc: '用正态分布、随机概率给图片加噪点',                  status: 'todo' },
  { title: '自适应阈值二值化',       path: '/probability/adaptive-threshold', desc: '基于图像灰度直方图统计概率，自动计算分割阈值',      status: 'todo' },
  { title: '二维随机游走 & 布朗运动', path: '/probability/random-walk',      desc: 'Canvas 粒子随机游走、布朗运动轨迹可视化',           status: 'todo' },
]
</script>

<template>
  <div class="index-page">
    <h2>🎲 第六部分：概率的图像魔法</h2>
    <p class="subtitle"><strong>用概率驱动图像与运动</strong>——加噪、自适应二值化、随机游走、布朗运动。</p>

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
