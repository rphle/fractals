import adapter from '@sveltejs/adapter-static';

const isPages = Boolean(process.env.IS_PAGES);

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter(), paths: {
      base: isPages ? '/fractals' : ''
    }
  }
};

export default config;
