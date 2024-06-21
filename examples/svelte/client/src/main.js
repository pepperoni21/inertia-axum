import './app.css'
import { createInertiaApp } from '@inertiajs/svelte'

// @ts-ignore
createInertiaApp({
  resolve: name => {
    // @ts-ignore
    const pages = import.meta.glob('./pages/**/*.svelte', { eager: true })
    return pages[`./pages/${name}.svelte`]
  },
  setup({ el, App, props }) {
    new App({ target: el, props })
  },
})