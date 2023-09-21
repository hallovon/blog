<script>
    import Header from "./header.svelte";
    import Footer from "./footer.svelte";
    import Aside from "./aside.svelte";
</script>

<Header />
<main class="main">
    <slot />
</main>
<Footer />
<Aside />

<style>
    .main {
        width: 55%;
        background-color: bisque;
        margin: 2rem auto;
    }
</style>
