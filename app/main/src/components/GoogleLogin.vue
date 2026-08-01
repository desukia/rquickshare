<template>
  <div class="auth-container">
    <div v-if="!isAuthenticated" class="login-section">
      <h2>Conecte com sua conta Google</h2>
      <button @click="redirectToGoogle" class="google-login-btn">
        <img src="/google-logo.svg" alt="Google">
        <span>Login com Google</span>
      </button>
      <p class="info-text">
        Faça login para sincronizar contatos e permitir que seus dispositivos se reconheçam automaticamente.
      </p>
    </div>

    <div v-else class="logged-in-section">
      <div class="user-info">
        <img :src="userProfile.picture" :alt="userProfile.name" class="profile-pic">
        <div>
          <p class="user-name">{{ userProfile.name }}</p>
          <p class="user-email">{{ userProfile.email }}</p>
        </div>
        <button @click="logout" class="logout-btn">Sair</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';

interface UserProfile {
  name: string;
  email: string;
  picture: string;
  sub: string;
}

const isAuthenticated = ref(false);
const userProfile = ref<UserProfile | null>(null);

const redirectToGoogle = () => {
  // Chamar backend para obter URL de autenticação
  window.location.href = '/api/auth/google';
};

const logout = async () => {
  await fetch('/api/auth/logout', { method: 'POST' });
  isAuthenticated.value = false;
  userProfile.value = null;
};

onMounted(async () => {
  // Verificar se já está autenticado
  const response = await fetch('/api/auth/user');
  if (response.ok) {
    userProfile.value = await response.json();
    isAuthenticated.value = true;
  }
});
</script>

<style scoped>
.auth-container {
  padding: 2rem;
}

.login-section {
  text-align: center;
}

.google-login-btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  background: #fff;
  border: 1px solid #ddd;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
}

.google-login-btn:hover {
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.logged-in-section {
  background: var(--surface-2);
  padding: 1.5rem;
  border-radius: 8px;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.profile-pic {
  width: 48px;
  height: 48px;
  border-radius: 50%;
}

.user-name {
  font-weight: 600;
  margin: 0;
}

.user-email {
  color: var(--text-secondary);
  margin: 0;
  font-size: 0.9rem;
}

.logout-btn {
  margin-left: auto;
  padding: 0.5rem 1rem;
  background: var(--bg-danger);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
}
</style>
