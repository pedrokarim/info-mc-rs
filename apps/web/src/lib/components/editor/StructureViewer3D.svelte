<script lang="ts">
  import { onMount } from 'svelte';
  import * as THREE from 'three';
  import { mergeGeometries } from 'three/examples/jsm/utils/BufferGeometryUtils.js';
  import { isAirBlock } from '$lib/utils/block-colors';
  import { getBlockFaceTextures } from '$lib/utils/block-textures';
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
      if (short === 'wooden_slab') return `minecraft:${v}_slab`;
      if (short === 'double_wooden_slab') return `minecraft:${v}_planks`;
      if (short === 'wooden_door') return `minecraft:${v}_door`;
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

  // Shared material / geometry caches
  const textureMaterialCache = new Map<string, THREE.MeshLambertMaterial>();
  const blockMaterialCache = new Map<string, THREE.Material | THREE.Material[]>();
  const geometryCache = new Map<string, THREE.BufferGeometry>();
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
    missingTexture.colorSpace = THREE.SRGBColorSpace;
    return missingTexture;
  }

  function getTextureMaterial(textureName: string): THREE.MeshLambertMaterial {
    if (textureMaterialCache.has(textureName)) return textureMaterialCache.get(textureName)!;

    if (failedTextures.has(textureName)) {
      const mat = new THREE.MeshLambertMaterial({
        map: getMissingTexture(),
        transparent: true,
        alphaTest: 0.1,
      });
      textureMaterialCache.set(textureName, mat);
      return mat;
    }

    const mat = new THREE.MeshLambertMaterial({
      map: getMissingTexture(),
      transparent: true,
      alphaTest: 0.1,
    });
    textureMaterialCache.set(textureName, mat);

    const tex = loader.load(
      `/images/blocks/${textureName}.png`,
      () => {
        tex.magFilter = THREE.NearestFilter;
        tex.minFilter = THREE.NearestFilter;
        tex.colorSpace = THREE.SRGBColorSpace;
        mat.map = tex;
        mat.needsUpdate = true;
      },
      undefined,
      () => {
        failedTextures.add(textureName);
      }
    );

    return mat;
  }

  function getBlockMaterial(
    blockName: string,
    props?: Record<string, string>
  ): THREE.Material | THREE.Material[] {
    const faces = getBlockFaceTextures(blockName, props);
    const signature = [
      faces.right,
      faces.left,
      faces.top,
      faces.bottom,
      faces.front,
      faces.back,
    ].join('|');

    if (blockMaterialCache.has(signature)) return blockMaterialCache.get(signature)!;

    const uniqueFaces = new Set(Object.values(faces));
    if (uniqueFaces.size === 1) {
      const mat = getTextureMaterial(faces.right);
      blockMaterialCache.set(signature, mat);
      return mat;
    }

    const materials = [
      getTextureMaterial(faces.right),
      getTextureMaterial(faces.left),
      getTextureMaterial(faces.top),
      getTextureMaterial(faces.bottom),
      getTextureMaterial(faces.front),
      getTextureMaterial(faces.back),
    ];
    blockMaterialCache.set(signature, materials);
    return materials;
  }

  function createBoxGeometry(
    width: number,
    height: number,
    depth: number,
    tx = 0,
    ty = 0,
    tz = 0
  ): THREE.BufferGeometry {
    const geo = new THREE.BoxGeometry(width, height, depth);
    if (tx !== 0 || ty !== 0 || tz !== 0) {
      geo.translate(tx, ty, tz);
    }
    return geo;
  }

  function getGeometry(geometryKey: string): THREE.BufferGeometry {
    if (geometryCache.has(geometryKey)) return geometryCache.get(geometryKey)!;

    let geo: THREE.BufferGeometry;

    switch (geometryKey) {
      case 'cube':
        geo = createBoxGeometry(0.98, 0.98, 0.98);
        break;
      case 'slab-bottom':
        geo = createBoxGeometry(0.98, 0.49, 0.98, 0, -0.245, 0);
        break;
      case 'slab-top':
        geo = createBoxGeometry(0.98, 0.49, 0.98, 0, 0.245, 0);
        break;
      case 'carpet':
        geo = createBoxGeometry(0.98, 0.08, 0.98, 0, -0.45, 0);
        break;
      case 'fence-post':
        geo = createBoxGeometry(0.28, 0.98, 0.28);
        break;
      case 'wall-post':
        geo = createBoxGeometry(0.5, 0.98, 0.5);
        break;
      case 'pane':
        geo = createBoxGeometry(0.98, 0.98, 0.12);
        break;
      case 'door':
        geo = createBoxGeometry(0.98, 0.98, 0.12);
        break;
      case 'sign':
        geo = createBoxGeometry(0.88, 0.72, 0.08, 0, 0.05, 0);
        break;
      case 'trapdoor-flat-top':
        geo = createBoxGeometry(0.98, 0.12, 0.98, 0, 0.43, 0);
        break;
      case 'trapdoor-flat-bottom':
        geo = createBoxGeometry(0.98, 0.12, 0.98, 0, -0.43, 0);
        break;
      case 'trapdoor-open':
        geo = createBoxGeometry(0.98, 0.98, 0.12);
        break;
      case 'torch':
        geo = createBoxGeometry(0.14, 0.7, 0.14, 0, -0.14, 0);
        break;
      case 'wall-torch':
        geo = createBoxGeometry(0.14, 0.7, 0.14, 0, 0.05, -0.24);
        break;
      case 'stair-bottom': {
        const lower = createBoxGeometry(0.98, 0.49, 0.98, 0, -0.245, 0);
        const upper = createBoxGeometry(0.98, 0.49, 0.49, 0, 0.245, -0.245);
        geo = mergeGeometries([lower, upper], false);
        lower.dispose();
        upper.dispose();
        break;
      }
      case 'stair-top': {
        const upper = createBoxGeometry(0.98, 0.49, 0.98, 0, 0.245, 0);
        const lower = createBoxGeometry(0.98, 0.49, 0.49, 0, -0.245, 0.245);
        geo = mergeGeometries([upper, lower], false);
        upper.dispose();
        lower.dispose();
        break;
      }
      default:
        geo = createBoxGeometry(0.98, 0.98, 0.98);
        break;
    }

    geometryCache.set(geometryKey, geo);
    return geo;
  }

  type RenderSpec = {
    geometryKey: string;
    rotationY: number;
    rotationX: number;
  };

  function facingToRotation(facing?: string): number {
    switch (facing) {
      case 'south': return Math.PI;
      case 'west': return Math.PI / 2;
      case 'east': return -Math.PI / 2;
      default: return 0;
    }
  }

  function getRenderSpec(
    blockName: string,
    props?: Record<string, string>
  ): RenderSpec {
    const short = blockName.replace('minecraft:', '');

    if (short.endsWith('_stairs')) {
      return {
        geometryKey: props?.half === 'top' ? 'stair-top' : 'stair-bottom',
        rotationY: facingToRotation(props?.facing),
        rotationX: 0,
      };
    }

    if (short === 'wooden_slab' || short.endsWith('_slab')) {
      return {
        geometryKey: props?.half === 'top' || props?.type === 'top' ? 'slab-top' : 'slab-bottom',
        rotationY: 0,
        rotationX: 0,
      };
    }

    if (short.endsWith('_carpet') || short === 'carpet') {
      return { geometryKey: 'carpet', rotationY: 0, rotationX: 0 };
    }

    if (short === 'trapdoor' || short.endsWith('_trapdoor')) {
      if (props?.open === 'true') {
        return {
          geometryKey: 'trapdoor-open',
          rotationY: facingToRotation(props?.facing),
          rotationX: 0,
        };
      }
      return {
        geometryKey: props?.half === 'top' ? 'trapdoor-flat-top' : 'trapdoor-flat-bottom',
        rotationY: 0,
        rotationX: 0,
      };
    }

    if (short === 'wooden_door' || short.endsWith('_door')) {
      return {
        geometryKey: 'door',
        rotationY: facingToRotation(props?.facing),
        rotationX: 0,
      };
    }

    if (short === 'fence' || short.endsWith('_fence')) {
      return { geometryKey: 'fence-post', rotationY: 0, rotationX: 0 };
    }

    if (short.endsWith('_wall')) {
      return { geometryKey: 'wall-post', rotationY: 0, rotationX: 0 };
    }

    if (short === 'iron_bars' || short.endsWith('_pane')) {
      return {
        geometryKey: 'pane',
        rotationY: facingToRotation(props?.facing),
        rotationX: 0,
      };
    }

    if (short === 'sign' || short.endsWith('_sign')) {
      return {
        geometryKey: 'sign',
        rotationY: facingToRotation(props?.facing),
        rotationX: 0,
      };
    }

    if (short === 'torch') {
      return { geometryKey: 'torch', rotationY: 0, rotationX: 0 };
    }

    if (short === 'wall_torch' || short.endsWith('_wall_torch')) {
      return {
        geometryKey: 'wall-torch',
        rotationY: facingToRotation(props?.facing),
        rotationX: 0,
      };
    }

    return { geometryKey: 'cube', rotationY: 0, rotationX: 0 };
  }

  function buildBlocks() {
    if (!root || !structure) return;

    while (root.children.length > 0) {
      const child = root.children[0];
      root.remove(child);
    }

    const { palette, blocks, size } = structure;

    const blockGroups = new Map<string, {
      blockName: string;
      props?: Record<string, string>;
      geometryKey: string;
      matrices: THREE.Matrix4[];
    }>();

    for (const block of blocks) {
      const entry = palette[block.state];
      const blockName = entry?.Name ?? 'minecraft:air';
      if (isAirBlock(blockName)) continue;
      if (layerY >= 0 && block.pos[1] !== layerY) continue;

      // Resolve block name with properties for texture lookup
      const resolvedName = resolveBlockName(blockName, entry?.Properties);
      const renderSpec = getRenderSpec(resolvedName, entry?.Properties);
      const groupKey = `${resolvedName}|${JSON.stringify(entry?.Properties ?? {})}|${renderSpec.geometryKey}`;
      let group = blockGroups.get(groupKey);
      if (!group) {
        group = {
          blockName: resolvedName,
          props: entry?.Properties,
          geometryKey: renderSpec.geometryKey,
          matrices: [],
        };
        blockGroups.set(groupKey, group);
      }

      const quaternion = new THREE.Quaternion().setFromEuler(
        new THREE.Euler(renderSpec.rotationX, renderSpec.rotationY, 0)
      );
      const matrix = new THREE.Matrix4();
      matrix.compose(
        new THREE.Vector3(
          block.pos[0] - size[0] / 2,
          block.pos[1] - size[1] / 2,
          block.pos[2] - size[2] / 2
        ),
        quaternion,
        new THREE.Vector3(1, 1, 1)
      );
      group.matrices.push(matrix);
    }

    for (const { blockName, props, geometryKey, matrices } of blockGroups.values()) {
      const geo = getGeometry(geometryKey);
      const mat = getBlockMaterial(blockName, props);
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

    const renderer = new THREE.WebGLRenderer({ antialias: false, alpha: true });
    renderer.setSize(initW, initH);
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    renderer.outputColorSpace = THREE.SRGBColorSpace;
    container.appendChild(renderer.domElement);

    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(50, initW / initH, 0.1, 2000);

    const maxDim = Math.max(structure.size[0], structure.size[1], structure.size[2]);
    camera.position.set(maxDim * 1.2, maxDim * 0.8, maxDim * 1.2);
    camera.lookAt(0, 0, 0);

    const ambLight = new THREE.AmbientLight(0xffffff, 0.95);
    scene.add(ambLight);
    const dirLight = new THREE.DirectionalLight(0xfff6dd, 0.5);
    dirLight.position.set(1, 2.2, 1.4);
    scene.add(dirLight);
    const fillLight = new THREE.DirectionalLight(0xb9d8ff, 0.24);
    fillLight.position.set(-1.2, 0.8, -0.8);
    scene.add(fillLight);

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
      ambLight.intensity = Math.max(0.45, brightness / 115);
      dirLight.intensity = brightness / 210;
      fillLight.intensity = brightness / 420;
      renderer.render(scene, camera);
    }
    animate();

    return () => {
      cancelAnimationFrame(animId);
      window.removeEventListener('pointerup', onUp);
      ro.disconnect();
      geometryCache.forEach((geo) => geo.dispose());
      textureMaterialCache.forEach((mat) => {
        mat.map?.dispose();
        mat.dispose();
      });
      blockMaterialCache.clear();
      geometryCache.clear();
      failedTextures.clear();
      missingTexture?.dispose();
      renderer.dispose();
    };
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
