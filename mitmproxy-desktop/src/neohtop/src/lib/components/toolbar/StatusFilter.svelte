<script lang="ts">
  import { STATUS_OPTIONS } from "$lib/constants";
  import type { AppConfig } from "$lib/types";
  import { settingsStore } from "$lib/stores";

  export let statusFilter: string = "all";

  function updateBehaviorConfig(key: keyof AppConfig["behavior"], value: any) {
    settingsStore.updateConfig({
      behavior: {
        ...$settingsStore.behavior,
        [key]: value,
      },
    });
  }
</script>

<div class="toolbar-group">
  <select
    bind:value={statusFilter}
    on:change={() => updateBehaviorConfig("defaultStatusFilter", statusFilter)}
    class="select-input"
  >
    {#each STATUS_OPTIONS as option}
      <option value={option.value}>{option.label}</option>
    {/each}
  </select>
</div>

<style>
  .select-input {
    height: 28px;
    padding: 0 8px;
    border: 1px solid var(--surface1);
    border-radius: 6px;
    background: var(--surface0);
    color: var(--text);
    font-size: 13px;
    cursor: pointer;
    appearance: none;
    padding-right: 24px;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23cdd6f4' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 8px center;
  }

  .select-input:hover {
    background-color: var(--surface1);
  }

  .select-input:focus {
    outline: none;
    border-color: var(--blue);
  }

  .select-input:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }
</style>
