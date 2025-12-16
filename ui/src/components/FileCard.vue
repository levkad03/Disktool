<script setup lang="ts">
  defineProps<{
    path: string;
    size: number;
    rank: number;
  }>();

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
  }
</script>

<template>
  <div
    class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-600 transition"
  >
    <div class="flex items-center space-x-4 flex-1">
      <span class="text-lg font-bold text-blue-600 dark:text-blue-400 w-8">#{{ rank }}</span>
      <span class="text-gray-800 dark:text-gray-200 truncate">{{ path }}</span>
    </div>
    <span class="text-lg font-semibold text-gray-900 dark:text-white ml-4 whitespace-nowrap">
      {{ formatBytes(size) }}
    </span>
  </div>
</template>
