<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import SkinViewer3D from '$lib/components/ui/SkinViewer3D.svelte';

  const STEVE_SKIN = 'https://minotar.net/skin/Notch';
  const ALEX_SKIN = 'https://minotar.net/skin/Dinnerbone';
  const CAPE_URL = 'https://minotar.net/skin/Notch'; // fallback — real cape URLs are session-bound

  const { Story } = defineMeta({
    title: 'DA/3D/SkinViewer3D',
    component: SkinViewer3D,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      skinUrl: { control: 'text' },
      capeUrl: { control: 'text' },
      slim: { control: 'boolean' },
      width: { control: { type: 'range', min: 120, max: 480, step: 10 } },
      height: { control: { type: 'range', min: 180, max: 600, step: 10 } },
      autoRotate: { control: 'boolean' },
      paused: { control: 'boolean' },
      backEquipment: {
        control: { type: 'inline-radio' },
        options: ['cape', 'elytra', 'none']
      }
    },
    args: {
      skinUrl: STEVE_SKIN,
      capeUrl: undefined,
      slim: false,
      width: 240,
      height: 360,
      autoRotate: false,
      paused: false,
      backEquipment: 'cape'
    }
  });
</script>

<Story name="Default (Steve)" />

<Story name="Slim (Alex)" args={{ skinUrl: ALEX_SKIN, slim: true }} />

<Story name="Auto Rotate" args={{ autoRotate: true }} />

<Story name="Paused" args={{ paused: true }} />

<Story name="Large" args={{ width: 400, height: 560 }} />

<Story name="With Cape" args={{ capeUrl: CAPE_URL, backEquipment: 'cape' }} />

<Story name="Elytra" args={{ capeUrl: CAPE_URL, backEquipment: 'elytra' }} />
