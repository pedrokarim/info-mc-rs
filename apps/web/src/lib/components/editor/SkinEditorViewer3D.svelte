<script lang="ts">
  import { onMount } from 'svelte';
  import * as THREE from 'three';
  import {
    editorState, getTextureDataUrl,
    editorSetPixel, editorGetPixel, editorFloodFill,
    pushHistory, addRecentColor,
  } from '$lib/stores/skin-editor.svelte';
  import { getUVTable } from '$lib/utils/skin-uv-regions';
  import type { Color } from '$lib/utils/skin-canvas-tools';

  let container: HTMLDivElement;
  let animId: number;

  // ── UV helpers ────────────────────────────────────────────────────
  function applyFaceUVs(
    geo: THREE.BoxGeometry,
    faces: [number, number, number, number][],
    tw = 64, th = 64
  ) {
    const uvAttr = geo.attributes.uv as THREE.BufferAttribute;
    for (let fi = 0; fi < 6; fi++) {
      const [px, py, pw, ph] = faces[fi];
      const flipX = pw < 0;
      const flipY = ph < 0;
      const ax = px / tw;
      const ay = 1 - py / th;
      const bx = (px + Math.abs(pw)) / tw;
      const by = 1 - (py + Math.abs(ph)) / th;
      const u0 = flipX ? bx : ax;
      const u1 = flipX ? ax : bx;
      const v0 = flipY ? ay : by;
      const v1 = flipY ? by : ay;
      uvAttr.setXY(fi * 4 + 0, u0, v1);
      uvAttr.setXY(fi * 4 + 1, u1, v1);
      uvAttr.setXY(fi * 4 + 2, u0, v0);
      uvAttr.setXY(fi * 4 + 3, u1, v0);
    }
    uvAttr.needsUpdate = true;
  }

  function makePart(
    w: number, h: number, d: number,
    uvKey: string,
    partId: string,
    UV: Record<string, [number, number, number, number][]>,
    baseMat: THREE.MeshLambertMaterial,
    overlayKey?: string,
    overlayMat?: THREE.MeshLambertMaterial,
    overlap = 0.5
  ): THREE.Group {
    const group = new THREE.Group();
    const geo = new THREE.BoxGeometry(w, h, d);
    applyFaceUVs(geo, UV[uvKey]);
    const baseMesh = new THREE.Mesh(geo, baseMat);
    baseMesh.userData = { partId, isOverlay: false };
    group.add(baseMesh);
    if (overlayKey && overlayMat) {
      const ogeo = new THREE.BoxGeometry(w + overlap, h + overlap, d + overlap);
      applyFaceUVs(ogeo, UV[overlayKey]);
      const ovMesh = new THREE.Mesh(ogeo, overlayMat);
      ovMesh.userData = { partId, isOverlay: true };
      group.add(ovMesh);
    }
    return group;
  }

  onMount(() => {
    const initW = container.clientWidth || 600;
    const initH = container.clientHeight || 500;

    // ── Three.js setup ─────────────────────────────────────────────
    const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setSize(initW, initH);
    renderer.setPixelRatio(window.devicePixelRatio);
    container.appendChild(renderer.domElement);

    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(38, initW / initH, 0.1, 1000);
    camera.position.set(0, 0, 60);
    camera.lookAt(0, 0, 0);

    scene.add(new THREE.AmbientLight(0xffffff, 0.7));
    const dir = new THREE.DirectionalLight(0xffffff, 0.3);
    dir.position.set(0.678, 0.284, 0.678);
    scene.add(dir);

    // Responsive resize
    const ro = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const { width: rw, height: rh } = entry.contentRect;
        if (rw > 0 && rh > 0) {
          renderer.setSize(rw, rh);
          camera.aspect = rw / rh;
          camera.updateProjectionMatrix();
        }
      }
    });
    ro.observe(container);

    // ── Texture setup ──────────────────────────────────────────────
    // Use CanvasTexture instead of Image loading for instant updates
    const texCanvas = document.createElement('canvas');
    texCanvas.width = 64;
    texCanvas.height = 64;
    const texCtx = texCanvas.getContext('2d')!;
    const tex = new THREE.CanvasTexture(texCanvas);
    tex.magFilter = THREE.NearestFilter;
    tex.minFilter = THREE.NearestFilter;
    tex.colorSpace = THREE.SRGBColorSpace;

    const mat = new THREE.MeshLambertMaterial({ map: tex });
    const matOv = new THREE.MeshLambertMaterial({ map: tex, transparent: true, alphaTest: 0.1 });

    // ── Model ──────────────────────────────────────────────────────
    const root = new THREE.Group();
    scene.add(root);

    let rArmPivot: THREE.Group;
    let lArmPivot: THREE.Group;
    let rLegPivot: THREE.Group;
    let lLegPivot: THREE.Group;
    const partGroups = new Map<string, THREE.Group>();
    let allMeshes: THREE.Mesh[] = [];
    let currentSlim = editorState.slim;

    function buildModel() {
      while (root.children.length > 0) root.remove(root.children[0]);
      partGroups.clear();
      allMeshes = [];

      const UV = getUVTable(editorState.slim);
      const armW = editorState.slim ? 3 : 4;
      const armOffX = editorState.slim ? 5.5 : 6;

      const headPivot = new THREE.Group();
      const headMesh = makePart(8, 8, 8, 'head', 'head', UV, mat, 'hat', matOv, 1.0);
      headPivot.add(headMesh);
      headPivot.position.set(0, 12, 0);
      root.add(headPivot);
      partGroups.set('head', headMesh);

      const body = makePart(8, 12, 4, 'body', 'body', UV, mat, 'jacket', matOv, 0.5);
      body.position.set(0, 2, 0);
      root.add(body);
      partGroups.set('body', body);

      rArmPivot = new THREE.Group();
      const rArmMesh = makePart(armW, 12, 4, 'rarm', 'rarm', UV, mat, 'rsleeve', matOv, 0.5);
      rArmMesh.position.set(0, -6, 0);
      rArmPivot.add(rArmMesh);
      rArmPivot.position.set(-armOffX, 8, 0);
      root.add(rArmPivot);
      partGroups.set('rarm', rArmMesh);

      lArmPivot = new THREE.Group();
      const lArmMesh = makePart(armW, 12, 4, 'larm', 'larm', UV, mat, 'lsleeve', matOv, 0.5);
      lArmMesh.position.set(0, -6, 0);
      lArmPivot.add(lArmMesh);
      lArmPivot.position.set(armOffX, 8, 0);
      root.add(lArmPivot);
      partGroups.set('larm', lArmMesh);

      rLegPivot = new THREE.Group();
      const rLegMesh = makePart(4, 12, 4, 'rleg', 'rleg', UV, mat, 'rpant', matOv, 0.5);
      rLegMesh.position.set(0, -6, 0);
      rLegPivot.add(rLegMesh);
      rLegPivot.position.set(-2, -4, 0);
      root.add(rLegPivot);
      partGroups.set('rleg', rLegMesh);

      lLegPivot = new THREE.Group();
      const lLegMesh = makePart(4, 12, 4, 'lleg', 'lleg', UV, mat, 'lpant', matOv, 0.5);
      lLegMesh.position.set(0, -6, 0);
      lLegPivot.add(lLegMesh);
      lLegPivot.position.set(2, -4, 0);
      root.add(lLegPivot);
      partGroups.set('lleg', lLegMesh);

      currentSlim = editorState.slim;

      // Collect all meshes flat for raycasting
      allMeshes = [];
      root.traverse((child) => {
        if (child instanceof THREE.Mesh && child.userData.partId) {
          allMeshes.push(child);
        }
      });
    }

    buildModel();

    // ── Sync texture from pixel buffer (direct canvas copy, no Image loading) ──
    let lastVersion = -1;

    function syncTexture() {
      const curVer = editorState.textureVersion;
      if (curVer === lastVersion) return;
      lastVersion = curVer;
      const imgData = new ImageData(new Uint8ClampedArray(editorState.pixels), 64, 64);
      texCtx.putImageData(imgData, 0, 0);
      tex.needsUpdate = true;
    }

    syncTexture();

    // ── Rotation ────────────────────────────────────────────────────
    let theta = THREE.MathUtils.degToRad(30);
    let phi = THREE.MathUtils.degToRad(21);
    let lastCameraReset = editorState.cameraResetTrigger;

    function applyRotation() {
      root.rotation.y = theta;
      root.rotation.x = phi;
    }
    applyRotation();

    // ── Raycasting ──────────────────────────────────────────────────
    const raycaster = new THREE.Raycaster();
    const mouseNDC = new THREE.Vector2();

    function raycast(e: PointerEvent): { x: number; y: number } | null {
      const rect = renderer.domElement.getBoundingClientRect();
      mouseNDC.x = ((e.clientX - rect.left) / rect.width) * 2 - 1;
      mouseNDC.y = -((e.clientY - rect.top) / rect.height) * 2 + 1;

      root.updateMatrixWorld(true);
      raycaster.setFromCamera(mouseNDC, camera);

      // Only raycast against meshes matching the active layer
      const isOverlay = editorState.activeLayer === 'overlay';
      const targets = allMeshes.filter((m) => m.userData.isOverlay === isOverlay && m.visible);

      const hits = raycaster.intersectObjects(targets, false);
      if (hits.length === 0) return null;

      const hit = hits[0];
      if (!hit.uv) return null;

      const px = Math.min(63, Math.max(0, Math.floor(hit.uv.x * 64)));
      const py = Math.min(63, Math.max(0, Math.floor((1 - hit.uv.y) * 64)));
      return { x: px, y: py };
    }

    // ── Pointer state ───────────────────────────────────────────────
    let rotating = false;
    let painting = false;
    let lastPointerX = 0;
    let lastPointerY = 0;
    let lastPaintPixel: { x: number; y: number } | null = null;
    const cvs = renderer.domElement;

    function currentToolColor(): Color {
      if (editorState.activeTool === 'eraser') return { r: 0, g: 0, b: 0, a: 0 };
      return { ...editorState.primaryColor };
    }

    function canPaint(): boolean {
      const t = editorState.activeTool;
      return t === 'pencil' || t === 'eraser' || t === 'eyedropper' || t === 'fill';
    }

    cvs.addEventListener('pointerdown', (e: PointerEvent) => {
      // Right-click or middle-click → always rotate
      if (e.button === 2 || e.button === 1) {
        rotating = true;
        lastPointerX = e.clientX;
        lastPointerY = e.clientY;
        cvs.setPointerCapture(e.pointerId);
        return;
      }

      if (e.button !== 0) return;

      // Left-click: try to paint if tool allows
      if (canPaint()) {
        const p = raycast(e);
        if (p) {
          // Eyedropper
          if (editorState.activeTool === 'eyedropper') {
            const c = editorGetPixel(p.x, p.y);
            editorState.primaryColor = { ...c };
            return;
          }
          // Fill
          if (editorState.activeTool === 'fill') {
            editorFloodFill(p.x, p.y, currentToolColor());
            addRecentColor({ ...editorState.primaryColor });
            return;
          }
          // Pencil / Eraser — start drag painting
          pushHistory();
          painting = true;
          lastPaintPixel = p;
          editorSetPixel(p.x, p.y, currentToolColor());
          if (editorState.activeTool === 'pencil') addRecentColor({ ...editorState.primaryColor });
          cvs.setPointerCapture(e.pointerId);
          return;
        }
      }

      // Didn't hit model or tool is pan → rotate
      rotating = true;
      lastPointerX = e.clientX;
      lastPointerY = e.clientY;
      cvs.setPointerCapture(e.pointerId);
    });

    cvs.addEventListener('pointermove', (e: PointerEvent) => {
      if (rotating) {
        theta += (e.clientX - lastPointerX) * 0.01;
        phi = THREE.MathUtils.clamp(phi + (e.clientY - lastPointerY) * 0.01, -Math.PI / 2, Math.PI / 2);
        lastPointerX = e.clientX;
        lastPointerY = e.clientY;
        applyRotation();
        return;
      }

      if (painting && lastPaintPixel) {
        const p = raycast(e);
        if (p) {
          const color = currentToolColor();
          // Interpolate for smooth strokes
          const dx = Math.abs(p.x - lastPaintPixel.x);
          const dy = Math.abs(p.y - lastPaintPixel.y);
          if (dx > 1 || dy > 1) {
            const steps = Math.max(dx, dy);
            for (let i = 1; i <= steps; i++) {
              const t = i / steps;
              const ix = Math.round(lastPaintPixel.x + (p.x - lastPaintPixel.x) * t);
              const iy = Math.round(lastPaintPixel.y + (p.y - lastPaintPixel.y) * t);
              editorSetPixel(ix, iy, color);
            }
          } else {
            editorSetPixel(p.x, p.y, color);
          }
          lastPaintPixel = p;
        }
      }
    });

    function onPointerUp() {
      rotating = false;
      painting = false;
      lastPaintPixel = null;
    }
    cvs.addEventListener('pointerup', onPointerUp);
    window.addEventListener('pointerup', onPointerUp);
    cvs.addEventListener('contextmenu', (e) => e.preventDefault());

    // Touch: always rotate
    cvs.addEventListener('touchstart', (e) => {
      rotating = true;
      lastPointerX = e.touches[0].clientX;
      lastPointerY = e.touches[0].clientY;
    }, { passive: true });
    cvs.addEventListener('touchend', () => { rotating = false; });
    cvs.addEventListener('touchmove', (e) => {
      if (!rotating) return;
      theta += (e.touches[0].clientX - lastPointerX) * 0.01;
      phi = THREE.MathUtils.clamp(phi + (e.touches[0].clientY - lastPointerY) * 0.01, -Math.PI / 2, Math.PI / 2);
      lastPointerX = e.touches[0].clientX;
      lastPointerY = e.touches[0].clientY;
      applyRotation();
    }, { passive: true });

    // ── Animation loop ────────────────────────────────────────────
    let t = 0;
    function animate() {
      animId = requestAnimationFrame(animate);

      // Sync texture from pixel buffer
      syncTexture();

      // Rebuild if slim changed
      if (editorState.slim !== currentSlim) buildModel();

      // Per-part visibility
      for (const [partId, group] of partGroups) {
        const vis = editorState.partVisibility[partId];
        if (!vis) continue;
        group.traverse((child) => {
          if (child instanceof THREE.Mesh && child.userData.partId) {
            child.visible = child.userData.isOverlay ? vis.overlay : vis.base;
          }
        });
      }

      // Wireframe
      mat.wireframe = editorState.wireframeMode;
      matOv.wireframe = editorState.wireframeMode;

      // Camera reset
      if (editorState.cameraResetTrigger !== lastCameraReset) {
        lastCameraReset = editorState.cameraResetTrigger;
        theta = THREE.MathUtils.degToRad(30);
        phi = THREE.MathUtils.degToRad(21);
      }

      // Walk animation
      if (!editorState.animationPaused) {
        t += 360 / 1500;
        const rad = THREE.MathUtils.degToRad(t);
        if (rArmPivot) rArmPivot.rotation.x = -THREE.MathUtils.degToRad(18) * Math.sin(rad);
        if (lArmPivot) lArmPivot.rotation.x = THREE.MathUtils.degToRad(18) * Math.sin(rad);
        if (rLegPivot) rLegPivot.rotation.x = THREE.MathUtils.degToRad(20) * Math.sin(rad);
        if (lLegPivot) lLegPivot.rotation.x = -THREE.MathUtils.degToRad(20) * Math.sin(rad);
      }

      applyRotation();
      renderer.render(scene, camera);
    }
    animate();

    return () => {
      cancelAnimationFrame(animId);
      window.removeEventListener('pointerup', onPointerUp);
      ro.disconnect();
      renderer.dispose();
    };
  });

  const cursorClass = $derived(
    (() => {
      const t = editorState.activeTool;
      if (t === 'pencil' || t === 'eraser' || t === 'eyedropper' || t === 'fill') return 'cursor-paint';
      return 'cursor-grab';
    })()
  );
</script>

<div
  bind:this={container}
  class="editor-viewer-3d {cursorClass}"
  role="img"
  aria-label="Éditeur 3D du skin — clic gauche pour peindre, clic droit pour tourner"
></div>

<style>
  .editor-viewer-3d {
    display: block;
    width: 100%;
    height: 100%;
    min-height: 400px;
    border-radius: var(--radius-md, 12px);
    overflow: hidden;
    background: linear-gradient(135deg, rgba(94,144,255,0.08) 0%, rgba(94,144,255,0.02) 100%);
  }

  .cursor-paint { cursor: crosshair; }
  .cursor-grab { cursor: grab; }
  .cursor-grab:active { cursor: grabbing; }
</style>
