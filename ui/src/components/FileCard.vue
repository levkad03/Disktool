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
    class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-600 transition overflow-hidden"
  >
    <div class="flex items-center space-x-4 flex-1 min-w-0">
      <span class="text-lg font-bold text-blue-600 dark:text-blue-400 shrink-0">#{{ rank }}</span>
      <span class="text-gray-800 dark:text-gray-200 truncate text-sm">{{ path }}</span>
    </div>
    <span class="text-lg font-semibold text-gray-900 dark:text-white ml-4 shrink-0">
      {{ formatBytes(size) }}
    </span>
  </div>
</template>
