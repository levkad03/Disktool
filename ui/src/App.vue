<script setup lang="ts">
  import { ref } from 'vue'
  import { invoke } from '@tauri-apps/api/core'

  interface FileInfo {
    path: string
    size: number
  }

  interface DirectoryReport {
    total_size: number
    top_files: FileInfo[]
  }

  const path = ref<string>('')
  const top = ref<number>(10)
  const report = ref<DirectoryReport | null>(null)

  async function scan() {
    report.value = await invoke<DirectoryReport>('scan', {
      path: path.value,
      top: top.value,
    })
  }
</script>

<template>
  <h2>Disk Tool</h2>

  <input v-model="path" placeholder="Path" />
  <input v-model="top" type="number" />
  <button @click="scan">Scan</button>

  <pre v-if="report">{{ report }}</pre>
</template>
