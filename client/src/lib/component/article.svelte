<script>
    export let article;
</script>

<article class="article">
    <h2 class="article_title">{@html article.title}</h2>
    <div class="article_description">
        <span>{article.topic}</span>
        <span>{new Date(article.create_time).toLocaleDateString()}</span>
    </div>
    <div>
        {@html article.article_content}
    </div>
</article>

<style>
    .article {
        font-size: 1.8rem;
        line-height: 1.6;
    }

    .article_title {
        margin: 7.2rem auto 2.4rem;
        font-size: 4.2rem;
        text-align: center;
    }

    .article_description {
        font-size: 1.4rem;
        display: flex;
        width: 20%;
        justify-content: space-between;
        margin: 2.4rem auto;
    }
</style>
