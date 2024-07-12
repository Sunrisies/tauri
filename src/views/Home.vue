<script setup lang="ts">
import { hide, show } from '@tauri-apps/api/app'
import { register } from '@tauri-apps/api/globalShortcut'
import { type } from '@tauri-apps/api/os'
import { invoke } from '@tauri-apps/api/tauri'
import { WebviewWindow, appWindow } from '@tauri-apps/api/window'
import { ref } from 'vue'
// import { fetch, type FetchOptions } from '@tauri-apps/api/http'
const greetMsg = ref('')
const name = ref('')

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke('greet', { name: name.value })
}

const windowFocused = ref(true)
register('Ctrl+.', () => {
  // 如果窗口已经显示，就隐藏
  console.log('Ctrl+.', windowFocused.value)
  if (windowFocused.value) {
    windowFocused.value = false
    showWindow()
  } else {
    windowFocused.value = true
    hideWindow()
  }
})

/**
 * 显示窗口
 */
const showWindow = async () => {
  const platform = await type()
  appWindow.unminimize()
  if (platform === 'Darwin') {
    show()
  } else {
    appWindow.show()
  }
  appWindow.setFocus()
}

/**
 * 隐藏窗口
 */
const hideWindow = async () => {
  const platform = await type()
  if (platform === 'Darwin') {
    hide()
  } else {
    appWindow.hide()
  }
}
const handleEnter = async () => {
  await invoke('opengug', { name: `https://www.baidu.com/s?ie=UTF-8&wd=${name.value}` })
  windowFocused.value = true
  hideWindow()
  name.value = ''
}
const openAbout = () => {
  invoke('open_about', {})
  // const webview = new WebviewWindow('search', {
  //   url: '/search',
  //   title: '搜索',

  //   width: 400,
  //   height: 400,
  //   x: 100,
  //   y:200
  //   // title: 'Search',
  //   // resizable: false,
  //   // decorations: false,
  //   // center: true,
  //   // focus: true,
  //   // transparent: true,
  // })
  // webview.once("tauri://created", () => {
  //   console.log('创建窗口')
  // })
  // webview.once("tauri://close-requested", () => {
  //   console.log('关闭窗口')
  // })
  // webview.once("tauri://resize", () => {
  //   console.log('窗口大小改变')
  // })
  // webview.once("tauri://focus", () => {
  //   console.log('窗口获得焦点')
  // })
  // webview.once("tauri://blur", () => {
  //   console.log('窗口失去焦点')
  // })
  // webview.once("tauri://move", () => {
  //   console.log('窗口位置改变')
  // })
  // webview.once("tauri://ready", () => {
  //   console.log('窗口准备好')
  // })
  // webview.once("tauri://error", () => {
  //   console.log('窗口错误')
  // })
  // webview.once("tauri://warn", () => {
  //   console.log('窗口警告')
  // })
}
</script>

<template>
  <div class="container">
    <div class="input-group">
      <el-input v-model="name" class="input" size="large" autofocus @press-enter="handleEnter" placeholder="Enter a name..." />

      <el-button @click="openAbout" type="primary">Greet</el-button>
    </div>
    <p class="version">版本号v0.0.1</p>
  </div>
 
</template>
<style>
.container {
  display: flex;
  align-items: center;
  flex-direction: column;
  border: 1px solid red;
  height: 100%;
  width: 100%;
}
.input-group {
  display: flex;

}
.input{
  width: 600px;
}
.version {
  color: red;
}
</style>
