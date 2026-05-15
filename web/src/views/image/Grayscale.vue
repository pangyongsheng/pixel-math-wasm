<script setup>
import { ref, onMounted } from 'vue'
import init, { grayscale } from 'pixel-math-wasm'

const originalCanvasRef = ref(null)
const grayscaleCanvasRef = ref(null)
const isLoaded = ref(false)

onMounted(async () => {
  await init()
  isLoaded.value = true
})

function handleFileChange(e) {
  const file = e.target.files[0]
  if (!file) return

  const reader = new FileReader()
  reader.onload = (event) => {
    const img = new Image()
    img.onload = () => processImage(img)
    img.src = event.target.result
  }
  reader.readAsDataURL(file)
}

function processImage(img) {
  // 原图
  const originalCanvas = originalCanvasRef.value
  const originalCtx = originalCanvas.getContext('2d')
  originalCanvas.width = img.width
  originalCanvas.height = img.height
  originalCtx.drawImage(img, 0, 0)

  // 灰度图
  const grayscaleCanvas = grayscaleCanvasRef.value
  const grayscaleCtx = grayscaleCanvas.getContext('2d')
  grayscaleCanvas.width = img.width
  grayscaleCanvas.height = img.height
  grayscaleCtx.drawImage(img, 0, 0)

  const imageData = originalCtx.getImageData(0, 0, img.width, img.height)
  const result = grayscale(imageData.data, img.width, img.height)
  grayscaleCtx.putImageData(new ImageData(new Uint8ClampedArray(result), img.width, img.height), 0, 0)
}
</script>

<template>
  <div class="grayscale-page">
    <h2>灰度化处理</h2>
    <p class="desc">使用加权灰度算法，将彩色图像转换为黑白灰度图</p>

    <div class="controls">
      <label class="upload-btn">
        <input type="file" accept="image/*" @change="handleFileChange" />
        <span v-if="!isLoaded">WASM 加载中...</span>
        <span v-else>选择图片</span>
      </label>
    </div>

    <div class="canvas-wrapper">
      <div class="canvas-item">
        <div class="canvas-label">原图</div>
        <canvas ref="originalCanvasRef"></canvas>
      </div>
      <div class="canvas-item">
        <div class="canvas-label">灰度图</div>
        <canvas ref="grayscaleCanvasRef"></canvas>
      </div>
    </div>
  </div>
</template>

<style scoped>
.grayscale-page {
  padding: 40px;
}
h2 {
  margin-bottom: 10px;
}
.desc {
  color: #888;
  margin-bottom: 30px;
}
.controls {
  margin-bottom: 30px;
}
.upload-btn {
  display: inline-block;
  padding: 10px 24px;
  background: #3b5bdb;
  color: #fff;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
}
.upload-btn:hover {
  background: #3450d1;
}
.upload-btn input {
  display: none;
}
.canvas-wrapper {
  display: flex;
  gap: 40px;
  flex-wrap: wrap;
}
.canvas-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}
.canvas-label {
  color: #888;
  font-size: 14px;
}
canvas {
  max-width: 300px;
  max-height: 300px;
  object-fit: contain;
  background: #1a1a2e;
  border-radius: 8px;
  padding: 8px;
}
</style>