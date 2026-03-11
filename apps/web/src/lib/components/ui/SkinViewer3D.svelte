<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import * as THREE from 'three';

  export let skinUrl: string;
  export let slim: boolean = false;
  export let width: number = 240;
  export let height: number = 360;
  export let autoRotate: boolean = false;

  let container: HTMLDivElement;
  let animId: number;

  // ── UV helpers ────────────────────────────────────────────────────
  // Converts pixel rectangle [x,y,w,h] in a 64×64 texture to UV [0-1]
  // negative w/h = flip
  function uvRect(
    x: number, y: number, w: number, h: number,
    tw = 64, th = 64
  ): [number, number, number, number, boolean, boolean] {
    return [x / tw, y / th, Math.abs(w) / tw, Math.abs(h) / th, w < 0, h < 0];
  }

  // BoxGeometry face order: +x(right) -x(left) +y(top) -y(bottom) +z(front) -z(back)
  // Each face = 2 triangles, 4 vertices.
  // We set UVs per-face group (geometry.groups[i] → materialIndex i).
  function applyFaceUVs(
    geo: THREE.BoxGeometry,
    faces: [number, number, number, number][], // [x, y, w, h] for each of the 6 faces
    tw = 64, th = 64
  ) {
    const uvAttr = geo.attributes.uv as THREE.BufferAttribute;
    // Each face = 4 vertices → 8 floats in the uv buffer (2 per vertex)
    for (let fi = 0; fi < 6; fi++) {
      const [px, py, pw, ph] = faces[fi];
      const flipX = pw < 0;
      const flipY = ph < 0;
      const ax = px / tw;
      const ay = 1 - py / th;
      const bx = (px + Math.abs(pw)) / tw;
      const by = 1 - (py + Math.abs(ph)) / th;
      // UV layout for 4 verts of one BoxGeometry face (CCW winding):
      // v0 = bottom-left, v1 = bottom-right, v2 = top-left, v3 = top-right
      // Three.js order: [bl, br, tl, tr]
      let u0 = flipX ? bx : ax;
      let u1 = flipX ? ax : bx;
      let v0 = flipY ? ay : by;
      let v1 = flipY ? by : ay;

      const base = fi * 4 * 2;
      // bottom-left
      uvAttr.setXY(fi * 4 + 0, u0, v0);
      // bottom-right
      uvAttr.setXY(fi * 4 + 1, u1, v0);
      // top-left
      uvAttr.setXY(fi * 4 + 2, u0, v1);
      // top-right
      uvAttr.setXY(fi * 4 + 3, u1, v1);
    }
    uvAttr.needsUpdate = true;
  }

  // ── Skin UV tables ─────────────────────────────────────────────────
  // Format: [x, y, w, h]  — negative = flip
  // Face order: right, left, top, bottom, front, back
  const ARM_W = slim ? 3 : 4;

  const UV: Record<string, [number, number, number, number][]> = {
    head:  [[16,8,8,8],[0,8,8,8],[8,0,8,8],[16,0,8,8],[8,8,8,8],[24,8,8,8]],
    hat:   [[48,8,8,8],[32,8,8,8],[40,0,8,8],[48,0,8,8],[40,8,8,8],[56,8,8,8]],
    body:  [[28,20,4,12],[16,20,4,12],[20,16,8,4],[28,16,8,4],[20,20,8,12],[32,20,8,12]],
    jacket:[[32+4,36,4,12],[32,36,4,12],[32,32,8,4],[32+8,32,8,4],[32,36,8,12],[32+8+4,36,8,12]],
    rarm:  [[48,20,ARM_W,12],[40,20,ARM_W,12],[44,16,ARM_W,4],[48,16,ARM_W,4],[44,20,ARM_W,12],[52,20,ARM_W,12]],
    larm:  [[52,52,ARM_W,12],[40,52,ARM_W,12],[44,48,ARM_W,4],[48,48,ARM_W,4],[36,52,ARM_W,12],[52,52,ARM_W,12]],
    rsleeve:[[48+16,36,ARM_W,12],[40+16,36,ARM_W,12],[44+16,32,ARM_W,4],[48+16,32,ARM_W,4],[44+16,36,ARM_W,12],[52+16,36,ARM_W,12]],
    lsleeve:[[52,52,ARM_W,12],[40,52,ARM_W,12],[44,48,ARM_W,4],[48,48,ARM_W,4],[48,52,ARM_W,12],[52+ARM_W,52,ARM_W,12]],
    rleg:  [[8,20,4,12],[0,20,4,12],[4,16,4,4],[8,16,4,4],[4,20,4,12],[12,20,4,12]],
    lleg:  [[24,52,4,12],[16,52,4,12],[20,48,4,4],[24,48,4,4],[20,52,4,12],[28,52,4,12]],
    rpant: [[8,36,4,12],[0,36,4,12],[4,32,4,4],[8,32,4,4],[4,36,4,12],[12,36,4,12]],
    lpant: [[8,52,4,12],[0,52,4,12],[4,48,4,4],[8,48,4,4],[4,52,4,12],[12,52,4,12]],
  };

  // ── Part builder ──────────────────────────────────────────────────
  function makePart(
    w: number, h: number, d: number,
    uvKey: string,
    mat: THREE.MeshLambertMaterial,
    overlayKey?: string,
    overlayMat?: THREE.MeshLambertMaterial,
    eps = 0
  ): THREE.Group {
    const group = new THREE.Group();

    const geo = new THREE.BoxGeometry(w, h, d);
    applyFaceUVs(geo, UV[uvKey]);
    group.add(new THREE.Mesh(geo, mat));

    if (overlayKey && overlayMat) {
      const e = 0.5 + eps;
      const ogeo = new THREE.BoxGeometry(w + e, h + e, d + e);
      applyFaceUVs(ogeo, UV[overlayKey]);
      group.add(new THREE.Mesh(ogeo, overlayMat));
    }

    return group;
  }

  onMount(() => {
    // ── Renderer ────────────────────────────────────────────────────
    const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setSize(width, height);
    renderer.setPixelRatio(window.devicePixelRatio);
    container.appendChild(renderer.domElement);

    // ── Scene / camera ───────────────────────────────────────────────
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(38, width / height, 0.1, 1000);
    camera.position.set(0, 0, 60);
    camera.lookAt(0, 0, 0);

    // ── Lights ───────────────────────────────────────────────────────
    scene.add(new THREE.AmbientLight(0xffffff, 0.7));
    const dir = new THREE.DirectionalLight(0xffffff, 0.3);
    dir.position.set(0.678, 0.284, 0.678);
    scene.add(dir);

    // ── Texture ──────────────────────────────────────────────────────
    const loader = new THREE.TextureLoader();
    const tex = loader.load(skinUrl);
    tex.magFilter = THREE.NearestFilter;
    tex.minFilter = THREE.NearestFilter;
    tex.colorSpace = THREE.SRGBColorSpace;

    const mat = new THREE.MeshLambertMaterial({ map: tex });
    const matOv = new THREE.MeshLambertMaterial({ map: tex, transparent: true, alphaTest: 0.1 });

    // ── Root pivot ───────────────────────────────────────────────────
    const root = new THREE.Group();
    scene.add(root);

    // Head
    const headPivot = new THREE.Group();
    const headMesh = makePart(8, 8, 8, 'head', mat, 'hat', matOv);
    headMesh.position.set(0, 4, 0); // center pivot at neck
    headPivot.add(headMesh);
    headPivot.position.set(0, 12, 0);
    root.add(headPivot);

    // Body
    const body = makePart(8, 12, 4, 'body', mat, 'jacket', matOv, 0);
    body.position.set(0, 2, 0);
    root.add(body);

    // Arms
    const armW = slim ? 3 : 4;
    const armOffX = slim ? 5.5 : 6;

    const rArmPivot = new THREE.Group();
    const rArmMesh = makePart(armW, 12, 4, 'rarm', mat, 'rsleeve', matOv, 0.5 * 4 - 0.5);
    rArmMesh.position.set(0, -6, 0); // pivot at shoulder
    rArmPivot.add(rArmMesh);
    rArmPivot.position.set(-(armOffX + armW / 2 - armW / 2), 8, 0);
    root.add(rArmPivot);

    const lArmPivot = new THREE.Group();
    const lArmMesh = makePart(armW, 12, 4, 'larm', mat, 'lsleeve', matOv, 0.5 * 4 - 0.5);
    lArmMesh.position.set(0, -6, 0);
    lArmPivot.add(lArmMesh);
    lArmPivot.position.set(armOffX + armW / 2 - armW / 2, 8, 0);
    root.add(lArmPivot);

    // Legs
    const rLegPivot = new THREE.Group();
    const rLegMesh = makePart(4, 12, 4, 'rleg', mat, 'rpant', matOv, 0.5 * 2 - 0.5);
    rLegMesh.position.set(0, -6, 0);
    rLegPivot.add(rLegMesh);
    rLegPivot.position.set(-2, -4, 0);
    root.add(rLegPivot);

    const lLegPivot = new THREE.Group();
    const lLegMesh = makePart(4, 12, 4, 'lleg', mat, 'lpant', matOv, 0.5 * 3 - 0.5);
    lLegMesh.position.set(0, -6, 0);
    lLegPivot.add(lLegMesh);
    lLegPivot.position.set(2, -4, 0);
    root.add(lLegPivot);

    // Center model vertically
    root.position.set(0, 4, 0);

    // ── Drag rotation ────────────────────────────────────────────────
    let theta = THREE.MathUtils.degToRad(30);
    let phi = THREE.MathUtils.degToRad(21);
    let dragging = false;
    let lastX = 0, lastY = 0;

    function applyRotation() {
      root.rotation.y = theta;
      root.rotation.x = phi;
    }
    applyRotation();

    const canvas = renderer.domElement;

    canvas.addEventListener('mousedown', (e) => { dragging = true; lastX = e.clientX; lastY = e.clientY; });
    window.addEventListener('mouseup', () => { dragging = false; });
    window.addEventListener('mousemove', (e) => {
      if (!dragging) return;
      theta += (e.clientX - lastX) * 0.01;
      phi = THREE.MathUtils.clamp(phi + (e.clientY - lastY) * 0.01, -Math.PI / 2, Math.PI / 2);
      lastX = e.clientX; lastY = e.clientY;
      applyRotation();
    });

    canvas.addEventListener('touchstart', (e) => {
      dragging = true;
      lastX = e.touches[0].clientX; lastY = e.touches[0].clientY;
    }, { passive: true });
    canvas.addEventListener('touchend', () => { dragging = false; });
    canvas.addEventListener('touchmove', (e) => {
      if (!dragging) return;
      theta += (e.touches[0].clientX - lastX) * 0.01;
      phi = THREE.MathUtils.clamp(phi + (e.touches[0].clientY - lastY) * 0.01, -Math.PI / 2, Math.PI / 2);
      lastX = e.touches[0].clientX; lastY = e.touches[0].clientY;
      applyRotation();
    }, { passive: true });

    // ── Animation loop ───────────────────────────────────────────────
    let t = 0;
    function animate() {
      animId = requestAnimationFrame(animate);
      t += 360 / 1500;

      const rad = THREE.MathUtils.degToRad(t);
      rArmPivot.rotation.x = -THREE.MathUtils.degToRad(18) * Math.sin(rad);
      lArmPivot.rotation.x =  THREE.MathUtils.degToRad(18) * Math.sin(rad);
      rLegPivot.rotation.x =  THREE.MathUtils.degToRad(20) * Math.sin(rad);
      lLegPivot.rotation.x = -THREE.MathUtils.degToRad(20) * Math.sin(rad);

      if (autoRotate) theta += 0.005;
      applyRotation();

      renderer.render(scene, camera);
    }
    animate();

    return () => {};
  });

  onDestroy(() => {
    cancelAnimationFrame(animId);
  });
</script>

<div
  bind:this={container}
  class="skin-viewer-3d"
  style="width:{width}px; height:{height}px; cursor:grab;"
  role="img"
  aria-label="Minecraft 3D skin viewer"
></div>

<style>
  .skin-viewer-3d {
    display: block;
    border-radius: 10px;
    overflow: hidden;
    background: transparent;
  }
  .skin-viewer-3d:active {
    cursor: grabbing;
  }
</style>
