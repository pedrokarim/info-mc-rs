<script lang="ts">
  export let username = 'Notch';
  export let tag = '';
  export let image = '/images/skins/skin-placeholder-head-v01.png';
  export let href = '';

  $: profileHref = href || `/player/${encodeURIComponent(username)}`;
</script>

<a class="skin-card" href={profileHref}>
  <img src={image} alt={`Head ${username}`} loading="lazy" />
  <h4>{username}</h4>
  {#if tag}
    <p>{tag}</p>
  {/if}
</a>
