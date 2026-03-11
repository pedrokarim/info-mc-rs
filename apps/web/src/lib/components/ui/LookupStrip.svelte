<script lang="ts">
  import SearchInputRow from '$lib/components/ui/SearchInputRow.svelte';
  import SectionHeading from '$lib/components/ui/SectionHeading.svelte';
  import TogglePills from '$lib/components/ui/TogglePills.svelte';

  export let heading = 'Lookup rapide';
  export let description = 'Entre une adresse ou un pseudo pour ouvrir la fiche detaillee.';
  export let action = 'Analyser';
  export let serverPlaceholder = 'play.hypixel.net ou 172.65.128.35:25565';
  export let playerPlaceholder = 'Notch, Dream ou UUID';

  let mode: 'server' | 'player' = 'server';
  let value = '';
  const modeOptions = [
    { label: 'Server Lookup', value: 'server' },
    { label: 'Player / Skin Lookup', value: 'player' }
  ];

  $: placeholder = mode === 'server' ? serverPlaceholder : playerPlaceholder;

  function submit(event: SubmitEvent) {
    event.preventDefault();
  }
</script>

<section class="lookup-panel section-strip">
  <SectionHeading title={heading} description={description} />

  <form class="lookup-form" onsubmit={submit}>
    <TogglePills options={modeOptions} bind:value={mode} name="lookup-mode" ariaLabel="Mode lookup" />
    <SearchInputRow bind:value={value} {placeholder} actionLabel={action} />
  </form>
</section>
