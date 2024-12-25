<script setup lang="ts">
import { cn } from '@/lib/utils';
import { RouterLink } from 'vue-router';
</script>

<template>
  <nav
    :class="
      cn('flex items-center space-x-4 lg:space-x-6 text-muted-foreground', $attrs.class ?? '')
    "
  >
    <RouterLink
      :to="{ name: 'overview' }"
      exact-active-class="text-foreground"
      class="text-sm font-medium transition-colors hover:text-primary"
    >
      Overview
    </RouterLink>
  </nav>
</template>
