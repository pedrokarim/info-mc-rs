<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import HeroBleed from '$lib/components/ui/HeroBleed.svelte';

  const { Story } = defineMeta({
    title: 'DA/Sections/HeroBleed',
    component: HeroBleed,
    tags: ['autodocs'],
    parameters: {
      layout: 'fullscreen'
    },
    argTypes: {
      eyebrow: { control: 'text' },
      title: { control: 'text' },
      description: { control: 'text' },
      image: { control: 'text' },
      primaryLabel: { control: 'text' },
      primaryHref: { control: 'text' },
      secondaryLabel: { control: 'text' },
      secondaryHref: { control: 'text' }
    },
    args: {
      eyebrow: 'Minecraft Data Hub',
      title: 'Un front qui ressemble enfin a un vrai site de jeu.',
      description:
        'Hero pleine largeur, une seule boite de description et des CTA arcade lisibles.',
      image: '/images/hero/hero-main-v01.png',
      primaryLabel: 'Voir un serveur',
      primaryHref: '/server/play.hypixel.net',
      secondaryLabel: 'Explorer les skins',
      secondaryHref: '/skins'
    }
  });
</script>

<Story name="Homepage Hero" />

<Story
  name="Server Hero"
  args={{
    eyebrow: 'Server Intelligence',
    title: 'Snapshot serveur live avec MOTD visuel.',
    description:
      'Adresse, status online/offline, version, joueurs, et rendu du MOTD dans une seule fiche.',
    image: '/images/servers/server-card-bg-v01.png',
    primaryLabel: 'Tester play.hypixel.net',
    primaryHref: '/server/play.hypixel.net',
    secondaryLabel: 'Mode auto',
    secondaryHref: '/server'
  }}
/>

<Story
  name="Skins Hero"
  args={{
    eyebrow: 'Skin Explorer',
    title: 'Recherche skins a la NameMC, details inclus.',
    description:
      'Trouve un pseudo, ouvre la fiche skin, compare head/full render et mode Classic/Slim.',
    image: '/images/skins/skin-gallery-bg-v01.png',
    primaryLabel: 'Ouvrir la galerie',
    primaryHref: '/skins',
    secondaryLabel: 'Chercher un joueur',
    secondaryHref: '/player/Notch'
  }}
/>
