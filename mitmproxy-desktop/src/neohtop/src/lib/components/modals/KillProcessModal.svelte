<script lang="ts">
  import { Modal } from "$lib/components";

  interface Process {
    pid: number;
    name: string;
  }

  export let show = false;
  export let process: Process | null = null;
  export let onClose: () => void;
  export let onConfirm: () => Promise<void>;
  export let isKilling = false;
</script>

<Modal {show} title="Confirm Action" maxWidth="400px" {onClose}>
  {#if process}
    <div class="confirm-content">
      <p class="confirm-message">Are you sure you want to end this process?</p>
      <div class="process-info">
        <span class="process-name">{process.name}</span>
        <span class="process-pid">(PID: {process.pid})</span>
      </div>
      <div class="confirm-actions">
        <button class="btn-secondary" on:click={onClose} disabled={isKilling}>
          Cancel
        </button>
        <button class="btn-danger" on:click={onConfirm} disabled={isKilling}>
          {#if isKilling}
            <div class="spinner"></div>
            <span>Ending...</span>
          {:else}
            End Process
          {/if}
        </button>
      </div>
    </div>
  {/if}
</Modal>

<style>
  .confirm-content {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .confirm-message {
    color: var(--text);
    margin: 0;
    font-size: 14px;
  }

  .process-info {
    background: var(--mantle);
    padding: 12px;
    border-radius: 6px;
  }

  .process-name {
    color: var(--text);
    font-weight: 500;
    font-size: 14px;
  }

  .process-pid {
    color: var(--subtext0);
    font-size: 12px;
    margin-left: 8px;
  }

  .confirm-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
  }

  .btn-secondary {
    padding: 8px 16px;
    font-size: 13px;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-secondary:hover {
    background: var(--surface1);
  }

  .btn-danger {
    padding: 8px 16px;
    font-size: 13px;
    color: var(--base);
    background: var(--red);
    border: none;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .btn-danger:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .btn-danger:hover {
    background: color-mix(in srgb, var(--red) 90%, white);
  }

  .spinner {
    width: 16px;
  }
</style>
