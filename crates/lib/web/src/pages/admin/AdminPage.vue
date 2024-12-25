<script setup lang="ts">
import { MainNav, UserNav } from '@/components/custom/admin';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Icon } from '@iconify/vue';
import { useColorMode } from '@vueuse/core';
import { computed } from 'vue';
import { useRouter } from 'vue-router';

const mode = useColorMode({ disableTransition: false });

const router = useRouter();
const title = computed(() => router.currentRoute.value.meta.adminTitle as string);
</script>

<template>
  <div class="bg-background flex-col flex min-h-svh">
    <div class="border-b">
      <div class="flex h-16 items-center px-4">
        <MainNav class="mx-6" />
        <div class="ml-auto flex items-center space-x-4">
          <DropdownMenu>
            <DropdownMenuTrigger as-child>
              <Button variant="outline">
                <Icon
                  icon="radix-icons:moon"
                  class="h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all text-foreground dark:rotate-0 dark:scale-100"
                />
                <Icon
                  icon="radix-icons:sun"
                  class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-100 transition-all text-foreground dark:-rotate-90 dark:scale-0"
                />
                <span class="sr-only">Toggle theme</span>
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem @click="mode = 'light'"> Light </DropdownMenuItem>
              <DropdownMenuItem @click="mode = 'dark'"> Dark </DropdownMenuItem>
              <DropdownMenuItem @click="mode = 'auto'"> System </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
          <UserNav />
        </div>
      </div>
    </div>
    <div class="container min-[1400px]:max-w-[1638px] flex-1 space-y-4 p-8 pt-6">
      <div class="space-y-2">
        <h2 class="text-3xl font-bold text-foreground tracking-normal">{{ title }}</h2>
      </div>
      <RouterView />
    </div>
  </div>
</template>
