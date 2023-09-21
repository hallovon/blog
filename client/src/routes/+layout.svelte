<script>
    import Header from "$lib/component/header.svelte";
    import Footer from "$lib/component/footer.svelte";
    import BackToTop from "../lib/component/back_to_top.svelte";
</script>

<div class="container">
    <Header />
    <slot />
    <div class="long-div"></div>
    <Footer />
    <BackToTop />
</div>

<style scoped>
    .container {
        position: relative;
    }

    .long-div {
        height: 100vh;
        width: 100%;
    }
</style>
