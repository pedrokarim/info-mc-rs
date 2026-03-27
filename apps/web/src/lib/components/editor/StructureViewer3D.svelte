<script lang="ts">
  import { onMount } from 'svelte';
  import * as THREE from 'three';
  import { getBlockColor, isAirBlock } from '$lib/utils/block-colors';
  import { getTextureFileName } from '$lib/utils/block-textures';
  import type { StructureData } from '$lib/utils/nbt-parser';

  let {
    structure,
    layerY = -1,
    brightness = 100,
  }: {
    structure: StructureData;
    layerY?: number;
    brightness?: number;
  } = $props();

  /**
   * Resolve block name using properties for blocks that have color/type variants.
   * e.g. minecraft:coral_block + {coral_color: "tube"} → minecraft:tube_coral_block
   */
  function resolveBlockName(name: string, props?: Record<string, string>): string {
    if (!props) return name;
    const short = name.replace('minecraft:', '');

    // Legacy variant property (pre-1.13 "The Flattening")
    // e.g. minecraft:stone + variant=granite → minecraft:granite
    if (props.variant) {
      const v = props.variant;
      // Stone variants
      if (short === 'stone' && v !== 'stone') return `minecraft:${v}`;
      // Dirt variants
      if (short === 'dirt' && v !== 'dirt') return `minecraft:${v}`;
      // Sand variants
      if (short === 'sand' && v !== 'sand') return `minecraft:${v}`;
      // Planks / logs
      if (short === 'planks') return `minecraft:${v}_planks`;
      if (short === 'log') return `minecraft:${v}_log`;
      if (short === 'log2') return `minecraft:${v}_log`;
      if (short === 'sapling') return `minecraft:${v}_sapling`;
      if (short === 'leaves') return `minecraft:${v}_leaves`;
      if (short === 'leaves2') return `minecraft:${v}_leaves`;
      // Stonebrick variants
      if (short === 'stonebrick') {
        if (v === 'default' || v === 'stonebrick') return 'minecraft:stone_bricks';
        if (v === 'mossy') return 'minecraft:mossy_stone_bricks';
        if (v === 'cracked') return 'minecraft:cracked_stone_bricks';
        if (v === 'chiseled') return 'minecraft:chiseled_stone_bricks';
      }
      // Prismarine
      if (short === 'prismarine') {
        if (v === 'default' || v === 'prismarine') return 'minecraft:prismarine';
        if (v === 'bricks') return 'minecraft:prismarine_bricks';
        if (v === 'dark') return 'minecraft:dark_prismarine';
      }
    }

    // Coral blocks: coral_block + coral_color → {color}_coral_block
    if (short === 'coral_block' && props.coral_color) {
      return `minecraft:${props.dead === 'true' ? 'dead_' : ''}${props.coral_color}_coral_block`;
    }
    if (short === 'coral_block' && !props.coral_color) {
      // Old format — no color specified, use brain_coral as fallback
      return 'minecraft:brain_coral_block';
    }
    // Coral fans/wall fans
    if ((short === 'coral_fan' || short === 'coral_wall_fan') && props.coral_color) {
      return `minecraft:${props.dead === 'true' ? 'dead_' : ''}${props.coral_color}_${short}`;
    }
    // Colored blocks
    if (props.color) {
      return `minecraft:${props.color}_${short}`;
    }
    // Wood type
    if (props.wood_type) {
      return `minecraft:${props.wood_type}_${short}`;
    }

    return name;
  }

  let container: HTMLDivElement;
  let animId: number;
  let root: THREE.Group;
  let currentStructure: StructureData | null = null;
  let currentLayerY = -1;

  // Material cache (texture or missing texture fallback)
  const materialCache = new Map<string, THREE.MeshLambertMaterial>();
  const loader = new THREE.TextureLoader();
  const failedTextures = new Set<string>();
  let missingTexture: THREE.Texture | null = null;

  /** Generate the classic Minecraft missing texture (black & magenta checkerboard 16×16) */
  function getMissingTexture(): THREE.Texture {
    if (missingTexture) return missingTexture;
    const canvas = document.createElement('canvas');
    canvas.width = 16;
    canvas.height = 16;
    const ctx = canvas.getContext('2d')!;
    const size = 8;
    for (let y = 0; y < 2; y++) {
      for (let x = 0; x < 2; x++) {
        ctx.fillStyle = (x + y) % 2 === 0 ? '#f800f8' : '#000000';
        ctx.fillRect(x * size, y * size, size, size);
      }
    }
    missingTexture = new THREE.CanvasTexture(canvas);
    missingTexture.magFilter = THREE.NearestFilter;
    missingTexture.minFilter = THREE.NearestFilter;
    return missingTexture;
  }

  function getBlockMaterial(blockName: string): THREE.MeshLambertMaterial {
    if (materialCache.has(blockName)) return materialCache.get(blockName)!;

    const texFileName = getTextureFileName(blockName);

    // If we already know this texture doesn't exist, use missing texture
    if (failedTextures.has(texFileName)) {
      const mat = new THREE.MeshLambertMaterial({ map: getMissingTexture() });
      materialCache.set(blockName, mat);
      return mat;
    }

    // Create material first, then load texture into it
    // This way the SAME material object gets updated when texture loads or fails
    const mat = new THREE.MeshLambertMaterial({ map: getMissingTexture() }); // start with missing
    materialCache.set(blockName, mat);

    const tex = loader.load(
      `/images/blocks/${texFileName}.png`,
      () => {
        // Success — swap to real texture
        tex.magFilter = THREE.NearestFilter;
        tex.minFilter = THREE.NearestFilter;
        tex.colorSpace = THREE.SRGBColorSpace;
        mat.map = tex;
        mat.needsUpdate = true;
      },
      undefined,
      () => {
        // Failed — keep missing texture (already set)
        failedTextures.add(texFileName);
      }
    );

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

    // Group blocks by resolved texture key for batching
    const blockGroups = new Map<string, THREE.Matrix4[]>();

    for (const block of blocks) {
      const entry = palette[block.state];
      const blockName = entry?.Name ?? 'minecraft:air';
      if (isAirBlock(blockName)) continue;
      if (layerY >= 0 && block.pos[1] !== layerY) continue;

      // Resolve block name with properties for texture lookup
      const resolvedName = resolveBlockName(blockName, entry?.Properties);

      let group = blockGroups.get(resolvedName);
      if (!group) { group = []; blockGroups.set(resolvedName, group); }

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
    background: linear-gradient(135deg, #6b8cad 0%, #4a6a8a 100%);
  }
  .structure-viewer:active { cursor: grabbing; }
</style>
