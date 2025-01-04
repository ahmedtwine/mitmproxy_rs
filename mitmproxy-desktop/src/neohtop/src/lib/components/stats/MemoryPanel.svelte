<script lang="ts">
  import { faMemory } from "@fortawesome/free-solid-svg-icons";
  import { PanelHeader, ProgressBar, StatItem } from "$lib/components";
  import { formatMemorySize, formatPercentage } from "$lib/utils";

  export let memoryTotal: number;
  export let memoryUsed: number;
  export let memoryFree: number;

  $: memoryPercentage = (memoryUsed / memoryTotal) * 100;
</script>

<div class="stat-panel">
  <PanelHeader
    icon={faMemory}
    title="Memory"
    usageValue={formatPercentage(memoryPercentage)}
  />
  <div class="stats-content">
    <div class="stat-item with-progress">
      <ProgressBar
        label="Memory usage"
        value={memoryPercentage}
        labelWidth="5rem"
        valueWidth="2.5rem"
      />
    </div>
    <StatItem label="Total" value={formatMemorySize(memoryTotal)} />
    <StatItem label="Used" value={formatMemorySize(memoryUsed)} />
    <StatItem label="Free" value={formatMemorySize(memoryFree)} />
  </div>
</div>

<style>
  .stat-panel {
    flex: 2;
    min-width: 0;
    background-color: var(--mantle);
    border-radius: 6px;
    padding: 0.75rem;
    display: flex;
    flex-direction: column;
  }

  .stats-content {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .stat-item.with-progress {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }
</style>
