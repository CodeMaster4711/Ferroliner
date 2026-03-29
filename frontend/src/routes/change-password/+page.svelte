<script lang="ts">
  import { Field, FieldLabel } from '$lib/components/ui/field/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { Alert, AlertDescription } from '$lib/components/ui/alert/index.js';
  import { SettingsService } from '$lib/services/settings';
  import { AuthService } from '$lib/services/auth';
  import { authStore } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  let newPassword = $state('');
  let confirmPassword = $state('');
  let isLoading = $state(false);
  let errorMessage = $state('');

  async function handlePasswordChange(event: Event) {
    event.preventDefault();

    if (newPassword !== confirmPassword) {
      errorMessage = 'Passwords do not match';
      return;
    }

    if (newPassword.length < 6) {
      errorMessage = 'Password must be at least 6 characters';
      return;
    }

    isLoading = true;
    errorMessage = '';

    try {
      const publicKey = await AuthService.getPublicKey();
      const encryptedNewPassword = await AuthService.encryptPassword(newPassword, publicKey);

      await SettingsService.changePassword('', encryptedNewPassword);
      const user = $authStore.user;
      if (user) {
        authStore.login({ ...user, force_password_change: false }, $authStore.token!);
      }
      goto('/');
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : 'Password change failed';
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="flex min-h-screen">
  <div class="relative hidden overflow-hidden lg:flex lg:flex-1 lg:flex-col lg:justify-end lg:p-12">
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

  <div class="flex flex-1 flex-col items-center justify-center px-8 lg:px-16">
    <div class="w-full max-w-xs">
      <div class="mb-8">
        <h2 class="text-2xl font-semibold tracking-tight">Change password</h2>
        <p class="mt-1 text-sm text-muted-foreground">Choose a new password to continue</p>
      </div>

      <form onsubmit={handlePasswordChange} class="space-y-4">
        {#if errorMessage}
          <Alert variant="destructive">
            <AlertDescription>{errorMessage}</AlertDescription>
          </Alert>
        {/if}

        <Field>
          <FieldLabel for="new-password">New password</FieldLabel>
          <Input
            id="new-password"
            type="password"
            bind:value={newPassword}
            placeholder="••••••••"
            disabled={isLoading}
            required
            minlength={6}
            autocomplete="new-password"
          />
        </Field>

        <Field>
          <FieldLabel for="confirm-password">Confirm password</FieldLabel>
          <Input
            id="confirm-password"
            type="password"
            bind:value={confirmPassword}
            placeholder="••••••••"
            disabled={isLoading}
            required
            minlength={6}
            autocomplete="new-password"
          />
        </Field>

        <Button type="submit" disabled={isLoading} class="w-full">
          {#if isLoading}
            <div class="mr-2 h-4 w-4 animate-spin rounded-full border-b-2 border-current"></div>
          {/if}
          Set password
        </Button>
      </form>
    </div>
  </div>
</div>
