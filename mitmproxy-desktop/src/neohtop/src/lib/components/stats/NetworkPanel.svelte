<script lang="ts">
  import { faNetworkWired } from "@fortawesome/free-solid-svg-icons";
  import { PanelHeader, StatItem } from "$lib/components";
  import { formatBytes } from "$lib/utils";

  export let networkRxBytes: number;
  export let networkTxBytes: number;
</script>

<div class="stat-panel">
  <PanelHeader icon={faNetworkWired} title="Network I/O" />
  <div class="network-stats">
    <StatItem label="↓ Receiving" value={formatBytes(networkRxBytes)} />
    <StatItem label="↑ Sending" value={formatBytes(networkTxBytes)} />
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

  .network-stats {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }
</style>
