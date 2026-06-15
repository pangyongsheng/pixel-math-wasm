<script setup>
import { ref } from 'vue'

// 7 个分部与 README/docs 完全对齐
const menuItems = [
  {
    title: '第一部分：单像素画笔',
    path: '/image',
    icon: '🎨',
    children: [
      { title: '灰度化', path: '/image/grayscale' },
      { title: '反色 / 负片', path: '/image/invert' },
      { title: '亮度 / 对比度', path: '/image/brightness' },
      { title: '二值化阈值', path: '/image/threshold' },
      { title: '单色滤镜', path: '/image/color-filter' },
    ]
  },
  {
    title: '第二部分：邻域卷积与图像特效',
    path: '/convolve',
    icon: '🌀',
    children: [
      { title: '均值模糊 & 中值模糊', path: '/image/blur' },
      { title: '马赛克', path: '/image/mosaic' },
      { title: '暗角 & 复古胶片', path: '/image/vignette' },
      { title: 'Sobel 边缘', path: '/image/sobel' },
      { title: '浮雕', path: '/image/emboss' },
      { title: 'RGB 通道偏移', path: '/image/rgb-shift' },
    ]
  },
  {
    title: '第三部分：图像几何变换',
    path: '/transform',
    icon: '📐',
    children: [
      { title: '翻转', path: '/transform/flip' },
      { title: '旋转', path: '/transform/rotate' },
      { title: '缩放', path: '/transform/scale' },
      { title: '斜切', path: '/transform/shear' },
      { title: '鱼眼', path: '/transform/fisheye' },
    ]
  },
  {
    title: '第四部分：分形画板与交互',
    path: '/fractal',
    icon: '✨',
    children: [
      { title: '曼德博集合', path: '/fractal/mandelbrot' },
      { title: 'Julia 分形', path: '/fractal/julia' },
      { title: '分形缩放与拖拽', path: '/fractal/canvas' },
    ]
  },
  {
    title: '第五部分：矩阵世界（线代工具箱）',
    path: '/linear',
    icon: '🔢',
    children: [
      { title: '矩阵计算器', path: '/linear/matrix' },
      { title: '向量工具', path: '/linear/vector' },
      { title: '色彩矩阵', path: '/linear/color-matrix' },
      { title: 'PCA 降维', path: '/linear/pca' },
      { title: '最小二乘回归', path: '/linear/regression' },
    ]
  },
  {
    title: '第六部分：概率的图像魔法',
    path: '/probability',
    icon: '🎲',
    children: [
      { title: '分布计算', path: '/probability/distribution' },
      { title: '高斯噪声', path: '/probability/gaussian-noise' },
      { title: '椒盐噪声', path: '/probability/salt-pepper' },
      { title: '自适应二值化', path: '/probability/adaptive-threshold' },
      { title: '随机游走', path: '/probability/random-walk' },
    ]
  },
  {
    title: '第七部分：终极综合',
    path: '/synthesis',
    icon: '🚀',
    children: [
      { title: 'K-Means 聚类', path: '/synthesis/kmeans' },
      { title: '朴素贝叶斯', path: '/synthesis/naive-bayes' },
      { title: '粒子物理引擎', path: '/synthesis/particle' },
    ]
  },
]

const expandedMenus = ref([])

function toggleMenu(path) {
  const index = expandedMenus.value.indexOf(path)
  if (index > -1) {
    expandedMenus.value.splice(index, 1)
  } else {
    expandedMenus.value.push(path)
  }
}
</script>

<template>
  <div class="layout">
    <aside class="sidebar">
      <div class="logo">Pixel Math</div>
      <nav>
        <div v-for="item in menuItems" :key="item.path">
          <div
            class="menu-item"
            :class="{ expanded: expandedMenus.includes(item.path) }"
            @click.stop="toggleMenu(item.path)"
          >
            <span class="icon">{{ item.icon }}</span>
            <router-link :to="item.path">{{ item.title }}</router-link>
            <span class="arrow">{{ expandedMenus.includes(item.path) ? '▼' : '▶' }}</span>
          </div>
          <div v-if="expandedMenus.includes(item.path)" class="submenu">
            <router-link
              v-for="child in item.children"
              :key="child.path"
              :to="child.path"
              class="submenu-item"
            >
              {{ child.title }}
            </router-link>
          </div>
        </div>
      </nav>
    </aside>
    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<style scoped>
.layout {
  display: flex;
  height: 100vh;
}
.sidebar {
  width: 280px;
  background: #1a1a2e;
  color: #fff;
  overflow-y: auto;
}
.logo {
  padding: 20px;
  font-size: 20px;
  font-weight: bold;
  border-bottom: 1px solid #333;
}
.menu-item {
  padding: 12px 20px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 13px;
}
.menu-item:hover {
  background: #252545;
}
.menu-item a {
  color: #fff;
  text-decoration: none;
  flex: 1;
}
.arrow {
  font-size: 10px;
  color: #888;
}
.submenu {
  background: #16162a;
}
.submenu-item {
  padding: 10px 20px 10px 50px;
  color: #aaa;
  text-decoration: none;
  display: block;
  font-size: 13px;
}
.submenu-item:hover,
.submenu-item.router-link-active {
  color: #fff;
  background: #252545;
}
.main-content {
  flex: 1;
  overflow-y: auto;
  background: #0f0f1a;
  color: #fff;
}
</style>
