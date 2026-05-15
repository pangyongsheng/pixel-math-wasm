import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    component: () => import('../views/Home.vue'),
  },
  {
    path: '/image',
    component: () => import('../views/image/ImageIndex.vue'),
  },
  {
    path: '/image/grayscale',
    component: () => import('../views/image/Grayscale.vue'),
  },
  {
    path: '/transform',
    component: () => import('../views/transform/TransformIndex.vue'),
  },
  {
    path: '/fractal',
    component: () => import('../views/fractal/FractalIndex.vue'),
  },
  {
    path: '/linear',
    component: () => import('../views/linear/LinearIndex.vue'),
  },
  {
    path: '/probability',
    component: () => import('../views/probability/ProbabilityIndex.vue'),
  },
]

export default createRouter({
  history: createWebHashHistory(),
  routes,
})