<script lang="ts">
  export let value = 0;
  export let max = 100;

  $: safeMax = max > 0 ? max : 1;
  $: percent = Math.max(0, Math.min(100, (value / safeMax) * 100));
</script>

<div class="bar" role="progressbar" aria-valuenow={Math.round(value)} aria-valuemin={0} aria-valuemax={safeMax}>
  <span style={`width: ${percent}%;`}></span>
</div>
