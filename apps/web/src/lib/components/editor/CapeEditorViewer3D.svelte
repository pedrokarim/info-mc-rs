<script lang="ts">
  import { onMount } from 'svelte';
  import * as THREE from 'three';
  import {
    capeState, getTextureDataUrl,
    capeSetPixel, capeGetPixel, capeFloodFill,
    pushHistory, addRecentColor,
  } from '$lib/stores/cape-editor.svelte';
  import { getCapeUVs, getElytraUVs } from '$lib/utils/cape-uv-regions';
  import { getUVTable } from '$lib/utils/skin-uv-regions';
  import type { Color } from '$lib/utils/skin-canvas-tools';

  let container: HTMLDivElement;
  let animId: number;

  function applyFaceUVs(geo: THREE.BoxGeometry, faces: [number, number, number, number][], tw: number, th: number) {
    const uvAttr = geo.attributes.uv as THREE.BufferAttribute;
    for (let fi = 0; fi < 6; fi++) {
      const [px, py, pw, ph] = faces[fi];
      const flipX = pw < 0, flipY = ph < 0;
      const ax = px / tw, ay = 1 - py / th;
      const bx = (px + Math.abs(pw)) / tw, by = 1 - (py + Math.abs(ph)) / th;
      const u0 = flipX ? bx : ax, u1 = flipX ? ax : bx;
      const v0 = flipY ? ay : by, v1 = flipY ? by : ay;
      uvAttr.setXY(fi * 4 + 0, u0, v1);
      uvAttr.setXY(fi * 4 + 1, u1, v1);
      uvAttr.setXY(fi * 4 + 2, u0, v0);
      uvAttr.setXY(fi * 4 + 3, u1, v0);
    }
    uvAttr.needsUpdate = true;
  }

  function makeSkinPart(w: number, h: number, d: number, uvKey: string, UV: Record<string, [number,number,number,number][]>, mat: THREE.MeshLambertMaterial, overlayKey?: string, matOv?: THREE.MeshLambertMaterial): THREE.Group {
    const group = new THREE.Group();
    const geo = new THREE.BoxGeometry(w, h, d);
    applyFaceUVs(geo, UV[uvKey], 64, 64);
    group.add(new THREE.Mesh(geo, mat));
    if (overlayKey && matOv) {
      const ogeo = new THREE.BoxGeometry(w + 0.5, h + 0.5, d + 0.5);
      applyFaceUVs(ogeo, UV[overlayKey], 64, 64);
      group.add(new THREE.Mesh(ogeo, matOv));
    }
    return group;
  }

  onMount(() => {
    const initW = container.clientWidth || 600;
    const initH = container.clientHeight || 500;

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

    const ro = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const { width: rw, height: rh } = entry.contentRect;
        if (rw > 0 && rh > 0) { renderer.setSize(rw, rh); camera.aspect = rw / rh; camera.updateProjectionMatrix(); }
      }
    });
    ro.observe(container);

    // ── Steve body (static, from skin texture) ──────────────────────
    const skinImg = new Image();
    skinImg.crossOrigin = 'anonymous';
    const skinTex = new THREE.Texture(skinImg);
    skinTex.magFilter = THREE.NearestFilter;
    skinTex.minFilter = THREE.NearestFilter;
    skinTex.colorSpace = THREE.SRGBColorSpace;
    skinImg.onload = () => { skinTex.needsUpdate = true; };
    skinImg.src = '/images/skins/steve.png';

    const skinMat = new THREE.MeshLambertMaterial({ map: skinTex });
    const skinMatOv = new THREE.MeshLambertMaterial({ map: skinTex, transparent: true, alphaTest: 0.1 });

    const root = new THREE.Group();
    scene.add(root);

    const UV = getUVTable(false);
    const bodyGroup = new THREE.Group(); // group all body parts for visibility toggle

    const headPivot = new THREE.Group();
    headPivot.add(makeSkinPart(8, 8, 8, 'head', UV, skinMat, 'hat', skinMatOv));
    headPivot.position.set(0, 12, 0);
    bodyGroup.add(headPivot);

    const bodyMesh = makeSkinPart(8, 12, 4, 'body', UV, skinMat, 'jacket', skinMatOv);
    bodyMesh.position.set(0, 2, 0);
    bodyGroup.add(bodyMesh);

    const rArm = makeSkinPart(4, 12, 4, 'rarm', UV, skinMat, 'rsleeve', skinMatOv);
    rArm.position.set(0, -6, 0);
    const rArmP = new THREE.Group(); rArmP.add(rArm); rArmP.position.set(-6, 8, 0);
    bodyGroup.add(rArmP);

    const lArm = makeSkinPart(4, 12, 4, 'larm', UV, skinMat, 'lsleeve', skinMatOv);
    lArm.position.set(0, -6, 0);
    const lArmP = new THREE.Group(); lArmP.add(lArm); lArmP.position.set(6, 8, 0);
    bodyGroup.add(lArmP);

    const rLeg = makeSkinPart(4, 12, 4, 'rleg', UV, skinMat, 'rpant', skinMatOv);
    rLeg.position.set(0, -6, 0);
    const rLegP = new THREE.Group(); rLegP.add(rLeg); rLegP.position.set(-2, -4, 0);
    bodyGroup.add(rLegP);

    const lLeg = makeSkinPart(4, 12, 4, 'lleg', UV, skinMat, 'lpant', skinMatOv);
    lLeg.position.set(0, -6, 0);
    const lLegP = new THREE.Group(); lLegP.add(lLeg); lLegP.position.set(2, -4, 0);
    bodyGroup.add(lLegP);

    root.add(bodyGroup);

    // ── Cape texture (editable, from capeState) ─────────────────────
    const texCanvas = document.createElement('canvas');
    texCanvas.width = capeState.width;
    texCanvas.height = capeState.height;
    const texCtx = texCanvas.getContext('2d')!;
    const capeTex = new THREE.CanvasTexture(texCanvas);
    capeTex.magFilter = THREE.NearestFilter;
    capeTex.minFilter = THREE.NearestFilter;
    capeTex.colorSpace = THREE.SRGBColorSpace;

    const capeMat = new THREE.MeshLambertMaterial({ map: capeTex, side: THREE.DoubleSide, transparent: true, alphaTest: 0.1 });

    // Cape mesh
    const capeGeo = new THREE.BoxGeometry(10, 16, 1);
    capeGeo.translate(0, -8, 0.5);
    applyFaceUVs(capeGeo, getCapeUVs(1, capeState.width, capeState.height), capeState.width, capeState.height);
    const capeMesh = new THREE.Mesh(capeGeo, capeMat);
    capeMesh.userData = { isCape: true };

    const capeGroup = new THREE.Group();
    capeGroup.position.set(0, 8, -2);
    capeGroup.rotation.y = Math.PI;
    capeGroup.add(capeMesh);
    root.add(capeGroup);

    // Elytra meshes
    const elytraUVs = getElytraUVs(1, capeState.width, capeState.height);
    const leftGeo = new THREE.BoxGeometry(12, 22, 4);
    applyFaceUVs(leftGeo, elytraUVs, capeState.width, capeState.height);
    const leftMesh = new THREE.Mesh(leftGeo, capeMat);
    leftMesh.position.set(-5, -10, 0);
    leftMesh.userData = { isCape: true };

    const elytraLeft = new THREE.Group();
    elytraLeft.position.set(5, 0, 0);
    elytraLeft.rotation.set(0.2618, 0.01, 0.1);
    elytraLeft.add(leftMesh);

    const rightGeo = new THREE.BoxGeometry(12, 22, 4);
    applyFaceUVs(rightGeo, elytraUVs, capeState.width, capeState.height);
    const rightMesh = new THREE.Mesh(rightGeo, capeMat);
    rightMesh.scale.x = -1;
    rightMesh.position.set(5, -10, 0);
    rightMesh.userData = { isCape: true };

    const elytraRight = new THREE.Group();
    elytraRight.position.set(-5, 0, 0);
    elytraRight.rotation.set(0.2618, -0.01, -0.1);
    elytraRight.add(rightMesh);

    const elytraGroup = new THREE.Group();
    elytraGroup.position.set(0, 8, 0);
    elytraGroup.add(elytraLeft);
    elytraGroup.add(elytraRight);
    root.add(elytraGroup);

    // Collect paintable cape meshes
    const capeMeshes = [capeMesh, leftMesh, rightMesh];

    // ── Sync texture ────────────────────────────────────────────────
    let lastVersion = -1;
    function syncTexture() {
      const curVer = capeState.textureVersion;
      if (curVer === lastVersion) return;
      lastVersion = curVer;
      texCanvas.width = capeState.width;
      texCanvas.height = capeState.height;
      const imgData = new ImageData(new Uint8ClampedArray(capeState.pixels), capeState.width, capeState.height);
      texCtx.putImageData(imgData, 0, 0);
      capeTex.needsUpdate = true;
    }
    syncTexture();

    // ── Rotation (default: view from back) ──────────────────────────
    let theta = Math.PI;
    let phi = THREE.MathUtils.degToRad(10);
    let lastCameraReset = capeState.cameraResetTrigger;
    function applyRotation() { root.rotation.y = theta; root.rotation.x = phi; }
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

      const visibleMeshes = capeMeshes.filter((m) => {
        let p: THREE.Object3D | null = m;
        while (p) { if (!p.visible) return false; p = p.parent; }
        return true;
      });

      const hits = raycaster.intersectObjects(visibleMeshes, false);
      if (hits.length === 0 || !hits[0].uv) return null;
      const uv = hits[0].uv;
      const px = Math.min(capeState.width - 1, Math.max(0, Math.floor(uv.x * capeState.width)));
      const py = Math.min(capeState.height - 1, Math.max(0, Math.floor((1 - uv.y) * capeState.height)));
      return { x: px, y: py };
    }

    function canPaint(): boolean {
      const t = capeState.activeTool;
      return t === 'pencil' || t === 'eraser' || t === 'eyedropper' || t === 'fill';
    }
    function toolColor(): Color {
      return capeState.activeTool === 'eraser' ? { r: 0, g: 0, b: 0, a: 0 } : { ...capeState.primaryColor };
    }

    // ── Pointer events ──────────────────────────────────────────────
    let rotating = false, painting = false;
    let lastPX = 0, lastPY = 0;
    let lastPaintPx: { x: number; y: number } | null = null;
    const cvs = renderer.domElement;

    cvs.addEventListener('pointerdown', (e) => {
      if (e.button === 2 || e.button === 1) {
        rotating = true; lastPX = e.clientX; lastPY = e.clientY; cvs.setPointerCapture(e.pointerId); return;
      }
      if (e.button !== 0) return;
      if (canPaint()) {
        const p = raycast(e);
        if (p) {
          if (capeState.activeTool === 'eyedropper') { capeState.primaryColor = { ...capeGetPixel(p.x, p.y) }; return; }
          if (capeState.activeTool === 'fill') { capeFloodFill(p.x, p.y, toolColor()); addRecentColor({ ...capeState.primaryColor }); return; }
          pushHistory(); painting = true; lastPaintPx = p;
          capeSetPixel(p.x, p.y, toolColor());
          if (capeState.activeTool === 'pencil') addRecentColor({ ...capeState.primaryColor });
          cvs.setPointerCapture(e.pointerId); return;
        }
      }
      rotating = true; lastPX = e.clientX; lastPY = e.clientY; cvs.setPointerCapture(e.pointerId);
    });

    cvs.addEventListener('pointermove', (e) => {
      if (rotating) {
        theta += (e.clientX - lastPX) * 0.01;
        phi = THREE.MathUtils.clamp(phi + (e.clientY - lastPY) * 0.01, -Math.PI / 2, Math.PI / 2);
        lastPX = e.clientX; lastPY = e.clientY; applyRotation(); return;
      }
      if (painting && lastPaintPx) {
        const p = raycast(e);
        if (p) {
          const c = toolColor();
          const dx = Math.abs(p.x - lastPaintPx.x), dy = Math.abs(p.y - lastPaintPx.y);
          if (dx > 1 || dy > 1) {
            const steps = Math.max(dx, dy);
            for (let i = 1; i <= steps; i++) {
              const t = i / steps;
              capeSetPixel(Math.round(lastPaintPx.x + (p.x - lastPaintPx.x) * t), Math.round(lastPaintPx.y + (p.y - lastPaintPx.y) * t), c);
            }
          } else { capeSetPixel(p.x, p.y, c); }
          lastPaintPx = p;
        }
      }
    });

    const onPointerUp = () => { rotating = false; painting = false; lastPaintPx = null; };
    cvs.addEventListener('pointerup', onPointerUp);
    window.addEventListener('pointerup', onPointerUp);
    cvs.addEventListener('contextmenu', (e) => e.preventDefault());

    // ── Animation loop ────────────────────────────────────────────
    function animate() {
      animId = requestAnimationFrame(animate);
      syncTexture();

      bodyGroup.visible = capeState.showBody;
      capeGroup.visible = capeState.backEquipment === 'cape';
      elytraGroup.visible = capeState.backEquipment === 'elytra';

      capeMat.wireframe = capeState.wireframeMode;
      if (capeState.cameraResetTrigger !== lastCameraReset) {
        lastCameraReset = capeState.cameraResetTrigger;
        theta = Math.PI; phi = THREE.MathUtils.degToRad(10);
      }
      applyRotation();
      renderer.render(scene, camera);
    }
    animate();

    return () => { cancelAnimationFrame(animId); window.removeEventListener('pointerup', onPointerUp); ro.disconnect(); renderer.dispose(); };
  });

  const cursorClass = $derived((() => {
    const t = capeState.activeTool;
    return (t === 'pencil' || t === 'eraser' || t === 'eyedropper' || t === 'fill') ? 'cursor-paint' : 'cursor-grab';
  })());
</script>

<div bind:this={container} class="cape-viewer-3d {cursorClass}" role="img" aria-label="Editeur 3D de cape"></div>

<style>
  .cape-viewer-3d { display: block; width: 100%; height: 100%; min-height: 400px; border-radius: var(--radius-md, 12px); overflow: hidden; background: linear-gradient(135deg, rgba(94,144,255,0.08) 0%, rgba(94,144,255,0.02) 100%); }
  .cursor-paint { cursor: crosshair; }
  .cursor-grab { cursor: grab; }
  .cursor-grab:active { cursor: grabbing; }
</style>
