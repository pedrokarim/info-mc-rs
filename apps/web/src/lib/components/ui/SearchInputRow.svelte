<script lang="ts">
  export let value = '';
  export let placeholder = '';
  export let required = true;
  export let disabled = false;
  export let actionLabel = 'Analyser';
  export let actionType: 'button' | 'submit' = 'submit';
  export let actionVariant: 'primary' | 'secondary' = 'primary';
  export let name = 'lookup';
</script>

<div class="input-row">
  <input class="input" bind:value name={name} {placeholder} {required} {disabled} />
  <button class={`btn btn-${actionVariant}`} type={actionType} {disabled}>{actionLabel}</button>
</div>
