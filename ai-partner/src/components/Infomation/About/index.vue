<template>
  <div class="app-about" :class="{ 'dark-mode': darkMode }">
    <!-- 头部品牌信息 -->

    <n-tabs type="segment">

      <n-tab-pane name="chap1" tab="使用说明">
        <div >
          安装后开始对话前需要先
          <b>配置api相关信息:</b>
          <br/>
          在设置中找到api配置，填入api地址、api密钥以及调用的模型名即可开始对话。
        </div>
        <n-divider></n-divider>
        <div>
          smooth选项为是否开启平滑输出，开启后每个字符会逐个输出，仅优化视觉效果，实际性能较低
        </div>
        <n-divider></n-divider>
        <div>
          <b>快捷键:</b>
          <br/>
          打开快速对话框: <b>alt + 1</b> / <b>alt + n</b>
          <br/>

        </div>
      </n-tab-pane>
      <!-- <n-tab-pane name="chap2" tab="第三章">
        但是忽然，公寓的烟味消失，火警也停了。我的女朋友走进了房间，让我震惊的是，她摘下了自己的假发，她是
        Jeff Bezos（Amazon 老板）假扮的！<br><br>
        “我对你坚持顾客至上的原则感到十分骄傲”，说完，他递给我一张五美金的亚马逊礼品卡，从我家窗户翻了出去，跳上了一辆
        Amazon 会员服务的小货车，一溜烟离开了。<br><br>虽然现在我已不在 Amazon
        工作，但我还是非常感激在哪里学的到的经验，这些经验我终身难忘。你们同意么？
      </n-tab-pane> -->
      <n-tab-pane name="chap3" tab="版本信息" display-directive="show-lazy">
        <header class="brand-header">
          <img src="/src/assets/tauri.svg" alt="App Logo" class="app-logo" />
          <div class="title-group">
            <h1>{{ appInfo.name }}</h1>
            <p class="version-info">
              Version {{ appInfo.version }}
            </p>
          </div>
        </header>

        <!-- 基础信息卡片 -->
        <div class="info-card">
          <h2><i class="icon-info"></i> 基本信息</h2>
          <div class="info-grid">
            <div class="info-item">
              <label>项目地址：</label>
              <span><a href="https://github.com/Yoak3n/rust_homework/tree/main/ai-partner" target="_blank" rel="noopener noreferrer">ai-partner</a></span>
            </div>
            <div class="info-item">
              <label>编译日期：</label>
              <span>{{ appInfo.buildDate }}</span>
            </div>
            <div class="info-item">
              <label>开发框架：</label>
              <span>
                <a href="https://tauri.app/" target="_blank" rel="noopener noreferrer">tauri {{ tauriVersion }}</a>
                /
                <a href="https://vuejs.org/" target="_blank" rel="noopener noreferrer">vue {{version}}</a>
              </span>
            </div>
          </div>
        </div>
        <div class="info-card">
          <h2><i class="icon-info"></i>开发者信息</h2>
          <div class="info-grid">

            <div class="info-item">
              <label>                
                <n-avatar src="https://avatars.githubusercontent.com/u/120039624?v=4">
                </n-avatar>
              </label>
              <span>
                <a href="https://github.com/Yoak3n" target="_blank" rel="noopener noreferrer">Yoake</a>
              </span>
            </div>
            <div class="info-item">
              <label>网站：</label>
              <span><a href="https://www.yoake.top" target="_blank" rel="noopener noreferrer">Mou1ght</a></span>
            </div>
            <div>
              一个喜欢摸鱼偷闲时写点东西的人
            </div>
          </div>
        </div>
        <!-- 操作按钮组 -->
        <div class="action-buttons">
          <button class="btn" @click="checkUpdate">
            <i class="icon-update"></i> 检查更新
          </button>
          <button class="btn" @click="openLogsFolder">
            <i class="icon-folder"></i> 打开所在目录
          </button>
          <button class="btn" @click="copySystemInfo">
            <i class="icon-copy"></i> 复制诊断信息
          </button>
        </div>
      </n-tab-pane>
    </n-tabs>







    <!-- 许可证弹窗 -->
    <!-- <LicenseDialog 
      v-if="showLicenseDialog"
      :library="selectedLibrary"
      @close="showLicenseDialog = false"
    /> -->
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, version } from 'vue';
import { NButton, NTabs, NTabPane,NDivider,NAvatar } from 'naive-ui';
import { getVersion, getName, getTauriVersion } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api/core';
import {open} from '@tauri-apps/plugin-shell'
import versionJson from '../../../utils/versionJson.json'
const darkMode = ref(false);
const showLicenseDialog = ref(false);
const selectedLibrary = ref(null);

let appInfo = reactive({
  name: 'My Application',
  version: '',
  buildNumber: '2101',
  buildDate: '2023-08-20',
  configPath: '/Users/username/.app/config',
  logo: '/path/to/logo.png'
});


let tauriVersion = ref('');
onMounted(async () => {
  tauriVersion.value = await getTauriVersion();
  appInfo.version = await getVersion();
  appInfo.name = await getName()
  appInfo.buildDate = new Date(versionJson.compileTime).toLocaleDateString();
});

const checkUpdate = async () => {
  // 实现更新检查逻辑
};

const openLogsFolder= async () => {
  const pwd = await invoke('get_app_install_path')
  open(pwd)
}

const showLicense = (library) => {
  selectedLibrary.value = library;
  showLicenseDialog.value = true;
};

</script>

<style scoped>
.app-about {
  max-width: 800px;
  margin: 0 auto;
  background: var(--bg-color);
  color: var(--text-color);
  max-height: 80%;
}

.brand-header {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  margin-bottom: 0rem;
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
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
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