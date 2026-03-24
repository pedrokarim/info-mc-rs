<script lang="ts">
  interface Props {
    title: string;
    description: string;
    canonical?: string;
    ogImage?: string;
    ogType?: string;
    twitterCard?: 'summary' | 'summary_large_image';
    noindex?: boolean;
    jsonLd?: Record<string, unknown>;
    breadcrumbs?: { name: string; href: string }[];
  }

  const SITE_NAME = 'MCInfo';
  const SITE_URL = 'https://mcinfo.ascencia.re';
  const DEFAULT_OG_IMAGE = `${SITE_URL}/images/og-default.png`;

  let {
    title,
    description,
    canonical,
    ogImage,
    ogType = 'website',
    twitterCard = 'summary_large_image',
    noindex = false,
    jsonLd,
    breadcrumbs,
  }: Props = $props();

  const fullTitle = $derived(title ? `${title} | ${SITE_NAME}` : SITE_NAME);
  const canonicalUrl = $derived(canonical ? `${SITE_URL}${canonical}` : undefined);
  const ogImageUrl = $derived(ogImage ?? DEFAULT_OG_IMAGE);
  const breadcrumbLd = $derived(breadcrumbs ? {
    '@context': 'https://schema.org',
    '@type': 'BreadcrumbList',
    itemListElement: breadcrumbs.map((b, i) => ({
      '@type': 'ListItem',
      position: i + 1,
      name: b.name,
      item: `${SITE_URL}${b.href}`,
    })),
  } : undefined);
</script>

<svelte:head>
  <title>{fullTitle}</title>
  <meta name="description" content={description} />

  {#if noindex}
    <meta name="robots" content="noindex, nofollow" />
  {/if}

  {#if canonicalUrl}
    <link rel="canonical" href={canonicalUrl} />
  {/if}

  <!-- Open Graph -->
  <meta property="og:type" content={ogType} />
  <meta property="og:title" content={fullTitle} />
  <meta property="og:description" content={description} />
  <meta property="og:image" content={ogImageUrl} />
  <meta property="og:site_name" content={SITE_NAME} />
  {#if canonicalUrl}
    <meta property="og:url" content={canonicalUrl} />
  {/if}
  <meta property="og:locale" content="fr_FR" />

  <!-- Twitter Card -->
  <meta name="twitter:card" content={twitterCard} />
  <meta name="twitter:title" content={fullTitle} />
  <meta name="twitter:description" content={description} />
  <meta name="twitter:image" content={ogImageUrl} />

  <!-- JSON-LD Structured Data -->
  {#if jsonLd}
    {@html `<script type="application/ld+json">${JSON.stringify(jsonLd)}</script>`}
  {/if}
  {#if breadcrumbLd}
    {@html `<script type="application/ld+json">${JSON.stringify(breadcrumbLd)}</script>`}
  {/if}
</svelte:head>
