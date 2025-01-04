<script lang="ts">
  export let label: string;
  export let value: number;
  export let labelWidth = "2.5rem";
  export let valueWidth = "2.5rem";

  function getUsageClass(usage: number): string {
    if (usage > 90) return "critical";
    if (usage > 75) return "high";
    if (usage > 50) return "medium";
    return "low";
  }
</script>

<div
  class="progress-container"
  style="--label-width: {labelWidth}; --value-width: {valueWidth}"
>
  <span class="label">{label}</span>
  <div class="bar-container">
    <div
      class="usage-bar {getUsageClass(value)}"
      style="transform: translateX({value - 100}%);"
    ></div>
  </div>
  <span class="value">{Math.round(value)}%</span>
</div>

<style>
  .progress-container {
    width: 100%;
    display: grid;
    grid-template-columns: var(--label-width) 1fr var(--value-width);
    align-items: center;
    gap: 0.3rem;
    font-size: 0.7rem;
  }

  .label {
    color: var(--subtext0);
    white-space: nowrap;
  }

  .value {
    color: var(--text);
    text-align: right;
    white-space: nowrap;
  }

  .bar-container {
    height: 8px;
    background: var(--surface0);
    border-radius: 4px;
    overflow: hidden;
    position: relative;
  }

  .usage-bar {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 100%;
    transition: transform 0.3s ease;
    transform-origin: left;
  }

  .usage-bar.low {
    background: var(--blue);
  }
  .usage-bar.medium {
    background: var(--yellow);
  }
  .usage-bar.high {
    background: var(--peach);
  }
  .usage-bar.critical {
    background: var(--red);
  }
</style>
