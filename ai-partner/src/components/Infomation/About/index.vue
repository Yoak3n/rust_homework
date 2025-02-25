<template>
    <div class="software-container">
      <!-- 头部信息 -->
      <header class="software-header">
        <div class="title-section">
          <img :src="software.icon" alt="软件图标" class="software-icon" />
          <div>
            <h1>{{ software.name }}</h1>
            <p class="developer">开发者：{{ software.developer }}</p>
          </div>
        </div>
        <div class="action-buttons">
          <button class="download-btn" @click="handleDownload">
            {{ software.latestVersion ? `下载 v${software.latestVersion}` : '立即下载' }}
          </button>
        </div>
      </header>
  
      <!-- 主要内容区域 -->
      <main class="software-content">
        <!-- 左侧内容 -->
        <div class="main-info">
          <section class="screenshots">
            <h2>截图预览</h2>
            <div class="screenshot-grid">
              <img 
                v-for="(img, index) in software.screenshots" 
                :key="index"
                :src="img"
                alt="软件截图"
                class="screenshot"
              />
            </div>
          </section>
  
          <section class="description">
            <h2>软件描述</h2>
            <p>{{ software.description }}</p>
          </section>
        </div>
  
        <!-- 右侧侧边栏 -->
        <aside class="sidebar">
          <div class="info-card">
            <h3>基本信息</h3>
            <ul class="info-list">
              <li>
                <span class="label">版本：</span>
                {{ software.version }}
              </li>
              <li>
                <span class="label">更新时间：</span>
                {{ software.updateDate }}
              </li>
              <li>
                <span class="label">文件大小：</span>
                {{ software.fileSize }}
              </li>
              <li>
                <span class="label">许可证：</span>
                {{ software.license }}
              </li>
            </ul>
          </div>
  
          <div class="info-card">
            <h3>系统要求</h3>
            <ul class="requirements">
              <li v-for="(req, key) in software.requirements" :key="key">
                <span class="req-title">{{ key }}：</span>
                {{ req }}
              </li>
            </ul>
          </div>
        </aside>
      </main>
    </div>
  </template>
  
  <script setup>
  import { reactive } from 'vue';
  
  const software = reactive({
    name: 'Awesome Studio',
    developer: 'Creative Tech Inc.',
    version: '3.2.1',
    latestVersion: '3.2.1',
    updateDate: '2023-08-15',
    fileSize: '286 MB',
    license: '免费增值',
    description: '一款功能强大的创意设计工具，提供全面的图形设计、视频编辑和3D建模功能。支持多平台协作，内置丰富的素材库和智能模板，助力您的创意工作流程。',
    icon: '/path/to/icon.png',
    screenshots: [
      '/path/to/screenshot1.jpg',
      '/path/to/screenshot2.jpg',
      '/path/to/screenshot3.jpg'
    ],
    requirements: {
      '操作系统': 'Windows 10/11 64位 或 macOS 10.15+',
      '处理器': 'Intel Core i5 或同等性能处理器',
      '内存': '8 GB RAM (推荐16 GB)',
      '显卡': '支持DirectX 12 / Metal的独立显卡',
      '存储': '2 GB 可用空间'
    }
  });
  
  const handleDownload = () => {
    // 处理下载逻辑
    alert('开始下载...');
    // 实际应用中这里可以跳转到下载链接
  };
  </script>
  
  <style scoped>
  .software-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }
  
  .software-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }
  
  .title-section {
    display: flex;
    align-items: center;
    gap: 1.5rem;
  }
  
  .software-icon {
    width: 80px;
    height: 80px;
    border-radius: 16px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  }
  
  h1 {
    font-size: 2rem;
    margin: 0;
  }
  
  .developer {
    color: #666;
    margin: 0.5rem 0 0;
  }
  
  .download-btn {
    background: #007bff;
    color: white;
    padding: 0.8rem 1.5rem;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-size: 1rem;
    transition: background 0.3s;
  }
  
  .download-btn:hover {
    background: #0056b3;
  }
  
  .software-content {
    display: grid;
    grid-template-columns: 1fr 300px;
    gap: 2rem;
  }
  
  .screenshots {
    margin-bottom: 2rem;
  }
  
  .screenshot-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
    margin-top: 1rem;
  }
  
  .screenshot {
    width: 100%;
    border-radius: 8px;
    box-shadow: 0 2px 6px rgba(0,0,0,0.1);
  }
  
  .description {
    line-height: 1.6;
    color: #444;
  }
  
  .info-card {
    background: #f8f9fa;
    border-radius: 12px;
    padding: 1.5rem;
    margin-bottom: 1.5rem;
  }
  
  .info-card h3 {
    margin-top: 0;
    color: #333;
    font-size: 1.2rem;
  }
  
  .info-list {
    list-style: none;
    padding: 0;
  }
  
  .info-list li {
    margin: 0.8rem 0;
    display: flex;
    justify-content: space-between;
  }
  
  .label {
    color: #666;
    font-weight: 500;
  }
  
  .requirements li {
    margin: 0.8rem 0;
  }
  
  .req-title {
    font-weight: 500;
    color: #444;
  }
  
  @media (max-width: 768px) {
    .software-content {
      grid-template-columns: 1fr;
    }
    
    .software-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 1rem;
    }
  }
  </style>