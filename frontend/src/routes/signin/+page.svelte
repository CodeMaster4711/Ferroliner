<script lang="ts">
  import { Field, FieldLabel } from '$lib/components/ui/field/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { Alert, AlertDescription } from '$lib/components/ui/alert/index.js';
  import { AuthService } from '$lib/services/auth';
  import { authStore } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  let username = $state('');
  let password = $state('');
  let isLoading = $state(false);
  let errorMessage = $state('');

  async function handleSubmit(event: Event) {
    event.preventDefault();
    isLoading = true;
    errorMessage = '';

    try {
      const result = await AuthService.login(username, password);

      await fetch('/api/set-auth-cookie', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ token: result.token }),
      });

      authStore.login(result.user, result.token);

      if (result.user.force_password_change) {
        goto('/change-password');
      } else {
        goto('/');
      }
    } catch (error) {
      if (error instanceof Error && error.message === '2FA_REQUIRED') {
        sessionStorage.setItem('pending_2fa_username', username);
        sessionStorage.setItem('pending_2fa_password', password);
        goto('/otp');
        return;
      }
      errorMessage = error instanceof Error ? error.message : 'Login fehlgeschlagen';
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="flex min-h-screen">
  <div class="relative hidden w-1/2 overflow-hidden lg:flex lg:flex-col lg:justify-end lg:p-12">
    <img
      src="/Gradient-dark.svg"
      alt=""
      class="absolute inset-0 h-full w-full object-cover"
      aria-hidden="true"
    />
    <div class="relative z-10">
      <h1 class="text-5xl font-bold tracking-tight text-white">Ferroliner</h1>
      <p class="mt-3 text-base text-white/60">Project management for railway operations</p>
    </div>
  </div>

  <div class="flex w-full flex-col items-center justify-center px-8 lg:w-72 lg:flex-none lg:px-0">
    <div class="w-full max-w-xs">
      <div class="mb-8">
        <h2 class="text-2xl font-semibold tracking-tight">Sign in</h2>
        <p class="mt-1 text-sm text-muted-foreground">Enter your credentials to continue</p>
      </div>

      <form onsubmit={handleSubmit} class="space-y-4">
        {#if errorMessage}
          <Alert variant="destructive">
            <AlertDescription>{errorMessage}</AlertDescription>
          </Alert>
        {/if}

        <Field>
          <FieldLabel for="username">Email</FieldLabel>
          <Input
            id="username"
            type="email"
            bind:value={username}
            placeholder="name@example.com"
            disabled={isLoading}
            required
            autocomplete="email"
          />
        </Field>

        <Field>
          <FieldLabel for="password">Password</FieldLabel>
          <Input
            id="password"
            type="password"
            bind:value={password}
            placeholder="••••••••"
            disabled={isLoading}
            required
            autocomplete="current-password"
          />
        </Field>

        <Button type="submit" disabled={isLoading} class="w-full">
          {#if isLoading}
            <div class="mr-2 h-4 w-4 animate-spin rounded-full border-b-2 border-current"></div>
          {/if}
          Sign in
        </Button>
      </form>
    </div>
  </div>
</div>
