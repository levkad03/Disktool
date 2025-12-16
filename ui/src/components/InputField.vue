<script setup lang="ts">
  defineProps<{
    label: string;
    placeholder?: string;
    type?: string;
    modelValue: string | number;
  }>();

  const emit = defineEmits<{
    'update:modelValue': [value: string | number];
  }>();

  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
    emit('update:modelValue', target.value);
  }
</script>

<template>
  <div class="space-y-2 font-medium text-gray-700 dark:text-gray-300">
    <label class="block text-sm">{{ label }}</label>
    <input
      :value="modelValue"
      :type="type || 'text'"
      :placeholder="placeholder"
      class="w-full px-4 py-2 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-800 dark:border-gray-600 dark:text-white transition"
      @input="handleInput"
    />
  </div>
</template>
