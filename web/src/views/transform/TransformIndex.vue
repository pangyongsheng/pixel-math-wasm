<script setup>
const features = [
  { title: '图片翻转与旋转', path: '/transform/flip',    desc: '左右/上下翻转、90° 顺时针逆时针旋转', status: 'todo' },
  { title: '任意角度旋转',    path: '/transform/rotate', desc: '自定义角度旋转图片，自动补白边',       status: 'todo' },
  { title: '缩放与斜切畸变', path: '/transform/scale',  desc: '双线性插值缩放、水平/垂直斜切变形',    status: 'todo' },
  { title: '鱼眼广角畸变',    path: '/transform/fisheye', desc: '模拟鱼眼镜头球面扭曲效果',           status: 'todo' },
]
</script>

<template>
  <div class="index-page">
    <h2>📐 第三部分：图像几何变换</h2>
    <p class="subtitle"><strong>像玩相机一样变形</strong>——翻转、旋转、缩放、畸变。</p>

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
