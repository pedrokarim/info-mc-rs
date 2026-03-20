<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Avatar from '$lib/components/ui/Avatar.svelte';

  const { Story } = defineMeta({
    title: 'DA/Data/Avatar',
    component: Avatar,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    argTypes: {
      src: { control: 'text' },
      alt: { control: 'text' },
      fallback: { control: 'text' },
      size: {
        control: { type: 'inline-radio' },
        options: ['sm', 'md', 'lg']
      }
    },
    args: { src: 'https://mc-heads.net/avatar/Notch/64', alt: 'Notch', size: 'md' }
  });
</script>

<Story name="With Image" args={{ src: 'https://mc-heads.net/avatar/Notch/64', alt: 'Notch' }} />
<Story name="Fallback Initials" args={{ src: '', fallback: 'Steve', size: 'md' }} />
<Story name="Small" args={{ src: 'https://mc-heads.net/avatar/jeb_/64', alt: 'jeb_', size: 'sm' }} />
<Story name="Large" args={{ src: 'https://mc-heads.net/avatar/Dinnerbone/64', alt: 'Dinnerbone', size: 'lg' }} />

<Story name="All Sizes">
  <div style="display: flex; gap: 0.8rem; align-items: center;">
    <Avatar src="https://mc-heads.net/avatar/Notch/64" alt="Notch" size="sm" />
    <Avatar src="https://mc-heads.net/avatar/Notch/64" alt="Notch" size="md" />
    <Avatar src="https://mc-heads.net/avatar/Notch/64" alt="Notch" size="lg" />
  </div>
</Story>

<Story name="Broken Image" args={{ src: 'https://invalid.example/nope.png', fallback: 'Err', size: 'md' }} />
