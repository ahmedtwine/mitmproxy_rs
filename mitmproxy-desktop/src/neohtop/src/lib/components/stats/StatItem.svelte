<script lang="ts">
  export let label: string;
  export let value: string;
</script>

<div class="stat-item">
  <span>{label}</span>
  <span>{value}</span>
</div>

<style>
  .stat-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.7rem;
    line-height: 1.2;
    margin: 0;
    padding: 0;
  }

  .stat-item span:first-child {
    color: var(--subtext0);
  }

  .stat-item span:last-child {
    color: var(--text);
    font-weight: 500;
  }
</style>
