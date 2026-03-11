<script lang="ts">
  export let message = '';
  export let kind: 'error' | 'info' = 'error';
</script>

<div class={`notice${kind === 'info' ? ' notice-info' : ''}`}>{message}</div>
