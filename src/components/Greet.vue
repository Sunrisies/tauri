<script setup lang="ts">
import { type } from '@tauri-apps/api/os'
import { appWindow, PhysicalSize } from '@tauri-apps/api/window'
import { hide, show } from '@tauri-apps/api/app'
import { register, unregisterAll } from '@tauri-apps/api/globalShortcut'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { fetch, type FetchOptions } from '@tauri-apps/api/http'
const greetMsg = ref('')
const name = ref('')

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke('greet', { name: name.value })
}
const open = async () => {
  await invoke('opengug',{name:`https://www.baidu.com/s?ie=UTF-8&wd=${name.value}`})
}

const windowFocused = ref(true)
register('Ctrl+.', () => {
  // 如果窗口已经显示，就隐藏
  if (windowFocused.value) {
    // 窗口打开时居中还是上次位置
    // if (!isRememberPosition.value) {
    //   appWindow.center()
    // }
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
  console.log(111)
  appWindow.unminimize()

  if (platform === 'Darwin') {
    show()
  } else {
    appWindow.show()
  }

  appWindow.setFocus()
}
const fetchApi = async (url) => {
  // const { data }: Record<string, any> = await fetch(url, {
  //   ...options,
  //   method,
  //   timeout: 1000 * 60,
  //   headers: {
  //     ...headers,
  //     'Content-Type': 'application/json',
  //     'user-agent':
  //       'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36'
  //   }
  // })
  console.log(url, 'url')
  const data = await fetch(url)
  console.log(data, 'data')
  const { error } = data

  if (error) throw new Error(error.message)

  return data
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
const handleEnter = (v) => {
  const a = document.createElement('a')
  a.href = 'https://www.baidu.com/s?ie=UTF-8&wd=sa'
  a.target = '_blank'
  a.click()
  console.log('按钮被点击了')
  // console.log(v.target.value, 'sakdaskdka')
  // fetchApi(`https://www.baidu.com/s?wd=${v.target.value}`).then((res) => {
  //   console.log(res, 'res')
  // })
}
</script>

<template>
  <div class="container">
    <form class="row" @submit.prevent="greet">
    <a-button @click="open">打开</a-button>
      <a-input id="greet-input" v-model="name" autofocus @press-enter="handleEnter" placeholder="Enter a name..." />
    </form>
  </div>
</template>
<style>
.container {
  display: flex;
  align-items: center;
  justify-content: center;
}
input {
  padding: 10px;
  width: 700px;
  border-radius: 5px;
}
</style>
