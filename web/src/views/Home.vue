<template>
  <div class="home">
    <h1>Pixel Math WASM</h1>
    <p>高性能图像处理与数学算法库</p>
    <div class="features">
      <div class="feature-card">
        <span class="icon">🎨</span>
        <h3>图像滤镜</h3>
        <p>灰度、反色、模糊、边缘检测等</p>
      </div>
      <div class="feature-card">
        <span class="icon">🔄</span>
        <h3>几何变换</h3>
        <p>翻转、旋转、缩放、斜切等</p>
      </div>
      <div class="feature-card">
        <span class="icon">🌀</span>
        <h3>分形渲染</h3>
        <p>曼德博、Julia 分形</p>
      </div>
      <div class="feature-card">
        <span class="icon">📐</span>
        <h3>线性代数</h3>
        <p>矩阵、向量、PCA降维</p>
      </div>
      <div class="feature-card">
        <span class="icon">🎲</span>
        <h3>概率统计</h3>
        <p>噪声生成、随机游走、K-Means</p>
      </div>
    </div>
    <div class="tech">
      <span>Rust</span>
      <span>+</span>
      <span>WebAssembly</span>
      <span>+</span>
      <span>Vue</span>
    </div>
  </div>
</template>

<style scoped>
.home {
  padding: 60px;
  text-align: center;
}
h1 {
  font-size: 48px;
  margin-bottom: 10px;
}
.features {
  display: flex;
  justify-content: center;
  gap: 20px;
  margin-top: 60px;
  flex-wrap: wrap;
}
.feature-card {
  background: #1a1a2e;
  padding: 30px;
  border-radius: 12px;
  width: 180px;
}
.icon {
  font-size: 40px;
}
.tech {
  margin-top: 60px;
  display: flex;
  justify-content: center;
  gap: 15px;
  color: #888;
}
</style>