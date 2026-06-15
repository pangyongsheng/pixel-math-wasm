<script setup>
const features = [
  { title: '在线线性代数计算器',   path: '/linear/matrix',        desc: '矩阵加减乘 + RGB↔YPbPr 调亮度（不偏色）',     status: 'todo' },
  { title: '向量计算工具',         path: '/linear/vector',        desc: '点积、夹角、投影 + 两图余弦相似度',           status: 'todo' },
  { title: 'RGB 色彩矩阵变换',     path: '/linear/color-matrix',  desc: '任务 5 升华 + 自定义 3×3 矩阵编辑器',         status: 'todo' },
  { title: 'PCA 主成分分析',       path: '/linear/pca',           desc: '对图像像素做 PCA 降维，保留主要特征',         status: 'todo' },
  { title: '最小二乘法线性回归',   path: '/linear/regression',    desc: '图像去噪 + 颜色校准（两个 demo）',            status: 'todo' },
]
</script>

<template>
  <div class="index-page">
    <h2>🔢 第五部分：图像里的矩阵世界</h2>
    <p class="subtitle"><strong>线代计算工具集</strong>——在线算矩阵、向量、PCA 压缩、线性回归。</p>

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
