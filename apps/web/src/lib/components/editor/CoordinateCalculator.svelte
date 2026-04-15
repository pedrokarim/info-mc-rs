<script lang="ts">
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';
  import Card from '$lib/components/ui/Card.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Tabs from '$lib/components/ui/Tabs.svelte';
  import {
    createSeedMapStore, TILE_SIZE, randomSeed,
    type SeedMapStore,
  } from '$lib/stores/seed-map.svelte';

  type Dimension = 'overworld' | 'nether' | 'end';
  type MarkerDef = { x: number; z: number; color: string; label: string };

  const LS_KEY = 'coord-calc-state';

  let activeTab = $state('converter');
  let seedInput = $state('');

  /* ── Converter ── */
  let fromDimension: Dimension = $state('overworld');
  let toDimension: Dimension = $state('nether');
  let fromX = $state('');
  let fromY = $state('');
  let fromZ = $state('');
  const NETHER_RATIO = 8;

  let converted = $derived.by(() => {
    const x = parseFloat(fromX);
    const y = parseFloat(fromY);
    const z = parseFloat(fromZ);
    if (isNaN(x) && isNaN(z)) return null;
    const cx = isNaN(x) ? 0 : x;
    const cy = isNaN(y) ? 0 : y;
    const cz = isNaN(z) ? 0 : z;
    let rx = cx, rz = cz;
    if (fromDimension === 'overworld' && toDimension === 'nether') {
      rx = Math.floor(cx / NETHER_RATIO);
      rz = Math.floor(cz / NETHER_RATIO);
    } else if (fromDimension === 'nether' && toDimension === 'overworld') {
      rx = cx * NETHER_RATIO;
      rz = cz * NETHER_RATIO;
    }
    return { x: rx, y: cy, z: rz, srcX: cx, srcZ: cz };
  });

  /* ── Chunk / Region ── */
  let chunkX = $state('');
  let chunkZ = $state('');
  let chunkInfo = $derived.by(() => {
    const x = parseFloat(chunkX);
    const z = parseFloat(chunkZ);
    if (isNaN(x) && isNaN(z)) return null;
    const bx = isNaN(x) ? 0 : x;
    const bz = isNaN(z) ? 0 : z;
    const cx = Math.floor(bx / 16);
    const cz = Math.floor(bz / 16);
    const rx = Math.floor(cx / 32);
    const rz = Math.floor(cz / 32);
    return {
      block: { x: bx, z: bz },
      chunk: { x: cx, z: cz },
      region: { x: rx, z: rz },
      regionFile: `r.${rx}.${rz}.mca`,
      localBlock: { x: Math.floor(((bx % 16) + 16) % 16), z: Math.floor(((bz % 16) + 16) % 16) },
    };
  });

  /* ── Distance ── */
  let distX1 = $state(''), distY1 = $state(''), distZ1 = $state('');
  let distX2 = $state(''), distY2 = $state(''), distZ2 = $state('');
  let distance = $derived.by(() => {
    const x1 = parseFloat(distX1), z1 = parseFloat(distZ1);
    const x2 = parseFloat(distX2), z2 = parseFloat(distZ2);
    if (isNaN(x1) || isNaN(z1) || isNaN(x2) || isNaN(z2)) return null;
    const y1 = isNaN(parseFloat(distY1)) ? 0 : parseFloat(distY1);
    const y2 = isNaN(parseFloat(distY2)) ? 0 : parseFloat(distY2);
    const dx = x2 - x1, dy = y2 - y1, dz = z2 - z1;
    return {
      dist2d: Math.sqrt(dx * dx + dz * dz).toFixed(2),
      dist3d: Math.sqrt(dx * dx + dy * dy + dz * dz).toFixed(2),
      dx, dy, dz, p1: { x: x1, z: z1 }, p2: { x: x2, z: z2 },
    };
  });

  /* ── Spawn chunks ── */
  let spawnX = $state(''), spawnZ = $state('');
  let spawnChunks = $derived.by(() => {
    const x = parseFloat(spawnX), z = parseFloat(spawnZ);
    if (isNaN(x) || isNaN(z)) return null;
    const ccx = Math.floor(x / 16), ccz = Math.floor(z / 16);
    return {
      center: { x: ccx, z: ccz }, spawnBlock: { x, z },
      from: { x: (ccx - 9) * 16, z: (ccz - 9) * 16 },
      to: { x: (ccx + 9) * 16 + 15, z: (ccz + 9) * 16 + 15 },
      lazyFrom: { x: (ccx - 11) * 16, z: (ccz - 11) * 16 },
      lazyTo: { x: (ccx + 11) * 16 + 15, z: (ccz + 11) * 16 + 15 },
    };
  });

  const tabs = [
    { label: 'Convertisseur', value: 'converter' },
    { label: 'Chunk / Région', value: 'chunk' },
    { label: 'Distance', value: 'distance' },
    { label: 'Spawn Chunks', value: 'spawn' },
  ];

  const dimensions: { label: string; value: Dimension }[] = [
    { label: 'Overworld', value: 'overworld' },
    { label: 'Nether', value: 'nether' },
    { label: 'End', value: 'end' },
  ];

  function swapDimensions() {
    [fromDimension, toDimension] = [toDimension, fromDimension];
  }

  /* ═══════════════════════════════════════════════
     MINI-MAP CANVAS ENGINE
     Each map panel has its own camera & canvas ref.
     ═══════════════════════════════════════════════ */

  const DIM_COLORS: Record<Dimension, { bg: string; grid: string; accent: string; label: string }> = {
    overworld: { bg: '#1a2a1a', grid: 'rgba(80,180,80,0.12)', accent: '#55cc55', label: 'Overworld' },
    nether:    { bg: '#2a1a1a', grid: 'rgba(200,80,60,0.12)', accent: '#dd5533', label: 'Nether' },
    end:       { bg: '#1a1a2e', grid: 'rgba(180,140,255,0.12)', accent: '#bb88ff', label: 'The End' },
  };

  // We have up to 2 canvases (for converter dual-map) + 1 for other tabs
  interface MapPanel {
    el?: HTMLCanvasElement;
    w: number; h: number;
    camX: number; camZ: number; camZoom: number;
    dragging: boolean;
    dragStartX: number; dragStartY: number;
    dragCamX: number; dragCamZ: number;
    hoverX: number; hoverZ: number; hoverActive: boolean;
    dim: Dimension;
    markers: MarkerDef[];
  }

  function createPanel(dim: Dimension): MapPanel {
    return {
      w: 0, h: 0, camX: 0, camZ: 0, camZoom: 0.5,
      dragging: false, dragStartX: 0, dragStartY: 0, dragCamX: 0, dragCamZ: 0,
      hoverX: 0, hoverZ: 0, hoverActive: false,
      dim, markers: [],
    };
  }

  // Origin & destination panels (converter)
  let panelFrom = $state(createPanel('overworld'));
  let panelTo = $state(createPanel('nether'));
  // Single panel (other tabs)
  let panelSingle = $state(createPanel('overworld'));

  let tileCanvas: OffscreenCanvas | null = null;

  // Two independent seed-map store instances (own workers, own tileCache each)
  let storeFrom: SeedMapStore | null = null;
  let storeTo: SeedMapStore | null = null;
  let lastTileRequest = 0;

  function getStoreForPanel(p: MapPanel): SeedMapStore | null {
    if (p === panelFrom) return storeFrom;
    if (p === panelTo) return storeTo;
    return storeFrom; // single panel uses storeFrom
  }

  function handleSeedChange(value: string) {
    seedInput = value;
    if (value.trim()) {
      storeFrom?.setSeed(value);
      storeTo?.setSeed(value);
    }
  }

  // ── Panel interaction helpers ──

  function w2s(p: MapPanel, wx: number, wz: number) {
    return { x: p.w / 2 + (wx - p.camX) * p.camZoom, y: p.h / 2 + (wz - p.camZ) * p.camZoom };
  }

  function s2w(p: MapPanel, sx: number, sy: number) {
    return { x: p.camX + (sx - p.w / 2) / p.camZoom, z: p.camZ + (sy - p.h / 2) / p.camZoom };
  }

  function onWheel(p: MapPanel, e: WheelEvent) {
    e.preventDefault();
    p.camZoom = Math.max(0.01, Math.min(32, p.camZoom * (e.deltaY > 0 ? 0.85 : 1.18)));
  }

  function onPointerDown(p: MapPanel, e: PointerEvent) {
    p.dragging = true;
    p.dragStartX = e.clientX; p.dragStartY = e.clientY;
    p.dragCamX = p.camX; p.dragCamZ = p.camZ;
    (e.target as HTMLElement).setPointerCapture(e.pointerId);
  }

  function onPointerMove(p: MapPanel, e: PointerEvent) {
    const rect = p.el?.getBoundingClientRect();
    if (rect) {
      const wc = s2w(p, e.clientX - rect.left, e.clientY - rect.top);
      p.hoverX = Math.floor(wc.x); p.hoverZ = Math.floor(wc.z); p.hoverActive = true;
    }
    if (!p.dragging) return;
    p.camX = p.dragCamX - (e.clientX - p.dragStartX) / p.camZoom;
    p.camZ = p.dragCamZ - (e.clientY - p.dragStartY) / p.camZoom;
  }

  function onPointerUp(p: MapPanel) { p.dragging = false; }
  function onPointerLeave(p: MapPanel) { p.hoverActive = false; p.dragging = false; }

  function centerPanel(p: MapPanel, markers: MarkerDef[], extraBounds?: { minX: number; maxX: number; minZ: number; maxZ: number }) {
    if (markers.length === 0 && !extraBounds) return;
    let minX = Infinity, maxX = -Infinity, minZ = Infinity, maxZ = -Infinity;
    if (extraBounds) { minX = extraBounds.minX; maxX = extraBounds.maxX; minZ = extraBounds.minZ; maxZ = extraBounds.maxZ; }
    for (const m of markers) {
      minX = Math.min(minX, m.x); maxX = Math.max(maxX, m.x);
      minZ = Math.min(minZ, m.z); maxZ = Math.max(maxZ, m.z);
    }
    p.camX = (minX + maxX) / 2;
    p.camZ = (minZ + maxZ) / 2;
    const range = Math.max(maxX - minX, maxZ - minZ, 64);
    p.camZoom = Math.max(0.01, Math.min(32, Math.min(p.w || 400, p.h || 400) / (range * 1.6)));
  }

  // ── Drawing ──

  function renderPanel(p: MapPanel) {
    if (!p.el) return;
    const ctx = p.el.getContext('2d');
    if (!ctx) return;
    const dpr = window.devicePixelRatio || 1;
    const w = p.el.clientWidth, h = p.el.clientHeight;
    if (w !== p.w || h !== p.h) { p.w = w; p.h = h; p.el.width = w * dpr; p.el.height = h * dpr; }
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

    const colors = DIM_COLORS[p.dim];

    // Background
    ctx.fillStyle = colors.bg;
    ctx.fillRect(0, 0, w, h);

    // Biome tiles
    drawBiomeTiles(ctx, p);

    // Grid
    drawGrid(ctx, p, colors);
    // Origin crosshair
    drawCrosshair(ctx, p, w, h);
    // Markers
    for (const m of p.markers) drawMarker(ctx, p, m);
    // Dim label
    ctx.font = '700 11px "Teko", sans-serif';
    ctx.fillStyle = colors.accent;
    ctx.globalAlpha = 0.6;
    ctx.fillText(colors.label.toUpperCase(), 10, 18);
    ctx.globalAlpha = 1;
    // Hover
    if (p.hoverActive) drawHoverInfo(ctx, p, w, h);
  }

  function drawBiomeTiles(ctx: CanvasRenderingContext2D, p: MapPanel) {
    const store = getStoreForPanel(p);
    if (!store || !store.state.seedValid || store.state.tileCache.size === 0) return;
    const cache = store.state.tileCache;
    const step = store.getStep();
    const tws = TILE_SIZE * step;
    const halfW = p.w / 2 / p.camZoom, halfH = p.h / 2 / p.camZoom;
    const minTX = Math.floor((p.camX - halfW) / tws) - 1, maxTX = Math.floor((p.camX + halfW) / tws) + 1;
    const minTZ = Math.floor((p.camZ - halfH) / tws) - 1, maxTZ = Math.floor((p.camZ + halfH) / tws) + 1;
    if (!tileCanvas) tileCanvas = new OffscreenCanvas(TILE_SIZE, TILE_SIZE);
    const tc = tileCanvas.getContext('2d');
    if (!tc) return;
    ctx.imageSmoothingEnabled = false;
    for (let tx = minTX; tx <= maxTX; tx++) {
      for (let tz = minTZ; tz <= maxTZ; tz++) {
        const tile = cache.get(`${tx},${tz},${step}`);
        if (!tile) continue;
        tc.putImageData(new ImageData(new Uint8ClampedArray(tile.rgba.buffer as ArrayBuffer, tile.rgba.byteOffset, tile.rgba.byteLength), TILE_SIZE, TILE_SIZE), 0, 0);
        const sp = w2s(p, tx * tws, tz * tws);
        ctx.drawImage(tileCanvas, sp.x, sp.y, tws * p.camZoom, tws * p.camZoom);
      }
    }
    ctx.imageSmoothingEnabled = true;
  }

  function drawGrid(ctx: CanvasRenderingContext2D, p: MapPanel, colors: { grid: string }) {
    const chunkSize = 16, regionSize = 512;
    let blockStep = p.camZoom >= 4 ? 1 : p.camZoom >= 1 ? chunkSize : regionSize;
    if (blockStep * p.camZoom < 3) return;
    const tl = s2w(p, 0, 0), br = s2w(p, p.w, p.h);
    const startX = Math.floor(tl.x / blockStep) * blockStep;
    const startZ = Math.floor(tl.z / blockStep) * blockStep;
    ctx.strokeStyle = colors.grid; ctx.lineWidth = 0.5; ctx.beginPath();
    for (let bx = startX; bx <= br.x; bx += blockStep) { const sx = w2s(p, bx, 0).x; ctx.moveTo(sx, 0); ctx.lineTo(sx, p.h); }
    for (let bz = startZ; bz <= br.z; bz += blockStep) { const sy = w2s(p, 0, bz).y; ctx.moveTo(0, sy); ctx.lineTo(p.w, sy); }
    ctx.stroke();
    if (blockStep < regionSize) {
      const rsX = Math.floor(tl.x / regionSize) * regionSize, rsZ = Math.floor(tl.z / regionSize) * regionSize;
      ctx.strokeStyle = 'rgba(255, 160, 50, 0.25)'; ctx.lineWidth = 1.5; ctx.beginPath();
      for (let bx = rsX; bx <= br.x; bx += regionSize) { const sx = w2s(p, bx, 0).x; ctx.moveTo(sx, 0); ctx.lineTo(sx, p.h); }
      for (let bz = rsZ; bz <= br.z; bz += regionSize) { const sy = w2s(p, 0, bz).y; ctx.moveTo(0, sy); ctx.lineTo(p.w, sy); }
      ctx.stroke();
    }
  }

  function drawCrosshair(ctx: CanvasRenderingContext2D, p: MapPanel, w: number, h: number) {
    const o = w2s(p, 0, 0);
    ctx.strokeStyle = 'rgba(255,60,60,0.5)'; ctx.lineWidth = 1; ctx.setLineDash([4, 4]); ctx.beginPath();
    ctx.moveTo(o.x, 0); ctx.lineTo(o.x, h); ctx.moveTo(0, o.y); ctx.lineTo(w, o.y); ctx.stroke(); ctx.setLineDash([]);
    ctx.fillStyle = 'rgba(255,60,60,0.7)'; ctx.font = '600 10px "Chakra Petch", sans-serif'; ctx.fillText('0, 0', o.x + 4, o.y - 4);
  }

  function drawMarker(ctx: CanvasRenderingContext2D, p: MapPanel, m: MarkerDef) {
    const s = w2s(p, m.x, m.z);
    // Shadow
    ctx.beginPath(); ctx.ellipse(s.x, s.y + 14, 5, 2, 0, 0, Math.PI * 2); ctx.fillStyle = 'rgba(0,0,0,0.3)'; ctx.fill();
    // Pin
    ctx.beginPath(); ctx.moveTo(s.x, s.y + 12);
    ctx.bezierCurveTo(s.x - 8, s.y - 2, s.x - 8, s.y - 14, s.x, s.y - 16);
    ctx.bezierCurveTo(s.x + 8, s.y - 14, s.x + 8, s.y - 2, s.x, s.y + 12);
    ctx.fillStyle = m.color; ctx.fill(); ctx.strokeStyle = 'rgba(0,0,0,0.3)'; ctx.lineWidth = 1; ctx.stroke();
    ctx.beginPath(); ctx.arc(s.x, s.y - 4, 3, 0, Math.PI * 2); ctx.fillStyle = 'rgba(255,255,255,0.9)'; ctx.fill();
    // Label
    ctx.font = '700 11px "Chakra Petch", sans-serif';
    const tw = ctx.measureText(m.label).width;
    ctx.fillStyle = 'rgba(0,0,0,0.6)'; ctx.beginPath(); ctx.roundRect(s.x - tw / 2 - 4, s.y - 32, tw + 8, 14, 3); ctx.fill();
    ctx.fillStyle = '#fff'; ctx.fillText(m.label, s.x - tw / 2, s.y - 22);
    // Coords
    ctx.font = '600 9px "JetBrains Mono", monospace';
    const ct = `${m.x}, ${m.z}`, ctw = ctx.measureText(ct).width;
    ctx.fillStyle = 'rgba(0,0,0,0.5)'; ctx.beginPath(); ctx.roundRect(s.x - ctw / 2 - 3, s.y + 16, ctw + 6, 13, 3); ctx.fill();
    ctx.fillStyle = 'rgba(255,255,255,0.85)'; ctx.fillText(ct, s.x - ctw / 2, s.y + 26);
  }

  function drawHoverInfo(ctx: CanvasRenderingContext2D, p: MapPanel, w: number, h: number) {
    const chX = Math.floor(p.hoverX / 16), chZ = Math.floor(p.hoverZ / 16);
    const text = `X: ${p.hoverX}  Z: ${p.hoverZ}  |  Chunk: ${chX}, ${chZ}`;
    ctx.font = '600 10px "JetBrains Mono", monospace';
    const tw = ctx.measureText(text).width;
    ctx.fillStyle = 'rgba(0,0,0,0.65)'; ctx.beginPath(); ctx.roundRect(8, h - 26, tw + 12, 20, 4); ctx.fill();
    ctx.fillStyle = 'rgba(255,255,255,0.85)'; ctx.fillText(text, 14, h - 12);
  }

  function drawChunkHighlight(ctx: CanvasRenderingContext2D, p: MapPanel, info: NonNullable<typeof chunkInfo>) {
    const tl = w2s(p, info.chunk.x * 16, info.chunk.z * 16);
    const br = w2s(p, info.chunk.x * 16 + 16, info.chunk.z * 16 + 16);
    ctx.fillStyle = 'rgba(94,144,255,0.15)'; ctx.fillRect(tl.x, tl.y, br.x - tl.x, br.y - tl.y);
    ctx.strokeStyle = 'rgba(94,144,255,0.5)'; ctx.lineWidth = 1.5; ctx.strokeRect(tl.x, tl.y, br.x - tl.x, br.y - tl.y);
    // Region
    const rtl = w2s(p, info.region.x * 512, info.region.z * 512);
    const rbr = w2s(p, info.region.x * 512 + 512, info.region.z * 512 + 512);
    ctx.strokeStyle = 'rgba(255,160,50,0.3)'; ctx.lineWidth = 2; ctx.setLineDash([6, 4]);
    ctx.strokeRect(rtl.x, rtl.y, rbr.x - rtl.x, rbr.y - rtl.y); ctx.setLineDash([]);
  }

  function drawSpawnChunks(ctx: CanvasRenderingContext2D, p: MapPanel) {
    if (!spawnChunks) return;
    const ltl = w2s(p, spawnChunks.lazyFrom.x, spawnChunks.lazyFrom.z);
    const lbr = w2s(p, spawnChunks.lazyTo.x + 1, spawnChunks.lazyTo.z + 1);
    ctx.fillStyle = 'rgba(255,200,50,0.06)'; ctx.fillRect(ltl.x, ltl.y, lbr.x - ltl.x, lbr.y - ltl.y);
    ctx.strokeStyle = 'rgba(255,200,50,0.3)'; ctx.lineWidth = 1; ctx.setLineDash([4, 3]);
    ctx.strokeRect(ltl.x, ltl.y, lbr.x - ltl.x, lbr.y - ltl.y); ctx.setLineDash([]);
    const atl = w2s(p, spawnChunks.from.x, spawnChunks.from.z);
    const abr = w2s(p, spawnChunks.to.x + 1, spawnChunks.to.z + 1);
    ctx.fillStyle = 'rgba(80,220,120,0.1)'; ctx.fillRect(atl.x, atl.y, abr.x - atl.x, abr.y - atl.y);
    ctx.strokeStyle = 'rgba(80,220,120,0.5)'; ctx.lineWidth = 1.5; ctx.strokeRect(atl.x, atl.y, abr.x - atl.x, abr.y - atl.y);
    ctx.font = '600 10px "Chakra Petch", sans-serif';
    ctx.fillStyle = 'rgba(80,220,120,0.8)'; ctx.fillText('Active 19×19', atl.x + 4, atl.y + 14);
    ctx.fillStyle = 'rgba(255,200,50,0.7)'; ctx.fillText('Lazy 23×23', ltl.x + 4, ltl.y + 14);
  }

  function drawDistanceLine(ctx: CanvasRenderingContext2D, p: MapPanel) {
    if (!distance) return;
    const p1 = w2s(p, distance.p1.x, distance.p1.z);
    const p2 = w2s(p, distance.p2.x, distance.p2.z);
    ctx.strokeStyle = 'rgba(255,255,255,0.4)'; ctx.lineWidth = 2; ctx.setLineDash([6, 4]);
    ctx.beginPath(); ctx.moveTo(p1.x, p1.y); ctx.lineTo(p2.x, p2.y); ctx.stroke(); ctx.setLineDash([]);
    const mx = (p1.x + p2.x) / 2, my = (p1.y + p2.y) / 2;
    const label = `${distance.dist2d} blocs`;
    ctx.font = '700 11px "JetBrains Mono", monospace';
    const tw = ctx.measureText(label).width;
    ctx.fillStyle = 'rgba(0,0,0,0.6)'; ctx.beginPath(); ctx.roundRect(mx - tw / 2 - 5, my - 8, tw + 10, 16, 4); ctx.fill();
    ctx.fillStyle = '#fff'; ctx.fillText(label, mx - tw / 2, my + 4);
  }

  // ── Derived markers (NOT in render loop to avoid retriggering $effect) ──
  let markersFrom: MarkerDef[] = $derived(
    converted ? [{ x: converted.srcX, z: converted.srcZ, color: DIM_COLORS[fromDimension].accent, label: 'Origin' }] : []
  );
  let markersTo: MarkerDef[] = $derived(
    converted && (fromDimension as string) !== (toDimension as string) && (fromDimension as string) !== 'end' && (toDimension as string) !== 'end'
      ? [{ x: converted.x, z: converted.z, color: DIM_COLORS[toDimension].accent, label: 'Destination' }] : []
  );
  let markersSingle: MarkerDef[] = $derived.by(() => {
    if (activeTab === 'chunk' && chunkInfo) return [{ x: chunkInfo.block.x, z: chunkInfo.block.z, color: '#5e90ff', label: 'Block' }];
    if (activeTab === 'distance' && distance) return [
      { x: distance.p1.x, z: distance.p1.z, color: '#5ecc77', label: 'A' },
      { x: distance.p2.x, z: distance.p2.z, color: '#ee6655', label: 'B' },
    ];
    if (activeTab === 'spawn' && spawnChunks) return [{ x: spawnChunks.spawnBlock.x, z: spawnChunks.spawnBlock.z, color: '#ffcc33', label: 'Spawn' }];
    return [];
  });

  // ── Main render loop ──
  function renderLoop() {
    if (activeTab === 'converter') {
      panelFrom.dim = fromDimension;
      panelTo.dim = toDimension;
      panelFrom.markers = markersFrom;
      panelTo.markers = markersTo;
      renderPanel(panelFrom);
      renderPanel(panelTo);
    } else {
      const p = panelSingle;
      p.dim = 'overworld';
      p.markers = markersSingle;
      renderPanel(p);
      const ctx = p.el?.getContext('2d');
      if (ctx) {
        if (activeTab === 'chunk' && chunkInfo) {
          drawChunkHighlight(ctx, p, chunkInfo);
          for (const m of p.markers) drawMarker(ctx, p, m);
        } else if (activeTab === 'distance' && distance) {
          drawDistanceLine(ctx, p);
          for (const m of p.markers) drawMarker(ctx, p, m);
        } else if (activeTab === 'spawn' && spawnChunks) {
          drawSpawnChunks(ctx, p);
          for (const m of p.markers) drawMarker(ctx, p, m);
        }
      }
    }

    // Sync store cameras with panels and request tiles
    const now = Date.now();
    const shouldRequest = now - lastTileRequest > 500;
    if (shouldRequest) lastTileRequest = now;

    const panels = activeTab === 'converter'
      ? [{ p: panelFrom, s: storeFrom }, { p: panelTo, s: storeTo }]
      : [{ p: panelSingle, s: storeFrom }];

    for (const { p, s } of panels) {
      if (!s || p.w === 0) continue;
      s.state.centerX = p.camX;
      s.state.centerZ = p.camZ;
      s.state.zoom = p.camZoom;
      s.state.canvasWidth = p.w;
      s.state.canvasHeight = p.h;
      if (shouldRequest) s.requestVisibleTiles();
    }

    requestAnimationFrame(renderLoop);
  }

  // Auto-center when data changes (uses $derived markers, not panel.markers)
  $effect(() => {
    if (activeTab === 'converter' && converted) {
      if (panelFrom.w > 0) centerPanel(panelFrom, markersFrom);
      if (panelTo.w > 0) centerPanel(panelTo, markersTo);
    } else if (activeTab === 'chunk' && chunkInfo) {
      centerPanel(panelSingle, markersSingle);
    } else if (activeTab === 'distance' && distance) {
      centerPanel(panelSingle, markersSingle);
    } else if (activeTab === 'spawn' && spawnChunks) {
      centerPanel(panelSingle, markersSingle, {
        minX: spawnChunks.lazyFrom.x, maxX: spawnChunks.lazyTo.x,
        minZ: spawnChunks.lazyFrom.z, maxZ: spawnChunks.lazyTo.z,
      });
    }
  });

  // Start render loop
  $effect(() => {
    if (panelFrom.el || panelTo.el || panelSingle.el) {
      requestAnimationFrame(renderLoop);
    }
  });

  // Sync store dimensions
  $effect(() => {
    if (storeFrom && storeFrom.state.dimension !== fromDimension) storeFrom.setDimension(fromDimension);
    if (storeTo && (storeTo.state.dimension as string) !== (toDimension as string)) storeTo.setDimension(toDimension);
  });

  // ── Persistence ──

  function persistCalcState() {
    if (!browser) return;
    const params = new URLSearchParams();
    if (seedInput) params.set('seed', seedInput);
    params.set('tab', activeTab);
    params.set('from', fromDimension);
    params.set('to', toDimension);
    if (fromX) params.set('fx', fromX);
    if (fromY) params.set('fy', fromY);
    if (fromZ) params.set('fz', fromZ);
    if (chunkX) params.set('cx', chunkX);
    if (chunkZ) params.set('cz', chunkZ);
    if (distX1) params.set('ax', distX1);
    if (distY1) params.set('ay', distY1);
    if (distZ1) params.set('az', distZ1);
    if (distX2) params.set('bx', distX2);
    if (distY2) params.set('by', distY2);
    if (distZ2) params.set('bz', distZ2);
    if (spawnX) params.set('sx', spawnX);
    if (spawnZ) params.set('sz', spawnZ);
    const hash = '#' + params.toString();
    if (window.location.hash !== hash) history.replaceState(null, '', hash);
    try {
      localStorage.setItem(LS_KEY, JSON.stringify({
        seed: seedInput, tab: activeTab, from: fromDimension, to: toDimension,
        fx: fromX, fy: fromY, fz: fromZ,
        cx: chunkX, cz: chunkZ,
        ax: distX1, ay: distY1, az: distZ1,
        bx: distX2, by: distY2, bz: distZ2,
        sx: spawnX, sz: spawnZ,
      }));
    } catch { /* quota exceeded */ }
  }

  function restoreCalcState(): boolean {
    if (!browser) return false;
    // URL hash first
    const hash = window.location.hash.slice(1);
    if (hash) {
      const p = new URLSearchParams(hash);
      if (p.has('tab')) {
        seedInput = p.get('seed') || '';
        activeTab = p.get('tab') || 'converter';
        fromDimension = (p.get('from') as Dimension) || 'overworld';
        toDimension = (p.get('to') as Dimension) || 'nether';
        fromX = p.get('fx') || ''; fromY = p.get('fy') || ''; fromZ = p.get('fz') || '';
        chunkX = p.get('cx') || ''; chunkZ = p.get('cz') || '';
        distX1 = p.get('ax') || ''; distY1 = p.get('ay') || ''; distZ1 = p.get('az') || '';
        distX2 = p.get('bx') || ''; distY2 = p.get('by') || ''; distZ2 = p.get('bz') || '';
        spawnX = p.get('sx') || ''; spawnZ = p.get('sz') || '';
        return true;
      }
    }
    // Fallback localStorage
    try {
      const raw = localStorage.getItem(LS_KEY);
      if (raw) {
        const s = JSON.parse(raw);
        seedInput = s.seed || '';
        activeTab = s.tab || 'converter';
        fromDimension = s.from || 'overworld';
        toDimension = s.to || 'nether';
        fromX = s.fx || ''; fromY = s.fy || ''; fromZ = s.fz || '';
        chunkX = s.cx || ''; chunkZ = s.cz || '';
        distX1 = s.ax || ''; distY1 = s.ay || ''; distZ1 = s.az || '';
        distX2 = s.bx || ''; distY2 = s.by || ''; distZ2 = s.bz || '';
        spawnX = s.sx || ''; spawnZ = s.sz || '';
        return true;
      }
    } catch { /* corrupt data */ }
    return false;
  }

  // Debounced persist
  let persistTimeout: ReturnType<typeof setTimeout> | null = null;
  $effect(() => {
    // Touch all fields to track
    void activeTab; void seedInput; void fromDimension; void toDimension;
    void fromX; void fromY; void fromZ;
    void chunkX; void chunkZ;
    void distX1; void distY1; void distZ1; void distX2; void distY2; void distZ2;
    void spawnX; void spawnZ;
    if (persistTimeout) clearTimeout(persistTimeout);
    persistTimeout = setTimeout(persistCalcState, 300);
  });

  // ── Init on mount ──

  onMount(() => {
    storeFrom = createSeedMapStore();
    storeTo = createSeedMapStore();
    storeFrom.initWorkers();
    storeTo.initWorkers();

    const restored = restoreCalcState();

    const seed = seedInput || randomSeed();
    seedInput = seed;

    // Init both stores with dimension + seed
    storeFrom.setDimension(fromDimension);
    storeTo.setDimension(toDimension);
    storeFrom.setSeed(seed);
    storeTo.setSeed(seed);

    return () => {
      if (persistTimeout) clearTimeout(persistTimeout);
      storeFrom?.terminateWorkers();
      storeTo?.terminateWorkers();
    };
  });
</script>

<div class="calc-root">
  <div class="seed-row">
    <Input label="Seed (optionnel)" placeholder="Entrez une seed pour afficher les biomes" bind:value={seedInput} oninput={handleSeedChange} />
  </div>

  <Tabs items={tabs} bind:selected={activeTab} />

  <div class="calc-body">
    {#if activeTab === 'converter'}
      <!-- Converter: controls + 2 maps -->
      <Card variant="elevated" padding="lg">
        <div class="dim-row">
          <div class="dim-select">
            <span class="dim-label">De</span>
            <div class="dim-pills">
              {#each dimensions as dim}
                <button class="dim-pill" class:active={fromDimension === dim.value} onclick={() => { fromDimension = dim.value; }}>{dim.label}</button>
              {/each}
            </div>
          </div>
          <button class="swap-btn" onclick={swapDimensions} title="Inverser">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M7 16l-4-4 4-4" /><path d="M17 8l4 4-4 4" /><path d="M3 12h18" />
            </svg>
          </button>
          <div class="dim-select">
            <span class="dim-label">Vers</span>
            <div class="dim-pills">
              {#each dimensions as dim}
                <button class="dim-pill" class:active={toDimension === dim.value} onclick={() => { toDimension = dim.value; }}>{dim.label}</button>
              {/each}
            </div>
          </div>
        </div>

        <div class="coord-grid">
          <Input label="X" placeholder="0" bind:value={fromX} type="number" />
          <Input label="Y" placeholder="64" bind:value={fromY} type="number" />
          <Input label="Z" placeholder="0" bind:value={fromZ} type="number" />
        </div>

        {#if converted}
          <div class="result-card">
            <span class="result-label">Résultat</span>
            <div class="result-coords">
              <div class="result-coord"><span class="coord-axis">X</span><span class="coord-val">{converted.x}</span></div>
              <div class="result-coord"><span class="coord-axis">Y</span><span class="coord-val">{converted.y}</span></div>
              <div class="result-coord"><span class="coord-axis">Z</span><span class="coord-val">{converted.z}</span></div>
            </div>
            {#if fromDimension === toDimension}
              <span class="result-note">Même dimension — coordonnées identiques.</span>
            {:else if fromDimension === 'end' || toDimension === 'end'}
              <span class="result-note">L'End n'a pas de conversion avec les autres dimensions.</span>
            {:else}
              <span class="result-note">Ratio {fromDimension === 'overworld' ? '÷' : '×'}{NETHER_RATIO} sur X et Z</span>
            {/if}
          </div>
        {/if}
      </Card>

      <div class="dual-maps">
        <div class="map-panel">
          <canvas
            bind:this={panelFrom.el}
            class="map-canvas"
            onpointerdown={(e) => onPointerDown(panelFrom, e)}
            onpointermove={(e) => onPointerMove(panelFrom, e)}
            onpointerup={() => onPointerUp(panelFrom)}
            onpointerleave={() => onPointerLeave(panelFrom)}
            onwheel={(e) => onWheel(panelFrom, e)}
          ></canvas>
        </div>
        <div class="map-panel">
          <canvas
            bind:this={panelTo.el}
            class="map-canvas"
            onpointerdown={(e) => onPointerDown(panelTo, e)}
            onpointermove={(e) => onPointerMove(panelTo, e)}
            onpointerup={() => onPointerUp(panelTo)}
            onpointerleave={() => onPointerLeave(panelTo)}
            onwheel={(e) => onWheel(panelTo, e)}
          ></canvas>
        </div>
      </div>

    {:else if activeTab === 'chunk'}
      <div class="single-layout">
        <Card variant="elevated" padding="lg">
          <span class="section-label">Trouver le chunk et la région</span>
          <div class="coord-grid coord-grid--2col">
            <Input label="Block X" placeholder="0" bind:value={chunkX} type="number" />
            <Input label="Block Z" placeholder="0" bind:value={chunkZ} type="number" />
          </div>
          {#if chunkInfo}
            <div class="result-card">
              <div class="info-grid">
                <div class="info-item"><span class="info-key">Chunk</span><span class="info-val">{chunkInfo.chunk.x}, {chunkInfo.chunk.z}</span></div>
                <div class="info-item"><span class="info-key">Région</span><span class="info-val">{chunkInfo.region.x}, {chunkInfo.region.z}</span></div>
                <div class="info-item"><span class="info-key">Fichier</span><span class="info-val mono">{chunkInfo.regionFile}</span></div>
                <div class="info-item"><span class="info-key">Dans le chunk</span><span class="info-val">{chunkInfo.localBlock.x}, {chunkInfo.localBlock.z}</span></div>
              </div>
            </div>
          {/if}
        </Card>
        <div class="map-panel map-panel--full">
          <canvas
            bind:this={panelSingle.el}
            class="map-canvas"
            onpointerdown={(e) => onPointerDown(panelSingle, e)}
            onpointermove={(e) => onPointerMove(panelSingle, e)}
            onpointerup={() => onPointerUp(panelSingle)}
            onpointerleave={() => onPointerLeave(panelSingle)}
            onwheel={(e) => onWheel(panelSingle, e)}
          ></canvas>
        </div>
      </div>

    {:else if activeTab === 'distance'}
      <div class="single-layout">
        <Card variant="elevated" padding="lg">
          <span class="section-label">Calculer la distance</span>
          <div class="point-row"><span class="point-label">Point A</span>
            <div class="coord-grid">
              <Input label="X" placeholder="0" bind:value={distX1} type="number" />
              <Input label="Y" placeholder="64" bind:value={distY1} type="number" />
              <Input label="Z" placeholder="0" bind:value={distZ1} type="number" />
            </div>
          </div>
          <div class="point-row"><span class="point-label">Point B</span>
            <div class="coord-grid">
              <Input label="X" placeholder="0" bind:value={distX2} type="number" />
              <Input label="Y" placeholder="64" bind:value={distY2} type="number" />
              <Input label="Z" placeholder="0" bind:value={distZ2} type="number" />
            </div>
          </div>
          {#if distance}
            <div class="result-card">
              <div class="info-grid">
                <div class="info-item"><span class="info-key">Distance 2D</span><span class="info-val">{distance.dist2d} blocs</span></div>
                <div class="info-item"><span class="info-key">Distance 3D</span><span class="info-val">{distance.dist3d} blocs</span></div>
                <div class="info-item"><span class="info-key">Différence</span><span class="info-val">ΔX={distance.dx}  ΔY={distance.dy}  ΔZ={distance.dz}</span></div>
              </div>
            </div>
          {/if}
        </Card>
        <div class="map-panel map-panel--full">
          <canvas
            bind:this={panelSingle.el}
            class="map-canvas"
            onpointerdown={(e) => onPointerDown(panelSingle, e)}
            onpointermove={(e) => onPointerMove(panelSingle, e)}
            onpointerup={() => onPointerUp(panelSingle)}
            onpointerleave={() => onPointerLeave(panelSingle)}
            onwheel={(e) => onWheel(panelSingle, e)}
          ></canvas>
        </div>
      </div>

    {:else if activeTab === 'spawn'}
      <div class="single-layout">
        <Card variant="elevated" padding="lg">
          <span class="section-label">Spawn Chunks</span>
          <p class="section-hint">Entrez les coordonnées du spawn pour visualiser la zone toujours chargée.</p>
          <div class="coord-grid coord-grid--2col">
            <Input label="Spawn X" placeholder="0" bind:value={spawnX} type="number" />
            <Input label="Spawn Z" placeholder="0" bind:value={spawnZ} type="number" />
          </div>
          {#if spawnChunks}
            <div class="result-card">
              <div class="info-grid">
                <div class="info-item"><span class="info-key">Chunk central</span><span class="info-val">{spawnChunks.center.x}, {spawnChunks.center.z}</span></div>
                <div class="info-item"><span class="info-key">Active (19×19)</span><span class="info-val">({spawnChunks.from.x}, {spawnChunks.from.z}) → ({spawnChunks.to.x}, {spawnChunks.to.z})</span></div>
                <div class="info-item"><span class="info-key">Lazy (23×23)</span><span class="info-val">({spawnChunks.lazyFrom.x}, {spawnChunks.lazyFrom.z}) → ({spawnChunks.lazyTo.x}, {spawnChunks.lazyTo.z})</span></div>
              </div>
              <span class="result-note">Les spawn chunks restent chargés en permanence. La zone lazy a un tick rate réduit.</span>
            </div>
          {/if}
        </Card>
        <div class="map-panel map-panel--full">
          <canvas
            bind:this={panelSingle.el}
            class="map-canvas"
            onpointerdown={(e) => onPointerDown(panelSingle, e)}
            onpointermove={(e) => onPointerMove(panelSingle, e)}
            onpointerup={() => onPointerUp(panelSingle)}
            onpointerleave={() => onPointerLeave(panelSingle)}
            onwheel={(e) => onWheel(panelSingle, e)}
          ></canvas>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .calc-root { display: flex; flex-direction: column; gap: 0.75rem; }

  .seed-row { max-width: 480px; }

  .calc-body { display: flex; flex-direction: column; gap: 1rem; }

  /* ── Dual maps (converter) ── */
  .dual-maps {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
  }

  .map-panel {
    position: relative;
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: var(--radius-lg, 16px);
    overflow: hidden;
    background: #111;
    min-height: 320px;
  }

  .map-panel--full { min-height: 400px; }

  .map-canvas { width: 100%; height: 100%; display: block; cursor: grab; }
  .map-canvas:active { cursor: grabbing; }

  /* ── Single layout (controls + map side by side) ── */
  .single-layout {
    display: grid;
    grid-template-columns: 420px 1fr;
    gap: 1rem;
    min-height: 420px;
  }

  /* ── Sections ── */
  .section-label { font-size: 0.65rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.08em; color: var(--ink-2, #5a7894); margin-bottom: 0.3rem; }
  .section-hint { font-size: 0.82rem; color: var(--ink-2, #5a7894); margin: 0 0 0.5rem; line-height: 1.4; }

  /* ── Dimension pills ── */
  .dim-row { display: flex; align-items: center; gap: 0.5rem; margin: 0.4rem 0 0.6rem; }
  .dim-select { display: flex; flex-direction: column; gap: 0.25rem; flex: 1; min-width: 0; }
  .dim-label { font-size: 0.6rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--ink-2, #5a7894); }
  .dim-pills { display: flex; border: 1px solid rgba(70, 113, 166, 0.35); border-radius: 6px; overflow: hidden; }
  .dim-pill { flex: 1; padding: 0.35rem 0.4rem; font-size: 0.72rem; font-family: inherit; font-weight: 600; border: none; background: rgba(255, 255, 255, 0.5); color: var(--ink-1, #2d4a65); cursor: pointer; transition: background 120ms ease, color 120ms ease; }
  .dim-pill:not(:last-child) { border-right: 1px solid rgba(70, 113, 166, 0.25); }
  .dim-pill:hover { background: rgba(94, 144, 255, 0.08); }
  .dim-pill.active { background: var(--blue-0, #5e90ff); color: #fff; }
  .swap-btn { display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; border: 1px solid rgba(70, 113, 166, 0.35); border-radius: 6px; background: rgba(255, 255, 255, 0.5); color: var(--ink-2, #5a7894); cursor: pointer; transition: background 120ms ease, color 120ms ease; flex-shrink: 0; margin-top: 0.8rem; }
  .swap-btn:hover { background: rgba(94, 144, 255, 0.1); color: var(--blue-0, #5e90ff); }

  /* ── Coord inputs ── */
  .coord-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 0.5rem; }
  .coord-grid--2col { grid-template-columns: repeat(2, 1fr); }

  /* ── Results ── */
  .result-card { margin-top: 0.8rem; padding: 0.7rem 0.8rem; background: rgba(94, 144, 255, 0.06); border: 1px solid rgba(94, 144, 255, 0.2); border-radius: 8px; }
  .result-label { font-size: 0.6rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--blue-0, #5e90ff); }
  .result-coords { display: flex; gap: 1.2rem; margin-top: 0.4rem; }
  .result-coord { display: flex; flex-direction: column; align-items: center; gap: 0.1rem; }
  .coord-axis { font-size: 0.6rem; font-weight: 700; text-transform: uppercase; color: var(--ink-2, #5a7894); }
  .coord-val { font-family: 'JetBrains Mono', monospace; font-size: 1.2rem; font-weight: 700; color: var(--ink-0, #0f253a); }
  .result-note { display: block; font-size: 0.72rem; color: var(--ink-2, #5a7894); margin-top: 0.4rem; }

  /* ── Info grid ── */
  .info-grid { display: flex; flex-direction: column; gap: 0.4rem; }
  .info-item { display: flex; justify-content: space-between; align-items: center; gap: 0.5rem; }
  .info-key { font-size: 0.75rem; font-weight: 600; color: var(--ink-1, #2d4a65); }
  .info-val { font-size: 0.8rem; font-weight: 700; color: var(--ink-0, #0f253a); }
  .info-val.mono { font-family: 'JetBrains Mono', monospace; font-size: 0.78rem; }

  /* ── Point labels ── */
  .point-row { margin-bottom: 0.5rem; }
  .point-label { display: block; font-family: 'Teko', sans-serif; font-size: 1rem; font-weight: 600; color: var(--ink-0, #0f253a); margin-bottom: 0.2rem; }

  @media (max-width: 768px) {
    .dual-maps { grid-template-columns: 1fr; }
    .single-layout { grid-template-columns: 1fr; }
    .map-panel { min-height: 250px; }
    .dim-row { flex-direction: column; align-items: stretch; }
    .swap-btn { align-self: center; margin-top: 0; transform: rotate(90deg); }
    .coord-grid { grid-template-columns: 1fr; }
    .coord-grid--2col { grid-template-columns: 1fr; }
  }
</style>
