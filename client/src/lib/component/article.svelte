<script>
    export let article;
</script>

<article>
    <h1 class="title">{article.title}</h1>
    <div class="description">
        <span>{article.topic}</span>
        <span>{new Date(article.create_time).toLocaleDateString()}</span>
    </div>
    <div>
        {@html article.article_content}
    </div>
</article>

<style>
    article {
        width: 60%;
        margin: 8.4rem auto 4.2rem;
        font-size: 2rem;
        line-height: 1.6;
        /* background-color: blueviolet; */
    }

    .title {
        margin: 9.6rem auto 2.4rem;
        font-size: 4.2rem;
        text-align: center;
    }

    .description {
        font-size: 1.4rem;
        display: flex;
        width: 20%;
        justify-content: space-between;
        margin: 2.4rem auto;
    }
</style>
