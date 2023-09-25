<script>
  import "iconify-icon";
  import Header from "$lib/component/Header.svelte";
  import Footer from "$lib/component/Footer.svelte";
  import BackToTop from "$lib/component/BackToTop.svelte";
  import { Svrollbar } from "svrollbar";
</script>

<div class="container">
  <Header />
  <slot />
  <Footer />
  <BackToTop />
  <Svrollbar />
</div>

<style scoped>
  .container {
    position: relative;

    /* --svrollbar-track-width: 2rem; */
    --svrollbar-thumb-width: 1rem;
    --svrollbar-thumb-background: #102f3d;
    --svrollbar-thumb-opacity: 0.6;
  }
</style>
