<script setup>
const features = [
  { title: 'K-Means 图像像素聚类',     path: '/synthesis/kmeans',      desc: '用向量距离+概率迭代，把图片颜色聚类成少数几种',   status: 'todo' },
  { title: '简易朴素贝叶斯分类器',     path: '/synthesis/naive-bayes', desc: 'WASM 实现条件概率分类，做简单文本/图像区域分类',   status: 'todo' },
  { title: '2D 粒子物理引擎',          path: '/synthesis/particle',    desc: '向量做位置速度、概率生成粒子、引力碰撞模拟',     status: 'todo' },
]
</script>

<template>
  <div class="index-page">
    <h2>🚀 第七部分：图像里的世界：动画、物理与智能</h2>
    <p class="subtitle"><strong>线代 + 概率 + 物理的终极融合</strong>——图像聚类、AI 分类、2D 粒子物理引擎。</p>

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
