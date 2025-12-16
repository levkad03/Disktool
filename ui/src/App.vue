<script setup lang="ts">
  import { ref } from 'vue';
  import { invoke } from '@tauri-apps/api/core';
  import InputField from './components/InputField.vue';
  import FileCard from './components/FileCard.vue';

  interface FileInfo {
    path: string;
    size: number;
  }

  interface DirectoryReport {
    total_size: number;
    top_files: FileInfo[];
  }

  const path = ref<string>('');
  const top = ref<number>(10);
  const report = ref<DirectoryReport | null>(null);
  const loading = ref<boolean>(false);

  async function scan() {
    loading.value = true;
    try {
      report.value = await invoke<DirectoryReport>('scan', {
        path: path.value,
        top: top.value,
      });
    } finally {
      loading.value = false;
    }
  }
  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
  }
</script>

<template>
  <div
    class="min-h-screen bg-linear-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800 py-12 px-4"
  >
    <div class="max-w-2xl mx-auto">
      <!-- Header -->
      <div class="mb-8">
        <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-2">Disk Tool</h1>
        <p class="text-gray-600 dark:text-gray-400">
          Analyze your disk usage and find the largest files
        </p>
      </div>

      <!-- Input Section -->
      <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6 mb-8">
        <InputField v-model="path" label="Directory Path" placeholder="e.g., C:\ or /home/user" />
        <InputField v-model.number="top" label="Top Files Count" type="number" class="mt-4" />
        <button
          :disabled="!path || loading"
          class="w-full mt-6 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white font-semibold py-3 px-6 rounded-lg transition duration-200 flex items-center justify-center space-x-2"
          @click="scan"
        >
          <span v-if="!loading">Scan Directory</span>
          <span v-else>Scanning...</span>
        </button>
      </div>

      <!-- Results Section -->
      <div v-if="report" class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6">
        <div class="mb-6">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">Results</h2>
          <p class="text-gray-600 dark:text-gray-400">
            Total Size:
            <span class="font-semibold text-lg">{{ formatBytes(report.total_size) }}</span>
          </p>
        </div>

        <!-- Files List -->
        <div class="space-y-3">
          <FileCard
            v-for="(file, index) in report.top_files"
            :key="file.path"
            :rank="index + 1"
            :path="file.path"
            :size="file.size"
          />
        </div>
      </div>

      <!-- Empty State -->
      <div v-else-if="!loading" class="text-center py-12">
        <p class="text-gray-600 dark:text-gray-400 text-lg">
          Enter a directory path and click "Scan" to analyze disk usage
        </p>
      </div>
    </div>
  </div>
</template>
