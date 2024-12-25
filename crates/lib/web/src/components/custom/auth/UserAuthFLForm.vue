<script setup lang="ts">
import { cn } from '@/lib/utils';
import { Loader2 } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';

import { useRoute, useRouter } from 'vue-router';
import { ref } from 'vue';

const router = useRouter();

const path = useRoute().fullPath;
const isSubmitting = ref(false);
const password = ref('');
const confirmPassword = ref('');

const handleSubmit = async (e: Event) => {
  e.preventDefault();
  isSubmitting.value = true;
  try {
    const response = await fetch(`/api/login${path}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: new URLSearchParams({
        newPassword: password.value,
      }).toString(),
    });

    if (response.ok) {
      const redirect = await response.json();
      router.push(redirect);
    } else {
      console.error('Error:', response.statusText);
    }
  } catch (error) {
    console.error('Error:', error);
  } finally {
    isSubmitting.value = false;
  }
};
</script>

<template>
  <div :class="cn('grid gap-6', $attrs.class ?? '')">
    <form @submit="handleSubmit">
      <div class="grid gap-2">
        <div class="grid gap-1">
          <Label class="sr-only" for="password"> New Password </Label>
          <Input
            id="password"
            name="password"
            placeholder="New Password"
            auto-capitalize="none"
            auto-complete="off"
            auto-correct="off"
            type="password"
            v-model="password"
            :disabled="isSubmitting"
          />
        </div>
        <div class="grid gap-1">
          <Label class="sr-only" for="newPassword"> Confirm New Password </Label>
          <Input
            id="newPassword"
            name="newPassword"
            placeholder="Confirm Password"
            auto-capitalize="none"
            auto-complete="off"
            auto-correct="off"
            type="password"
            v-model="confirmPassword"
            :disabled="isSubmitting"
          />
        </div>
        <Button :disabled="isSubmitting || !password || !confirmPassword || password !== confirmPassword">
          <Loader2 v-if="isSubmitting" class="mr-2 h-4 w-4 animate-spin" />
          Sign In
        </Button>
      </div>
    </form>
  </div>
</template>
