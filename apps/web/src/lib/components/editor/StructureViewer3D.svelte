<script lang="ts">
  import { onMount } from 'svelte';
  import * as THREE from 'three';
  import { getBlockColor, isAirBlock } from '$lib/utils/block-colors';
  import { getTextureFileName } from '$lib/utils/block-textures';
  import type { StructureData } from '$lib/utils/nbt-parser';

  let {
    structure,
    layerY = -1,
    brightness = 70,
  }: {
    structure: StructureData;
    layerY?: number;
    brightness?: number;
  } = $props();

  let container: HTMLDivElement;
  let animId: number;
  let root: THREE.Group;
  let currentStructure: StructureData | null = null;
  let currentLayerY = -1;

  // Material cache (texture or color fallback)
  const materialCache = new Map<string, THREE.MeshLambertMaterial>();
  const loader = new THREE.TextureLoader();
  const failedTextures = new Set<string>();

  function getBlockMaterial(blockName: string): THREE.MeshLambertMaterial {
    if (materialCache.has(blockName)) return materialCache.get(blockName)!;

    const texFileName = getTextureFileName(blockName);
    const color = getBlockColor(blockName);

    // If we already know this texture doesn't exist, use color directly
    if (failedTextures.has(texFileName)) {
      const mat = new THREE.MeshLambertMaterial({ color: color || '#ff00ff' });
      materialCache.set(blockName, mat);
      return mat;
    }

    // Try loading texture
    const tex = loader.load(
      `/images/blocks/${texFileName}.png`,
      () => { tex.needsUpdate = true; },
      undefined,
      () => {
        // Texture not found — swap to color material
        failedTextures.add(texFileName);
        const fallback = new THREE.MeshLambertMaterial({ color: color || '#ff00ff' });
        materialCache.set(blockName, fallback);
      }
    );
    tex.magFilter = THREE.NearestFilter;
    tex.minFilter = THREE.NearestFilter;
    tex.colorSpace = THREE.SRGBColorSpace;

    const mat = new THREE.MeshLambertMaterial({ map: tex });
    materialCache.set(blockName, mat);
    return mat;
  }

  function buildBlocks() {
    if (!root || !structure) return;

    while (root.children.length > 0) {
      const child = root.children[0];
      root.remove(child);
      if ((child as any).geometry) (child as any).geometry.dispose();
      if ((child as any).material) {
        const mat = (child as any).material;
        if (Array.isArray(mat)) mat.forEach((m: any) => m.dispose());
        else mat.dispose();
      }
    }

    const { palette, blocks, size } = structure;

    // Group blocks by block name for batching
    const blockGroups = new Map<string, THREE.Matrix4[]>();

    for (const block of blocks) {
      const blockName = palette[block.state]?.Name ?? 'minecraft:air';
      if (isAirBlock(blockName)) continue;
      if (layerY >= 0 && block.pos[1] !== layerY) continue;

      let group = blockGroups.get(blockName);
      if (!group) { group = []; blockGroups.set(blockName, group); }

      const matrix = new THREE.Matrix4();
      matrix.setPosition(
        block.pos[0] - size[0] / 2,
        block.pos[1] - size[1] / 2,
        block.pos[2] - size[2] / 2
      );
      group.push(matrix);
    }

    const geo = new THREE.BoxGeometry(0.98, 0.98, 0.98);

    for (const [blockName, matrices] of blockGroups) {
      const mat = getBlockMaterial(blockName);
      const mesh = new THREE.InstancedMesh(geo, mat, matrices.length);
      for (let i = 0; i < matrices.length; i++) {
        mesh.setMatrixAt(i, matrices[i]);
      }
      mesh.instanceMatrix.needsUpdate = true;
      root.add(mesh);
    }

    currentStructure = structure;
    currentLayerY = layerY;
  }

  onMount(() => {
    const initW = container.clientWidth || 600;
    const initH = container.clientHeight || 500;

    const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setSize(initW, initH);
    renderer.setPixelRatio(window.devicePixelRatio);
    container.appendChild(renderer.domElement);

    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(50, initW / initH, 0.1, 2000);

    const maxDim = Math.max(structure.size[0], structure.size[1], structure.size[2]);
    camera.position.set(maxDim * 1.2, maxDim * 0.8, maxDim * 1.2);
    camera.lookAt(0, 0, 0);

    const ambLight = new THREE.AmbientLight(0xffffff, brightness / 100);
    scene.add(ambLight);
    const dirLight = new THREE.DirectionalLight(0xffffff, brightness / 150);
    dirLight.position.set(1, 1.5, 1);
    scene.add(dirLight);

    const gridSize = Math.max(structure.size[0], structure.size[2]);
    const grid = new THREE.GridHelper(gridSize, gridSize, 0x888888, 0x444444);
    grid.position.y = -structure.size[1] / 2;
    scene.add(grid);

    const ro = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const { width: rw, height: rh } = entry.contentRect;
        if (rw > 0 && rh > 0) { renderer.setSize(rw, rh); camera.aspect = rw / rh; camera.updateProjectionMatrix(); }
      }
    });
    ro.observe(container);

    root = new THREE.Group();
    scene.add(root);
    buildBlocks();

    // Orbital camera
    let theta = Math.PI / 4;
    let phi = Math.PI / 6;
    let distance = maxDim * 1.8;
    let rotating = false;
    let lastPX = 0, lastPY = 0;
    const cvs = renderer.domElement;

    function applyCamera() {
      camera.position.x = distance * Math.sin(theta) * Math.cos(phi);
      camera.position.y = distance * Math.sin(phi);
      camera.position.z = distance * Math.cos(theta) * Math.cos(phi);
      camera.lookAt(0, 0, 0);
    }
    applyCamera();

    cvs.addEventListener('pointerdown', (e) => {
      rotating = true; lastPX = e.clientX; lastPY = e.clientY;
      cvs.setPointerCapture(e.pointerId);
    });
    cvs.addEventListener('pointermove', (e) => {
      if (!rotating) return;
      theta -= (e.clientX - lastPX) * 0.01;
      phi = THREE.MathUtils.clamp(phi + (e.clientY - lastPY) * 0.01, -Math.PI / 2.5, Math.PI / 2.5);
      lastPX = e.clientX; lastPY = e.clientY;
      applyCamera();
    });
    const onUp = () => { rotating = false; };
    cvs.addEventListener('pointerup', onUp);
    window.addEventListener('pointerup', onUp);

    cvs.addEventListener('wheel', (e) => {
      e.preventDefault();
      distance = THREE.MathUtils.clamp(distance + e.deltaY * 0.05, maxDim * 0.5, maxDim * 5);
      applyCamera();
    }, { passive: false });

    cvs.addEventListener('contextmenu', (e) => e.preventDefault());

    function animate() {
      animId = requestAnimationFrame(animate);
      if (structure !== currentStructure || layerY !== currentLayerY) buildBlocks();
      ambLight.intensity = brightness / 100;
      dirLight.intensity = brightness / 150;
      renderer.render(scene, camera);
    }
    animate();

    return () => { cancelAnimationFrame(animId); window.removeEventListener('pointerup', onUp); ro.disconnect(); renderer.dispose(); };
  });
</script>

<div bind:this={container} class="structure-viewer" role="img" aria-label="Vue 3D de la structure NBT"></div>

<style>
  .structure-viewer {
    display: block; width: 100%; height: 100%; min-height: 400px;
    border-radius: var(--radius-md, 12px); overflow: hidden; cursor: grab;
    background: linear-gradient(135deg, #1a1a2e 0%, #0a0a1a 100%);
  }
  .structure-viewer:active { cursor: grabbing; }
</style>
