<script lang="ts">
  import Card from '$lib/components/ui/Card.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import {
    COMMANDS, ITEMS, BLOCKS, ENTITIES, PARTICLES, EFFECTS,
    GAMEMODES, WEATHER_TYPES, TIME_PRESETS, TARGETS,
    SETBLOCK_MODES, FILL_MODES, fuzzyFilter,
    buildGive, buildSummon, buildSetblock, buildFill,
    buildParticle, buildEffect, buildTp, buildGamemode,
    buildTime, buildWeather,
    type CommandId,
  } from '$lib/utils/command-generator';

  let activeCmd = $state<CommandId>('give');
  let copied = $state(false);

  // ── Per-command state ──
  let give = $state({ target: '@p', item: 'diamond_sword', count: '1' });
  let summon = $state({ entity: 'zombie', x: '~', y: '~', z: '~' });
  let setblock = $state({ x: '~', y: '~', z: '~', block: 'stone', mode: 'replace' });
  let fill = $state({ x1: '~', y1: '~', z1: '~', x2: '~', y2: '~10', z2: '~10', block: 'stone', mode: 'replace' });
  let particle = $state({ name: 'flame', x: '~', y: '~1', z: '~', dx: '0.5', dy: '0.5', dz: '0.5', speed: '0', count: '10' });
  let effect = $state({ target: '@p', effect: 'speed', duration: '30', amplifier: '0' });
  let tp = $state({ target: '@p', x: '0', y: '100', z: '0' });
  let gamemode = $state({ mode: 'creative', target: '@p' });
  let time = $state({ value: 'day' });
  let weather = $state({ type: 'clear', duration: '' });

  // ── Search state ──
  let itemSearch = $state('');
  let blockSearch = $state('');
  let entitySearch = $state('');
  let particleSearch = $state('');
  let effectSearch = $state('');

  // ── Derived ──
  let command = $derived.by(() => {
    switch (activeCmd) {
      case 'give': return buildGive(give);
      case 'summon': return buildSummon(summon);
      case 'setblock': return buildSetblock(setblock);
      case 'fill': return buildFill(fill);
      case 'particle': return buildParticle(particle);
      case 'effect': return buildEffect(effect);
      case 'tp': return buildTp(tp);
      case 'gamemode': return buildGamemode(gamemode);
      case 'time': return buildTime(time);
      case 'weather': return buildWeather(weather);
    }
  });

  let filteredItems = $derived(fuzzyFilter(ITEMS, itemSearch));
  let filteredBlocks = $derived(fuzzyFilter(BLOCKS, blockSearch));
  let filteredEntities = $derived(fuzzyFilter(ENTITIES, entitySearch));
  let filteredParticles = $derived(fuzzyFilter(PARTICLES, particleSearch));
  let filteredEffects = $derived(fuzzyFilter(EFFECTS, effectSearch));

  let cmdDef = $derived(COMMANDS.find(c => c.id === activeCmd)!);

  async function copyCommand() {
    try {
      await navigator.clipboard.writeText(command);
      copied = true;
      setTimeout(() => { copied = false; }, 2000);
    } catch { /* fail */ }
  }
</script>

<div class="cmd-root">
  <!-- Command selector -->
  <div class="cmd-tabs">
    {#each COMMANDS as cmd}
      <button class="cmd-tab" class:active={activeCmd === cmd.id} onclick={() => { activeCmd = cmd.id; }}>
        {cmd.label}
      </button>
    {/each}
  </div>

  <!-- Syntax hint -->
  <div class="syntax-hint">
    <code>{cmdDef.syntax}</code>
  </div>

  <div class="cmd-layout">
    <!-- Fields -->
    <Card variant="elevated" padding="lg">
      <div class="fields">
        {#if activeCmd === 'give'}
          <div class="field-group">
            <span class="field-label">Cible</span>
            <div class="pill-row">
              {#each TARGETS as t}
                <button class="pill" class:active={give.target === t} onclick={() => { give.target = t; }}>{t}</button>
              {/each}
            </div>
          </div>
          <div class="field-group">
            <span class="field-label">Item</span>
            <input class="search-input" type="text" placeholder="Rechercher..." bind:value={itemSearch} />
            <div class="id-grid">
              {#each filteredItems as item}
                <button class="id-btn" class:active={give.item === item} onclick={() => { give.item = item; }}>{item}</button>
              {/each}
            </div>
          </div>
          <Input label="Quantité" type="number" bind:value={give.count} />

        {:else if activeCmd === 'summon'}
          <div class="field-group">
            <span class="field-label">Entité</span>
            <input class="search-input" type="text" placeholder="Rechercher..." bind:value={entitySearch} />
            <div class="id-grid">
              {#each filteredEntities as ent}
                <button class="id-btn" class:active={summon.entity === ent} onclick={() => { summon.entity = ent; }}>{ent}</button>
              {/each}
            </div>
          </div>
          <div class="coord-row">
            <Input label="X" bind:value={summon.x} placeholder="~" />
            <Input label="Y" bind:value={summon.y} placeholder="~" />
            <Input label="Z" bind:value={summon.z} placeholder="~" />
          </div>

        {:else if activeCmd === 'setblock'}
          <div class="coord-row">
            <Input label="X" bind:value={setblock.x} placeholder="~" />
            <Input label="Y" bind:value={setblock.y} placeholder="~" />
            <Input label="Z" bind:value={setblock.z} placeholder="~" />
          </div>
          <div class="field-group">
            <span class="field-label">Bloc</span>
            <input class="search-input" type="text" placeholder="Rechercher..." bind:value={blockSearch} />
            <div class="id-grid">
              {#each filteredBlocks as b}
                <button class="id-btn" class:active={setblock.block === b} onclick={() => { setblock.block = b; }}>{b}</button>
              {/each}
            </div>
          </div>
          <div class="field-group">
            <span class="field-label">Mode</span>
            <div class="pill-row">
              {#each SETBLOCK_MODES as m}
                <button class="pill" class:active={setblock.mode === m} onclick={() => { setblock.mode = m; }}>{m}</button>
              {/each}
            </div>
          </div>

        {:else if activeCmd === 'fill'}
          <div class="field-group">
            <span class="field-label">Coin 1</span>
            <div class="coord-row">
              <Input label="X" bind:value={fill.x1} placeholder="~" />
              <Input label="Y" bind:value={fill.y1} placeholder="~" />
              <Input label="Z" bind:value={fill.z1} placeholder="~" />
            </div>
          </div>
          <div class="field-group">
            <span class="field-label">Coin 2</span>
            <div class="coord-row">
              <Input label="X" bind:value={fill.x2} placeholder="~" />
              <Input label="Y" bind:value={fill.y2} placeholder="~" />
              <Input label="Z" bind:value={fill.z2} placeholder="~" />
            </div>
          </div>
          <div class="field-group">
            <span class="field-label">Bloc</span>
            <input class="search-input" type="text" placeholder="Rechercher..." bind:value={blockSearch} />
            <div class="id-grid">
              {#each filteredBlocks as b}
                <button class="id-btn" class:active={fill.block === b} onclick={() => { fill.block = b; }}>{b}</button>
              {/each}
            </div>
          </div>
          <div class="field-group">
            <span class="field-label">Mode</span>
            <div class="pill-row">
              {#each FILL_MODES as m}
                <button class="pill" class:active={fill.mode === m} onclick={() => { fill.mode = m; }}>{m}</button>
              {/each}
            </div>
          </div>

        {:else if activeCmd === 'particle'}
          <div class="field-group">
            <span class="field-label">Particule</span>
            <input class="search-input" type="text" placeholder="Rechercher..." bind:value={particleSearch} />
            <div class="id-grid">
              {#each filteredParticles as p}
                <button class="id-btn" class:active={particle.name === p} onclick={() => { particle.name = p; }}>{p}</button>
              {/each}
            </div>
          </div>
          <div class="field-group">
            <span class="field-label">Position</span>
            <div class="coord-row">
              <Input label="X" bind:value={particle.x} placeholder="~" />
              <Input label="Y" bind:value={particle.y} placeholder="~" />
              <Input label="Z" bind:value={particle.z} placeholder="~" />
            </div>
          </div>
          <div class="field-group">
            <span class="field-label">Delta (zone)</span>
            <div class="coord-row">
              <Input label="dX" bind:value={particle.dx} placeholder="0" />
              <Input label="dY" bind:value={particle.dy} placeholder="0" />
              <Input label="dZ" bind:value={particle.dz} placeholder="0" />
            </div>
          </div>
          <div class="coord-row">
            <Input label="Vitesse" bind:value={particle.speed} placeholder="0" />
            <Input label="Nombre" bind:value={particle.count} placeholder="1" />
          </div>

        {:else if activeCmd === 'effect'}
          <div class="field-group">
            <span class="field-label">Cible</span>
            <div class="pill-row">
              {#each TARGETS as t}
                <button class="pill" class:active={effect.target === t} onclick={() => { effect.target = t; }}>{t}</button>
              {/each}
            </div>
          </div>
          <div class="field-group">
            <span class="field-label">Effet</span>
            <input class="search-input" type="text" placeholder="Rechercher..." bind:value={effectSearch} />
            <div class="id-grid">
              {#each filteredEffects as e}
                <button class="id-btn" class:active={effect.effect === e} onclick={() => { effect.effect = e; }}>{e}</button>
              {/each}
            </div>
          </div>
          <div class="coord-row">
            <Input label="Durée (s)" bind:value={effect.duration} placeholder="30" />
            <Input label="Amplificateur" bind:value={effect.amplifier} placeholder="0" />
          </div>

        {:else if activeCmd === 'tp'}
          <div class="field-group">
            <span class="field-label">Cible</span>
            <div class="pill-row">
              {#each TARGETS as t}
                <button class="pill" class:active={tp.target === t} onclick={() => { tp.target = t; }}>{t}</button>
              {/each}
            </div>
          </div>
          <div class="coord-row">
            <Input label="X" bind:value={tp.x} placeholder="0" />
            <Input label="Y" bind:value={tp.y} placeholder="100" />
            <Input label="Z" bind:value={tp.z} placeholder="0" />
          </div>

        {:else if activeCmd === 'gamemode'}
          <div class="field-group">
            <span class="field-label">Mode</span>
            <div class="pill-row">
              {#each GAMEMODES as m}
                <button class="pill" class:active={gamemode.mode === m} onclick={() => { gamemode.mode = m; }}>{m}</button>
              {/each}
            </div>
          </div>
          <div class="field-group">
            <span class="field-label">Cible</span>
            <div class="pill-row">
              {#each TARGETS as t}
                <button class="pill" class:active={gamemode.target === t} onclick={() => { gamemode.target = t; }}>{t}</button>
              {/each}
            </div>
          </div>

        {:else if activeCmd === 'time'}
          <div class="field-group">
            <span class="field-label">Heure</span>
            <div class="pill-row pill-row--wrap">
              {#each TIME_PRESETS as tp}
                <button class="pill" class:active={time.value === tp.value} onclick={() => { time.value = tp.value; }}>{tp.label}</button>
              {/each}
            </div>
            <Input label="Valeur personnalisée" bind:value={time.value} placeholder="day" />
          </div>

        {:else if activeCmd === 'weather'}
          <div class="field-group">
            <span class="field-label">Type</span>
            <div class="pill-row">
              {#each WEATHER_TYPES as w}
                <button class="pill" class:active={weather.type === w} onclick={() => { weather.type = w; }}>{w}</button>
              {/each}
            </div>
          </div>
          <Input label="Durée (s, optionnel)" bind:value={weather.duration} placeholder="" />
        {/if}
      </div>
    </Card>

    <!-- Command output -->
    <Card variant="elevated" padding="lg">
      <span class="section-label">Commande générée</span>
      <div class="command-box">
        <code class="command-text">{command}</code>
      </div>
      <button class="copy-btn-big" onclick={copyCommand}>
        {copied ? 'Copié !' : 'Copier la commande'}
      </button>
    </Card>
  </div>
</div>

<style>
  .cmd-root { display: flex; flex-direction: column; gap: 1rem; }

  /* Command tabs */
  .cmd-tabs { display: flex; flex-wrap: wrap; gap: 4px; }
  .cmd-tab {
    padding: 0.4rem 0.7rem; font-family: 'JetBrains Mono', monospace; font-size: 0.75rem; font-weight: 600;
    border: 1px solid rgba(70,113,166,0.3); border-radius: 6px;
    background: rgba(255,255,255,0.4); color: var(--ink-1); cursor: pointer; transition: all 120ms;
  }
  .cmd-tab:hover { background: rgba(94,144,255,0.08); }
  .cmd-tab.active { background: var(--blue-0, #5e90ff); color: #fff; border-color: var(--blue-0); }

  .syntax-hint {
    font-family: 'JetBrains Mono', monospace; font-size: 0.72rem; color: var(--ink-2);
    background: rgba(0,0,0,0.03); padding: 6px 10px; border-radius: 6px;
  }

  .cmd-layout { display: flex; flex-direction: column; gap: 1rem; }

  .fields { display: flex; flex-direction: column; gap: 0.8rem; }

  .section-label { font-size: 0.65rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--ink-2, #5a7894); }
  .field-label { font-size: 0.68rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.04em; color: var(--ink-1, #2d4a65); }
  .field-group { display: flex; flex-direction: column; gap: 0.3rem; }

  .coord-row { display: grid; grid-template-columns: repeat(3, 1fr); gap: 0.5rem; }

  /* Pills */
  .pill-row { display: flex; gap: 4px; flex-wrap: wrap; }
  .pill-row--wrap { flex-wrap: wrap; }
  .pill {
    padding: 0.35rem 0.6rem; font-size: 0.72rem; font-family: 'JetBrains Mono', monospace; font-weight: 600;
    border: 1px solid rgba(70,113,166,0.3); border-radius: 5px;
    background: rgba(255,255,255,0.4); color: var(--ink-1); cursor: pointer; transition: all 120ms;
  }
  .pill:hover { background: rgba(94,144,255,0.08); }
  .pill.active { background: var(--blue-0); color: #fff; border-color: var(--blue-0); }

  /* ID search + grid */
  .search-input {
    width: 100%; box-sizing: border-box; padding: 0.4rem 0.6rem;
    border: 1px solid rgba(70,113,166,0.35); border-radius: 6px;
    background: rgba(255,255,255,0.8); font-size: 0.78rem; font-family: inherit;
    color: var(--ink-0); outline: none;
  }
  .search-input:focus { border-color: var(--blue-0); outline: 2px solid rgba(94,144,255,0.3); outline-offset: 1px; }

  .id-grid { display: flex; flex-wrap: wrap; gap: 3px; max-height: 180px; overflow-y: auto; }
  .id-btn {
    padding: 0.25rem 0.5rem; font-size: 0.65rem; font-family: 'JetBrains Mono', monospace;
    border: 1px solid rgba(70,113,166,0.2); border-radius: 4px;
    background: rgba(255,255,255,0.3); color: var(--ink-1); cursor: pointer; transition: all 100ms;
  }
  .id-btn:hover { background: rgba(94,144,255,0.08); border-color: rgba(94,144,255,0.3); }
  .id-btn.active { background: var(--blue-0); color: #fff; border-color: var(--blue-0); }

  /* Command output */
  .command-box {
    background: #1a1a2e; border-radius: 6px; padding: 12px 14px; margin-top: 0.4rem;
    overflow-x: auto;
  }
  .command-text { font-family: 'JetBrains Mono', monospace; font-size: 0.82rem; color: #e0e0e0; word-break: break-all; }

  .copy-btn-big {
    margin-top: 0.6rem; padding: 0.5rem 1.2rem; border-radius: 6px; border: none;
    background: var(--blue-0, #5e90ff); color: #fff;
    font-family: 'Chakra Petch', sans-serif; font-size: 0.82rem; font-weight: 600;
    cursor: pointer; transition: background 120ms; align-self: flex-start;
  }
  .copy-btn-big:hover { background: var(--blue-1, #345fcd); }
</style>
