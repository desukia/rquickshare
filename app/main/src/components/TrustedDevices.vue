<template>
  <div class="devices-panel">
    <h3>Meus Dispositivos</h3>
    
    <div class="devices-list">
      <div v-for="device in trustedDevices" :key="device.id" class="device-card">
        <div class="device-header">
          <span class="device-icon" :data-type="device.device_type">📱</span>
          <div class="device-info">
            <h4>{{ device.device_name }}</h4>
            <p class="device-os">{{ device.os_info }}</p>
          </div>
          <button v-if="!device.is_trusted" @click="trustDevice(device.id)" class="trust-btn">
            ✓ Confiar
          </button>
          <span v-else class="trusted-badge">✓ Confiável</span>
        </div>
        <div class="device-actions">
          <button @click="renameDevice(device)">Renomear</button>
          <button @click="removeDevice(device.id)" class="delete-btn">Remover</button>
        </div>
      </div>
    </div>

    <div class="new-device-section">
      <h4>Novo dispositivo disponível?</h4>
      <button @click="scanDevices" class="scan-btn">🔍 Procurar Dispositivos</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';

interface Device {
  id: string;
  device_name: string;
  device_type: string;
  os_info: string;
  is_trusted: boolean;
  last_seen: string;
}

const trustedDevices = ref<Device[]>([]);

const loadDevices = async () => {
  const response = await fetch('/api/devices');
  trustedDevices.value = await response.json();
};

const trustDevice = async (deviceId: string) => {
  await fetch(`/api/devices/${deviceId}/trust`, { method: 'PUT' });
  await loadDevices();
};

const removeDevice = async (deviceId: string) => {
  if (confirm('Remover este dispositivo?')) {
    await fetch(`/api/devices/${deviceId}`, { method: 'DELETE' });
    await loadDevices();
  }
};

const scanDevices = async () => {
  // Disparar varredura de mDNS/Bluetooth
  await fetch('/api/devices/scan', { method: 'POST' });
};

const renameDevice = async (device: Device) => {
  const newName = prompt('Novo nome:', device.device_name);
  if (newName) {
    await fetch(`/api/devices/${device.id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ device_name: newName }),
    });
    await loadDevices();
  }
};

onMounted(loadDevices);
</script>

<style scoped>
.devices-panel {
  padding: 1.5rem;
  border-radius: 8px;
  background: var(--surface-1);
}

.devices-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin: 1rem 0;
}

.device-card {
  padding: 1rem;
  border: 1px solid var(--border);
  border-radius: 8px;
  background: var(--surface-2);
}

.device-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 0.5rem;
}

.device-icon {
  font-size: 1.5rem;
}

.device-info h4 {
  margin: 0;
  font-weight: 600;
}

.device-os {
  margin: 0.25rem 0 0;
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.trust-btn, .delete-btn {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
}

.trust-btn {
  background: var(--text-success);
  color: white;
  margin-left: auto;
}

.delete-btn {
  background: var(--bg-danger);
  color: white;
}

.trusted-badge {
  margin-left: auto;
  padding: 0.5rem 1rem;
  background: var(--bg-success);
  color: var(--text-success);
  border-radius: 6px;
  font-weight: 600;
}

.device-actions {
  display: flex;
  gap: 0.5rem;
  justify-content: flex-end;
}

.scan-btn {
  padding: 0.75rem 1.5rem;
  background: var(--text-accent);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
}
</style>
