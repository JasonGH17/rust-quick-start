<script setup lang="ts">
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { computed, ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();

async function logout() {
  const response = await fetch('/api/login', {
    method: 'DELETE',
  });

  if (response.ok) {
    router.push('/login');
  }
}

const email = ref('admin@localhost');
const initials = computed(() => {
  const address = email.value.toUpperCase().split('@');
  const parts = address[0].split('.');
  return parts.length > 1 ? `${parts[0][0]}${parts[1][0]}` : parts[0].substring(0, 2);
});

fetch('/api/user/me')
  .then((res) => res.json())
  .then((json: { email: string }) => {
    email.value = json.email;
  });
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Button variant="ghost" class="relative h-8 w-8 rounded-full">
        <Avatar class="h-8 w-8">
          <AvatarFallback>{{ initials }}</AvatarFallback>
        </Avatar>
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent class="w-56" align="end">
      <DropdownMenuLabel class="font-normal flex">
        <div class="flex flex-col space-y-1">
          <p class="text-xs leading-none text-muted-foreground">{{ email }}</p>
        </div>
      </DropdownMenuLabel>
      <DropdownMenuItem @click="logout"> Log out </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>
