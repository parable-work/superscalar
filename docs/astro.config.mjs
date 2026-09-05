// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightLinksValidator from 'starlight-links-validator';

// Deployed to GitHub Pages from .github/workflows/release.yml (job docs-deploy)
// at https://parable-work.github.io/superscalar/. `base` must match the
// repository name; every in-content link is written with the base included.
export default defineConfig({
  site: 'https://parable-work.github.io',
  base: '/superscalar',
  integrations: [
    starlight({
      title: 'SuperScalar',
      description:
        'Cross-language scalar types: parse, normalize, validate. One Rust core; Go, Python, TypeScript and WASM bindings.',
      social: [
        {
          icon: 'github',
          label: 'GitHub',
          href: 'https://github.com/parable-work/superscalar',
        },
      ],
      // src/content/docs/404.md is the not-found page and is rendered by the
      // docs route; Starlight's own injected /404 route would conflict with it.
      disable404Route: true,
      editLink: {
        baseUrl: 'https://github.com/parable-work/superscalar/edit/main/docs/',
      },
      sidebar: [
        { label: 'Overview', link: '/' },
        { label: 'Install', items: [{ autogenerate: { directory: 'install' } }] },
        { label: 'Concepts', items: [{ autogenerate: { directory: 'concepts' } }] },
        { label: 'Guides', items: [{ autogenerate: { directory: 'guides' } }] },
        { label: 'Policy', items: [{ autogenerate: { directory: 'policy' } }] },
        // reference/ is written by `superscalar docs` in CI; only index.md is
        // committed. Autogeneration picks up whatever the generator produced.
        { label: 'Reference', items: [{ autogenerate: { directory: 'reference' } }] },
        { label: 'FAQ', slug: 'faq' },
      ],
      // Fails the build on a broken internal link or anchor.
      plugins: [starlightLinksValidator()],
    }),
  ],
});
