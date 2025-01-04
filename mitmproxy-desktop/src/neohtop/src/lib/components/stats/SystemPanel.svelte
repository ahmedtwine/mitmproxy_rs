<script lang="ts">
  import { faServer } from "@fortawesome/free-solid-svg-icons";
  import { PanelHeader, StatItem } from "$lib/components";
  import { formatUptime } from "$lib/utils";

  export let uptime: number;
  export let loadAvg: [number, number, number];
</script>

<div class="stat-panel">
  <PanelHeader icon={faServer} title="System" />
  <div class="system-grid">
    <StatItem label="Uptime" value={formatUptime(uptime)} />
    <StatItem label="1m Load" value={loadAvg[0].toFixed(2)} />
    <StatItem label="5m Load" value={loadAvg[1].toFixed(2)} />
    <StatItem label="15m Load" value={loadAvg[2].toFixed(2)} />
  </div>
</div>

<style>
  .stat-panel {
    flex: 0.8;
    min-width: 125px;
    background-color: var(--mantle);
    border-radius: 6px;
    padding: 0.75rem;
    display: flex;
    flex-direction: column;
  }

  .system-grid {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    flex: 1;
  }
</style>
