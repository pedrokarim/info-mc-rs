<script lang="ts">
  import Card from '$lib/components/ui/Card.svelte';
  import {
    ITEM_CATEGORIES, ENCHANTMENTS, getCompatibleEnchants, isCompatible,
    calculateAnvilCost, optimizeOrder, toGiveCommand,
    type SelectedEnchant, type EnchantDef,
  } from '$lib/utils/enchantment';

  let selectedCategory = $state('sword');
  let selectedEnchants = $state<SelectedEnchant[]>([]);
  let copied = $state(false);

  let compatible = $derived(getCompatibleEnchants(selectedCategory));
  let anvilResult = $derived(calculateAnvilCost(selectedEnchants));
  let optimized = $derived(optimizeOrder(selectedEnchants));
  let optimizedResult = $derived(calculateAnvilCost(optimized));
  let command = $derived(toGiveCommand(selectedCategory, selectedEnchants));

  function addEnchant(def: EnchantDef) {
    if (selectedEnchants.find(e => e.id === def.id)) return;
    if (!isCompatible(def.id, selectedEnchants)) return;
    selectedEnchants = [...selectedEnchants, { id: def.id, level: def.maxLevel }];
  }

  function removeEnchant(id: string) {
    selectedEnchants = selectedEnchants.filter(e => e.id !== id);
  }

  function setLevel(id: string, level: number) {
    selectedEnchants = selectedEnchants.map(e => e.id === id ? { ...e, level } : e);
  }

  function applyOptimalOrder() {
    selectedEnchants = optimized;
  }

  function changeCategory(cat: string) {
    selectedCategory = cat;
    selectedEnchants = [];
  }

  async function copyCommand() {
    try {
      await navigator.clipboard.writeText(command);
      copied = true;
      setTimeout(() => { copied = false; }, 2000);
    } catch { /* fail */ }
  }

  function getDef(id: string): EnchantDef | undefined {
    return ENCHANTMENTS.find(e => e.id === id);
  }
</script>

<div class="ench-root">
  <div class="ench-layout">
    <!-- Left: Item + enchant list -->
    <div class="ench-sidebar">
      <Card variant="elevated" padding="md">
        <span class="section-label">Item</span>
        <div class="cat-grid">
          {#each ITEM_CATEGORIES as cat}
            <button
              class="cat-btn"
              class:active={selectedCategory === cat.id}
              onclick={() => changeCategory(cat.id)}
            >{cat.label}</button>
          {/each}
        </div>
      </Card>

      <Card variant="elevated" padding="md">
        <span class="section-label">Enchantements disponibles</span>
        <div class="ench-list">
          {#each compatible as def}
            {@const alreadyAdded = selectedEnchants.some(e => e.id === def.id)}
            {@const canAdd = !alreadyAdded && isCompatible(def.id, selectedEnchants)}
            <button
              class="ench-item"
              class:disabled={!canAdd}
              class:added={alreadyAdded}
              onclick={() => { if (canAdd) addEnchant(def); }}
              disabled={!canAdd}
            >
              <span class="ench-name">{def.label}</span>
              <span class="ench-max">max {def.maxLevel}</span>
            </button>
          {/each}
        </div>
      </Card>
    </div>

    <!-- Center: Selected enchants + anvil steps -->
    <div class="ench-main">
      <Card variant="elevated" padding="lg">
        <span class="section-label">Enchantements sélectionnés</span>

        {#if selectedEnchants.length === 0}
          <p class="empty-hint">Cliquez sur un enchantement à gauche pour l'ajouter.</p>
        {:else}
          <div class="selected-list">
            {#each selectedEnchants as ench}
              {@const def = getDef(ench.id)}
              {#if def}
                <div class="sel-item">
                  <span class="sel-name">{def.label}</span>
                  <div class="sel-level">
                    {#each Array(def.maxLevel) as _, lvl}
                      <button
                        class="lvl-btn"
                        class:active={ench.level === lvl + 1}
                        onclick={() => setLevel(ench.id, lvl + 1)}
                      >{lvl + 1}</button>
                    {/each}
                  </div>
                  <button class="sel-del" onclick={() => removeEnchant(ench.id)}>×</button>
                </div>
              {/if}
            {/each}
          </div>
        {/if}
      </Card>

      {#if selectedEnchants.length > 0}
        <Card variant="elevated" padding="lg">
          <div class="cost-header">
            <span class="section-label">Coût sur l'enclume (ordre actuel)</span>
            <span class="cost-total" class:too-expensive={anvilResult.tooExpensive}>
              {anvilResult.totalCost} niveaux
              {#if anvilResult.tooExpensive}
                <span class="too-exp-badge">Too Expensive!</span>
              {/if}
            </span>
          </div>
          <div class="step-list">
            {#each anvilResult.steps as step, i}
              <div class="step-item" class:too-expensive={step.tooExpensive}>
                <span class="step-num">{i + 1}.</span>
                <span class="step-label">{step.label}</span>
                <span class="step-cost">{step.stepCost} <span class="step-unit">niv.</span></span>
                {#if step.tooExpensive}
                  <span class="too-exp-icon" title="Dépasse 39 niveaux !">⚠</span>
                {/if}
              </div>
            {/each}
          </div>
        </Card>

        {#if selectedEnchants.length > 1}
          <Card variant="elevated" padding="lg">
            <div class="cost-header">
              <span class="section-label">Ordre optimal</span>
              <span class="cost-total" class:too-expensive={optimizedResult.tooExpensive}>
                {optimizedResult.totalCost} niveaux
              </span>
            </div>
            <div class="step-list">
              {#each optimizedResult.steps as step, i}
                <div class="step-item" class:too-expensive={step.tooExpensive}>
                  <span class="step-num">{i + 1}.</span>
                  <span class="step-label">{step.label}</span>
                  <span class="step-cost">{step.stepCost} <span class="step-unit">niv.</span></span>
                </div>
              {/each}
            </div>
            {#if anvilResult.totalCost !== optimizedResult.totalCost}
              <div class="savings">
                Économie : {anvilResult.totalCost - optimizedResult.totalCost} niveaux
              </div>
            {/if}
            <button class="apply-btn" onclick={applyOptimalOrder}>Appliquer cet ordre</button>
          </Card>
        {/if}

        <Card variant="elevated" padding="md">
          <span class="section-label">Commande</span>
          <div class="command-box">
            <code class="command-text">{command}</code>
            <button class="copy-btn" onclick={copyCommand}>{copied ? 'Copié !' : 'Copier'}</button>
          </div>
        </Card>
      {/if}
    </div>
  </div>
</div>

<style>
  .ench-root { display: flex; flex-direction: column; }
  .ench-layout { display: grid; grid-template-columns: 300px 1fr; gap: 1rem; }
  .ench-sidebar { display: flex; flex-direction: column; gap: 0.8rem; }
  .ench-main { display: flex; flex-direction: column; gap: 0.8rem; }

  .section-label { font-size: 0.65rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--ink-2, #5a7894); }

  /* Item categories */
  .cat-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 4px; margin-top: 0.3rem; }
  .cat-btn { padding: 0.35rem 0.5rem; font-size: 0.72rem; font-family: inherit; font-weight: 600; border: 1px solid rgba(70,113,166,0.3); border-radius: 5px; background: rgba(255,255,255,0.4); color: var(--ink-1); cursor: pointer; transition: all 120ms; }
  .cat-btn:hover { background: rgba(94,144,255,0.08); }
  .cat-btn.active { background: var(--blue-0, #5e90ff); color: #fff; border-color: var(--blue-0); }

  /* Enchant list */
  .ench-list { display: flex; flex-direction: column; gap: 2px; margin-top: 0.3rem; max-height: 400px; overflow-y: auto; }
  .ench-item { display: flex; justify-content: space-between; align-items: center; padding: 0.35rem 0.6rem; font-size: 0.75rem; font-family: inherit; border: 1px solid rgba(70,113,166,0.2); border-radius: 5px; background: rgba(255,255,255,0.3); cursor: pointer; transition: all 120ms; }
  .ench-item:hover:not(:disabled) { background: rgba(94,144,255,0.08); border-color: rgba(94,144,255,0.3); }
  .ench-item.disabled { opacity: 0.35; cursor: not-allowed; }
  .ench-item.added { background: rgba(94,144,255,0.1); border-color: var(--blue-0); }
  .ench-name { font-weight: 600; color: var(--ink-0); }
  .ench-max { font-size: 0.65rem; color: var(--ink-2); }

  /* Selected enchants */
  .empty-hint { font-size: 0.8rem; color: var(--ink-2); margin: 0.5rem 0; }
  .selected-list { display: flex; flex-direction: column; gap: 6px; margin-top: 0.5rem; }
  .sel-item { display: flex; align-items: center; gap: 8px; padding: 8px 10px; background: rgba(255,255,255,0.2); border-radius: 6px; }
  .sel-name { flex: 1; font-size: 0.82rem; font-weight: 600; color: var(--ink-0); }
  .sel-level { display: flex; gap: 2px; }
  .lvl-btn { width: 26px; height: 26px; border: 1px solid rgba(70,113,166,0.3); border-radius: 4px; background: rgba(255,255,255,0.4); font-size: 0.72rem; font-weight: 700; color: var(--ink-1); cursor: pointer; }
  .lvl-btn.active { background: var(--blue-0); color: #fff; border-color: var(--blue-0); }
  .sel-del { width: 26px; height: 26px; border: none; border-radius: 4px; background: rgba(184,59,59,0.1); color: var(--danger, #b83b3b); font-size: 0.85rem; cursor: pointer; display: flex; align-items: center; justify-content: center; }
  .sel-del:hover { background: rgba(184,59,59,0.2); }

  /* Cost display */
  .cost-header { display: flex; justify-content: space-between; align-items: center; }
  .cost-total { font-family: 'JetBrains Mono', monospace; font-size: 0.85rem; font-weight: 700; color: var(--ink-0); }
  .cost-total.too-expensive { color: var(--danger, #b83b3b); }
  .too-exp-badge { font-size: 0.6rem; padding: 1px 5px; border-radius: 3px; background: rgba(184,59,59,0.15); color: var(--danger); font-weight: 700; margin-left: 4px; }

  .step-list { display: flex; flex-direction: column; gap: 3px; margin-top: 0.5rem; }
  .step-item { display: flex; align-items: center; gap: 6px; padding: 5px 8px; border-radius: 4px; font-size: 0.78rem; }
  .step-item.too-expensive { background: rgba(184,59,59,0.08); }
  .step-num { font-weight: 700; color: var(--ink-2); width: 20px; }
  .step-label { flex: 1; font-weight: 600; color: var(--ink-0); }
  .step-cost { font-family: 'JetBrains Mono', monospace; font-weight: 700; color: var(--ink-0); }
  .step-unit { font-size: 0.6rem; color: var(--ink-2); font-weight: 400; }
  .too-exp-icon { color: var(--danger); font-size: 0.85rem; }

  .savings { font-size: 0.78rem; font-weight: 700; color: var(--ok, #169a60); margin-top: 0.4rem; text-align: center; }
  .apply-btn { margin-top: 0.5rem; padding: 0.45rem 1rem; border-radius: 6px; border: none; background: var(--blue-0, #5e90ff); color: #fff; font-family: inherit; font-size: 0.78rem; font-weight: 600; cursor: pointer; transition: background 120ms; }
  .apply-btn:hover { background: var(--blue-1, #345fcd); }

  /* Command */
  .command-box { display: flex; align-items: center; gap: 8px; background: #1a1a2e; border-radius: 6px; padding: 8px 10px; overflow-x: auto; margin-top: 0.3rem; }
  .command-text { flex: 1; font-family: 'JetBrains Mono', monospace; font-size: 0.68rem; color: #e0e0e0; word-break: break-all; }
  .copy-btn { padding: 0.3rem 0.7rem; border-radius: 4px; border: 1px solid rgba(255,255,255,0.2); background: rgba(94,144,255,0.2); color: #fff; font-family: 'Chakra Petch', sans-serif; font-size: 0.68rem; font-weight: 600; cursor: pointer; flex-shrink: 0; }
  .copy-btn:hover { background: rgba(94,144,255,0.4); }

  @media (max-width: 768px) {
    .ench-layout { grid-template-columns: 1fr; }
  }
</style>
