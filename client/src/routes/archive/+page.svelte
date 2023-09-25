<script>
  export let data;

  const displayDate = (date) => {
    date = date.substring(0, 7);
    let arr = date.split("-");
    return arr[0] + "年" + arr[1] + "月";
  };
</script>

<div class="container">
  {#each Object.entries(data) as [date, articles]}
    <div class="article-wrapper">
      <h3 class="date">{displayDate(date)}</h3>
      {#each articles as article}
        <p class="title">
          <a href="/blog/{article.article_id}">{article.title}</a>
        </p>
      {/each}
    </div>
  {/each}
</div>

<style>
  .container {
    margin: 16rem auto 4.8rem;
    width: 55%;
    background-color: burlywood;
  }

  .article-wrapper {
    margin: 2.4rem auto;
  }

  .date {
    font-size: 3.2rem;
    margin-bottom: 1.8rem;
  }

  .title {
    font-size: 2.4rem;
    margin-bottom: 0.6rem;
  }
</style>
