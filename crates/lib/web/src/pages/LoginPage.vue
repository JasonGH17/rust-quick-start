<script setup lang="ts">
import { UserAuthForm } from '@/components/custom/auth';
import { cn } from '@/lib/utils';
import { buttonVariants } from '@/components/ui/button';
</script>

<template>
  <div
    class="container relative h-full flex-col items-center justify-center grid lg:max-w-none lg:grid-cols-2 lg:px-0"
  >
    <a
      href="/login"
      :class="
        cn(buttonVariants({ variant: 'ghost' }), 'absolute right-4 top-4 md:right-8 md:top-8')
      "
    >
      Login
    </a>
    <div
      class="relative hidden h-screen flex-col bg-muted p-10 text-primary dark:border-r lg:flex justify-center bg-gray-50"
    >
      <div class="absolute inset-0 h-screen top-1/4">
        <!-- Hero Image -->
      </div>
      <div class="relative z-20 flex items-center">
        <img width="200" src="../assets/HighResLogo.png" />
      </div>
      <div class="relative z-20 mt-auto">
        <blockquote class="space-y-2">
          <p class="font-medium text-xl">&ldquo;We bring your vision to reality&rdquo;</p>
          <footer class="text-sm"></footer>
        </blockquote>
      </div>
    </div>
    <div class="lg:p-8 h-screen">
      <div class="mx-auto flex w-full h-full flex-col justify-center space-y-6 sm:w-[350px]">
        <div class="flex flex-col space-y-2 text-center items-center">
          <img width="256" class="lg:hidden block mb-10" src="../assets/HighResLogo.png" />
          <h1 class="text-2xl font-semibold tracking-tight">Login to your account</h1>
          <p class="text-sm text-muted-foreground">
            Enter your username below to access your account
          </p>
        </div>
        <UserAuthForm />
      </div>
    </div>
  </div>
</template>
