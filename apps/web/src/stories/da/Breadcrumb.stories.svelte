<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Breadcrumb from '$lib/components/ui/Breadcrumb.svelte';

  const { Story } = defineMeta({
    title: 'DA/Navigation/Breadcrumb',
    component: Breadcrumb,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      separator: { control: 'text' }
    },
    args: { separator: '/' }
  });
</script>

<Story name="Default" args={{
  items: [
    { label: 'Accueil', href: '/' },
    { label: 'Serveurs', href: '/servers' },
    { label: 'play.hypixel.net' }
  ]
}} />

<Story name="Player Path" args={{
  items: [
    { label: 'Accueil', href: '/' },
    { label: 'Skins', href: '/skins' },
    { label: 'Notch' }
  ]
}} />

<Story name="Custom Separator" args={{
  items: [
    { label: 'Admin', href: '/admin' },
    { label: 'Config', href: '/admin/config' },
    { label: 'Rate Limit' }
  ],
  separator: '›'
}} />
