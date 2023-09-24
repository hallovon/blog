<script>
    import Article from "$lib/component/article.svelte";
    import { onMount } from "svelte";

    export let data;
    let article;
    onMount(() => {
        article = data.data;
    });
</script>

<Article {article} />
