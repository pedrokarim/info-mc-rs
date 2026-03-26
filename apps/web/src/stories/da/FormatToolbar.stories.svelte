<script module lang="ts">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import FormatToolbar from '$lib/components/ui/FormatToolbar.svelte';

  const { Story } = defineMeta({
    title: 'DA/Input/FormatToolbar',
    component: FormatToolbar,
    tags: ['autodocs'],
    parameters: { layout: 'centered' },
    args: {
      edition: 'java',
      activeFormats: { bold: false, italic: false, underlined: false, strikethrough: false, obfuscated: false },
    },
  });
</script>

<Story name="Default" />
<Story name="Bold + Italic Active" args={{ activeFormats: { bold: true, italic: true, underlined: false, strikethrough: false, obfuscated: false } }} />
<Story name="Bedrock Edition" args={{ edition: 'bedrock' }} />
