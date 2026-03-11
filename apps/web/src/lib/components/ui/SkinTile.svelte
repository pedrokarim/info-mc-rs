<script lang="ts">
  export let username = 'Notch';
  export let tag = '';
  export let rank: number | undefined = undefined;
  export let image = '/images/skins/skin-placeholder-full-v01.png';
  export let href = '';

  $: profileHref = href || `/player/${encodeURIComponent(username)}`;
</script>

<a class="skin-card" href={profileHref}>
  <div class="skin-card-img-wrap">
    {#if rank !== undefined}
      <span class="skin-rank">#{rank}</span>
    {/if}
    <img src={image} alt={`Skin ${username}`} loading="lazy" />
  </div>
  <h4>{username}</h4>
  {#if tag}
    <p>{tag}</p>
  {/if}
</a>

<style>
  .skin-card-img-wrap {
    position: relative;
  }
  .skin-rank {
    position: absolute;
    top: 0.3rem;
    left: 0.3rem;
    z-index: 1;
    font-size: 0.65rem;
    font-weight: 700;
    letter-spacing: 0.04em;
    color: rgba(255,255,255,0.82);
    background: rgba(0,0,0,0.42);
    border-radius: 4px;
    padding: 0.1em 0.38em;
    line-height: 1.4;
  }
</style>
