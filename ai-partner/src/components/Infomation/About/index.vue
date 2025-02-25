<template>
  <div class="app-about" :class="{ 'dark-mode': darkMode }">
    <!-- 头部品牌信息 -->
    <header class="brand-header">
      <img :src="appInfo.logo" alt="App Logo" class="app-logo" />
      <div class="title-group">
        <h1>{{ appInfo.name }}</h1>
        <p class="version-info">
          Version {{ appInfo.version }} (Build {{ appInfo.buildNumber }})
        </p>
      </div>
    </header>

    <!-- 主要信息卡片 -->
    <div class="info-cards">
      <!-- 基础信息卡片 -->
      <div class="info-card">
        <h2><i class="icon-info"></i> 基本信息</h2>
        <div class="info-grid">
          <div class="info-item">
            <label>架构：</label>
            <span>{{ systemInfo.architecture }}</span>
          </div>
          <div class="info-item">
            <label>编译日期：</label>
            <span>{{ appInfo.buildDate }}</span>
          </div>
          <div class="info-item">
            <label>运行环境：</label>
            <span>{{ appInfo.runtime }}</span>
          </div>
          <div class="info-item">
            <label>配置文件：</label>
            <a @click="openConfigFolder">{{ appInfo.configPath }}</a>
          </div>
        </div>
      </div>

      <!-- 系统状态卡片 -->
      <div class="info-card">
        <h2><i class="icon-monitor"></i> 系统状态</h2>
        <div class="status-grid">
          <div class="status-item">
            <div class="status-label">内存使用</div>
            <div class="status-value">
              {{ formatMemory(systemStatus.memoryUsage) }} / 
              {{ formatMemory(systemStatus.totalMemory) }}
            </div>
            <progress 
              :value="systemStatus.memoryUsage" 
              :max="systemStatus.totalMemory"
            ></progress>
          </div>
          <div class="status-item">
            <div class="status-label">CPU占用</div>
            <div class="status-value">{{ systemStatus.cpuLoad }}%</div>
            <progress 
              :value="systemStatus.cpuLoad" 
              max="100"
            ></progress>
          </div>
        </div>
      </div>

      <!-- 依赖库信息卡片 -->
      <div class="info-card">
        <h2><i class="icon-package"></i> 依赖库</h2>
        <div class="dependency-list">
          <div 
            v-for="(dep, index) in dependencies" 
            :key="index"
            class="dependency-item"
          >
            <div class="dep-name">{{ dep.name }}</div>
            <div class="dep-version">{{ dep.version }}</div>
            <a 
              v-if="dep.license" 
              class="dep-license"
              @click="showLicense(dep)"
            >{{ dep.license }}</a>
          </div>
        </div>
      </div>
    </div>

    <!-- 操作按钮组 -->
    <div class="action-buttons">
      <button class="btn" @click="checkUpdate">
        <i class="icon-update"></i> 检查更新
      </button>
      <button class="btn" @click="openLogsFolder">
        <i class="icon-folder"></i> 打开日志目录
      </button>
      <button class="btn" @click="copySystemInfo">
        <i class="icon-copy"></i> 复制诊断信息
      </button>
    </div>

    <!-- 许可证弹窗 -->
    <!-- <LicenseDialog 
      v-if="showLicenseDialog"
      :library="selectedLibrary"
      @close="showLicenseDialog = false"
    /> -->
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue';
import { platform,arch } from '@tauri-apps/plugin-os';
import {getVersion} from '@tauri-apps/api/app'
const darkMode = ref(false);
const showLicenseDialog = ref(false);
const selectedLibrary = ref(null);

const appInfo = reactive({
  name: 'My Application',
  version: '',
  buildNumber: '2101',
  buildDate: '2023-08-20',
  runtime: 'Electron 24.1.0 / Node.js 18.16.0',
  configPath: '/Users/username/.app/config',
  logo: '/path/to/logo.png'
});

const systemInfo = reactive({
  architecture: arch(),
  platform,
  // 其他系统信息...
});

const systemStatus = reactive({
  memoryUsage: 0,
  totalMemory: 0,
  cpuLoad: 0
});

const dependencies = ref([
  { name: 'Vue', version: '3.3.4', license: 'MIT' },
  { name: 'Electron', version: '24.1.0', license: 'MIT' },
  { name: 'TypeScript', version: '5.1.6', license: 'Apache-2.0' }
]);

// 实时更新系统状态
let updateInterval;
onMounted(async() => {
  appInfo.version = await getVersion()
  updateInterval = setInterval(() => {
    systemStatus.memoryUsage = window.performance.memory.usedJSHeapSize;
    systemStatus.totalMemory = window.performance.memory.totalJSHeapSize;
    // systemStatus.cpuLoad ; // 需要实现具体逻辑
  }, 1000);
});

const formatMemory = (bytes) => {
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
};

const checkUpdate = async () => {
  // 实现更新检查逻辑
};

const openConfigFolder = () => {
  window.shell.openPath(appInfo.configPath); // Electron API示例
};

const showLicense = (library) => {
  selectedLibrary.value = library;
  showLicenseDialog.value = true;
};

const copySystemInfo = () => {
  const info = `
    Application: ${appInfo.name} v${appInfo.version}
    Build: ${appInfo.buildNumber} (${appInfo.buildDate})
    Memory Usage: ${formatMemory(systemStatus.memoryUsage)}
    CPU Load: ${systemStatus.cpuLoad}%
  `;
  navigator.clipboard.writeText(info);
};
</script>

<style scoped>
.app-about {
  max-width: 800px;
  margin: 0 auto;
  background: var(--bg-color);
  color: var(--text-color);
  max-height: 100%;
}

.brand-header {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.app-logo {
  width: 64px;
  height: 64px;
}

.title-group h1 {
  margin: 0;
  font-size: 1.8rem;
}

.version-info {
  color: var(--secondary-text);
  margin: 0.25rem 0 0;
}

.info-cards {
  display: grid;
  gap: 1.5rem;
}

.info-card {
  background: var(--card-bg);
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
  margin-top: 1rem;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.status-grid {
  display: grid;
  gap: 1rem;
}

.status-item progress {
  width: 100%;
  height: 6px;
}

.dependency-list {
  display: grid;
  gap: 0.5rem;
}

.dependency-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.5rem;
  border-radius: 4px;
  background: var(--item-bg);
}

.action-buttons {
  margin-top: 2rem;
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.btn {
  padding: 0.5rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--button-bg);
  color: var(--button-text);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

/* 深色模式变量 */
.dark-mode {
  --bg-color: #1a1a1a;
  --text-color: #ffffff;
  --secondary-text: #a0a0a0;
  --card-bg: #2d2d2d;
  --item-bg: #3a3a3a;
  --border-color: #4a4a4a;
  --button-bg: #3a3a3a;
}
</style>