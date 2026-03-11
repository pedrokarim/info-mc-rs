<script lang="ts">
  export let html = '';
  export let fallback = 'No MOTD';
</script>

<div class="motd">
  {@html html || fallback}
</div>
