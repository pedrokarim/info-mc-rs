<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import * as THREE from 'three';

  export let skinUrl: string;
  export let capeUrl: string | undefined = undefined;
  export let slim: boolean = false;
  export let width: number = 240;
  export let height: number = 360;
  export let autoRotate: boolean = false;

  let container: HTMLDivElement;
  let animId: number;

  // ── UV helpers ────────────────────────────────────────────────────
  // BoxGeometry face order: +x(right) -x(left) +y(top) -y(bottom) +z(front) -z(back)
  // [x, y, w, h] in texture pixels — negative w/h = flip
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
      // Three.js BoxGeometry vertex order per face: tl(0), tr(1), bl(2), br(3)
      uvAttr.setXY(fi * 4 + 0, u0, v1); // top-left    → top of skin region
      uvAttr.setXY(fi * 4 + 1, u1, v1); // top-right   → top of skin region
      uvAttr.setXY(fi * 4 + 2, u0, v0); // bottom-left → bottom of skin region
      uvAttr.setXY(fi * 4 + 3, u1, v0); // bottom-right→ bottom of skin region
    }
    uvAttr.needsUpdate = true;
  }

  // ── UV tables (from NameMC script.js SKIN[1]) ────────────────────
  // Face order: +x, -x, +y, -y, +z, -z
  // Arm side faces (+x/-x) always use depth=4, not AW
  const AW = slim ? 3 : 4;

  const UV: Record<string, [number, number, number, number][]> = {
    head:    [[16,8,8,8],[0,8,8,8],[8,0,8,8],[16,0,8,8],[8,8,8,8],[24,8,8,8]],
    hat:     [[48,8,8,8],[32,8,8,8],[40,0,8,8],[48,0,8,8],[40,8,8,8],[56,8,8,8]],
    body:    [[28,20,4,12],[16,20,4,12],[20,16,8,4],[28,16,8,4],[20,20,8,12],[32,20,8,12]],
    jacket:  [[28,36,4,12],[16,36,4,12],[20,32,8,4],[28,32,8,4],[20,36,8,12],[32,36,8,12]],
    // right arm  — side faces always 4 wide (arm depth), top/front/back use AW
    rarm:    [[44+AW,20,4,12],[40,20,4,12],[44,16,AW,4],[44+AW,16,AW,4],[44,20,AW,12],[44+AW+4,20,AW,12]],
    rsleeve: [[44+AW,36,4,12],[40,36,4,12],[44,32,AW,4],[44+AW,32,AW,4],[44,36,AW,12],[44+AW+4,36,AW,12]],
    // left arm
    larm:    [[36+AW,52,4,12],[32,52,4,12],[36,48,AW,4],[36+AW,48,AW,4],[36,52,AW,12],[36+AW+4,52,AW,12]],
    lsleeve: [[52+AW,52,4,12],[48,52,4,12],[52,48,AW,4],[52+AW,48,AW,4],[52,52,AW,12],[52+AW+4,52,AW,12]],
    rleg:    [[8,20,4,12],[0,20,4,12],[4,16,4,4],[8,16,4,4],[4,20,4,12],[12,20,4,12]],
    rpant:   [[8,36,4,12],[0,36,4,12],[4,32,4,4],[8,32,4,4],[4,36,4,12],[12,36,4,12]],
    lleg:    [[24,52,4,12],[16,52,4,12],[20,48,4,4],[24,48,4,4],[20,52,4,12],[28,52,4,12]],
    lpant:   [[8,52,4,12],[0,52,4,12],[4,48,4,4],[8,48,4,4],[4,52,4,12],[12,52,4,12]],
  };

  // ── Part builder ──────────────────────────────────────────────────
  function makePart(
    w: number, h: number, d: number,
    uvKey: string,
    mat: THREE.MeshLambertMaterial,
    overlayKey?: string,
    overlayMat?: THREE.MeshLambertMaterial,
    overlap = 0.5
  ): THREE.Group {
    const group = new THREE.Group();
    const geo = new THREE.BoxGeometry(w, h, d);
    applyFaceUVs(geo, UV[uvKey]);
    group.add(new THREE.Mesh(geo, mat));
    if (overlayKey && overlayMat) {
      const ogeo = new THREE.BoxGeometry(w + overlap, h + overlap, d + overlap);
      applyFaceUVs(ogeo, UV[overlayKey]);
      group.add(new THREE.Mesh(ogeo, overlayMat));
    }
    return group;
  }

  // ── Cape scale detection (standard / Optifine / HD) ───────────────
  function capeScale(h: number): number {
    if (h % 22 === 0) return h / 22;
    if (h % 17 === 0) return h / 17;
    if (h >= 32 && (h & (h - 1)) === 0) return h / 32;
    return Math.max(1, Math.floor(h / 22));
  }

  onMount(() => {
    // ── Renderer ─────────────────────────────────────────────────────
    const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setSize(width, height);
    renderer.setPixelRatio(window.devicePixelRatio);
    container.appendChild(renderer.domElement);

    // ── Scene / camera ───────────────────────────────────────────────
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(38, width / height, 0.1, 1000);
    camera.position.set(0, 0, 60);
    camera.lookAt(0, 0, 0); // model centered at origin

    // ── Lights ───────────────────────────────────────────────────────
    scene.add(new THREE.AmbientLight(0xffffff, 0.7));
    const dir = new THREE.DirectionalLight(0xffffff, 0.3);
    dir.position.set(0.678, 0.284, 0.678);
    scene.add(dir);

    // ── Skin texture ─────────────────────────────────────────────────
    const loader = new THREE.TextureLoader();
    const tex = loader.load(skinUrl);
    tex.magFilter = THREE.NearestFilter;
    tex.minFilter = THREE.NearestFilter;
    tex.colorSpace = THREE.SRGBColorSpace;

    const mat = new THREE.MeshLambertMaterial({ map: tex });
    const matOv = new THREE.MeshLambertMaterial({ map: tex, transparent: true, alphaTest: 0.1 });

    // ── Root pivot (model centered at origin) ─────────────────────────
    const root = new THREE.Group();
    scene.add(root);

    // Head — pivot at neck (y=12 in root), box centered at pivot
    // Character top: y=16, bottom: y=-16 → center at y=0
    const headPivot = new THREE.Group();
    const headMesh = makePart(8, 8, 8, 'head', mat, 'hat', matOv, 1.0);
    // No position offset — box is naturally centered at headPivot origin
    headPivot.add(headMesh);
    headPivot.position.set(0, 12, 0); // pivot = neck; head spans y=8..16
    root.add(headPivot);

    // Body — center at y=2, spans y=-4..8
    const body = makePart(8, 12, 4, 'body', mat, 'jacket', matOv, 0.5);
    body.position.set(0, 2, 0);
    root.add(body);

    // Arms — pivot at shoulder (y=8), mesh offset -6 → spans y=-4..8
    const armW = slim ? 3 : 4;
    const armOffX = slim ? 5.5 : 6;

    const rArmPivot = new THREE.Group();
    const rArmMesh = makePart(armW, 12, 4, 'rarm', mat, 'rsleeve', matOv, 0.5);
    rArmMesh.position.set(0, -6, 0);
    rArmPivot.add(rArmMesh);
    rArmPivot.position.set(-armOffX, 8, 0);
    root.add(rArmPivot);

    const lArmPivot = new THREE.Group();
    const lArmMesh = makePart(armW, 12, 4, 'larm', mat, 'lsleeve', matOv, 0.5);
    lArmMesh.position.set(0, -6, 0);
    lArmPivot.add(lArmMesh);
    lArmPivot.position.set(armOffX, 8, 0);
    root.add(lArmPivot);

    // Legs — pivot at hip (y=-4), mesh offset -6 → spans y=-16..-4
    const rLegPivot = new THREE.Group();
    const rLegMesh = makePart(4, 12, 4, 'rleg', mat, 'rpant', matOv, 0.5);
    rLegMesh.position.set(0, -6, 0);
    rLegPivot.add(rLegMesh);
    rLegPivot.position.set(-2, -4, 0);
    root.add(rLegPivot);

    const lLegPivot = new THREE.Group();
    const lLegMesh = makePart(4, 12, 4, 'lleg', mat, 'lpant', matOv, 0.5);
    lLegMesh.position.set(0, -6, 0);
    lLegPivot.add(lLegMesh);
    lLegPivot.position.set(2, -4, 0);
    root.add(lLegPivot);

    // ── Cape ─────────────────────────────────────────────────────────
    let capeGroup: THREE.Group | null = null;

    if (capeUrl) {
      const capeTex = loader.load(capeUrl);
      capeTex.magFilter = THREE.NearestFilter;
      capeTex.minFilter = THREE.NearestFilter;
      capeTex.colorSpace = THREE.SRGBColorSpace;

      const img = new Image();
      img.crossOrigin = 'anonymous';
      img.onload = () => {
        const cs = capeScale(img.naturalHeight);
        const tw = img.naturalWidth;
        const th = img.naturalHeight;

        // Cape geometry: 10×16×1, pivot at top edge (translate -8 down, +0.5 forward)
        const capeGeo = new THREE.BoxGeometry(10, 16, 1);
        capeGeo.translate(0, -8, 0.5);

        // UV from reference script.js (cs-scaled pixel coords)
        applyFaceUVs(capeGeo, [
          [11*cs, cs,   cs,    16*cs],  // +x right
          [0,     cs,   cs,    16*cs],  // -x left
          [cs,    0,    10*cs, cs   ],  // +y top
          [11*cs, cs,   10*cs, -cs  ],  // -y bottom (flip)
          [cs,    cs,   10*cs, 16*cs],  // +z front (outer face)
          [12*cs, cs,   10*cs, 16*cs],  // -z back (inner face)
        ], tw, th);

        const capeMat = new THREE.MeshLambertMaterial({
          map: capeTex,
          side: THREE.DoubleSide,
          transparent: true,
          alphaTest: 0.1,
        });

        capeGroup = new THREE.Group();
        capeGroup.position.set(0, 8, -2);
        capeGroup.rotation.y = Math.PI; // rotate 180° so front faces outward
        capeGroup.add(new THREE.Mesh(capeGeo, capeMat));
        root.add(capeGroup);
      };
      img.src = capeUrl;
    }

    // ── Drag rotation ─────────────────────────────────────────────────
    let theta = THREE.MathUtils.degToRad(30);
    let phi   = THREE.MathUtils.degToRad(21);
    let dragging = false;
    let lastX = 0, lastY = 0;

    function applyRotation() {
      root.rotation.y = theta;
      root.rotation.x = phi;
    }
    applyRotation();

    const canvas = renderer.domElement;

    const onMouseDown = (e: MouseEvent) => { dragging = true; lastX = e.clientX; lastY = e.clientY; };
    const onMouseUp   = () => { dragging = false; };
    const onMouseMove = (e: MouseEvent) => {
      if (!dragging) return;
      theta += (e.clientX - lastX) * 0.01;
      phi = THREE.MathUtils.clamp(phi + (e.clientY - lastY) * 0.01, -Math.PI / 2, Math.PI / 2);
      lastX = e.clientX; lastY = e.clientY;
      applyRotation();
    };

    canvas.addEventListener('mousedown', onMouseDown);
    window.addEventListener('mouseup',   onMouseUp);
    window.addEventListener('mousemove', onMouseMove);

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

    // ── Animation loop ────────────────────────────────────────────────
    let t = 0;
    function animate() {
      animId = requestAnimationFrame(animate);
      t += 360 / 1500;
      const rad = THREE.MathUtils.degToRad(t);

      rArmPivot.rotation.x = -THREE.MathUtils.degToRad(18) * Math.sin(rad);
      lArmPivot.rotation.x =  THREE.MathUtils.degToRad(18) * Math.sin(rad);
      rLegPivot.rotation.x =  THREE.MathUtils.degToRad(20) * Math.sin(rad);
      lLegPivot.rotation.x = -THREE.MathUtils.degToRad(20) * Math.sin(rad);

      if (capeGroup) {
        // Cape sways gently: base tilt 18° minus 6° wave on slower cycle
        capeGroup.rotation.x = THREE.MathUtils.degToRad(18) - THREE.MathUtils.degToRad(6) * Math.sin(rad / 4);
      }

      if (autoRotate) theta += 0.005;
      applyRotation();

      renderer.render(scene, camera);
    }
    animate();

    return () => {
      cancelAnimationFrame(animId);
      window.removeEventListener('mouseup',   onMouseUp);
      window.removeEventListener('mousemove', onMouseMove);
      renderer.dispose();
    };
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
