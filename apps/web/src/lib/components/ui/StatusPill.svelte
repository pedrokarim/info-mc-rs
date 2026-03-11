<script lang="ts">
  export let label = 'En attente';
  export let kind: 'neutral' | 'online' | 'offline' | 'loading' = 'neutral';
</script>

<span class={`status ${kind}`}>{label}</span>
