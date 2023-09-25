<script>
  let visible = true;

  const handleOnScroll = () => {
    let scrollTop = Math.max(
      document.body.scrollTop,
      document.documentElement.scrollTop
    );
    window.onscroll = () => {
      if (scrollTop >= window.screen.height) {
        visible = false;
      } else {
        visible = true;
      }
    };
  };
</script>

<svelte:window on:scroll={handleOnScroll} />

<!-- {#if visible}
  
{/if} -->

<header id="header">
  <h3><a href="/" class="item">hallovon</a></h3>
  <ul class="list">
    <li>
      <a href="/" class="item">
        <iconify-icon icon="ic:outline-home" />
        <span>首页</span>
      </a>
    </li>
    <li>
      <a href="/" class="item">
        <iconify-icon icon="ic:outline-archive" />
        <span>归档</span>
      </a>
    </li>
    <li>
      <a href="/" class="item">
        <iconify-icon icon="ic:outline-bookmarks" />
        <span>标签</span>
      </a>
    </li>
    <li>
      <a href="/" class="item">
        <iconify-icon icon="ic:outline-account-circle" />
        <span>关于</span>
      </a>
    </li>
    <li>
      <a href="/" class="item">
        <iconify-icon icon="ic:baseline-edit" />
        <span>留言</span>
      </a>
    </li>
  </ul>
</header>

<style scoped>
  #header {
    color: #f1f3f5;
    font-size: 1.8rem;
    width: 100%;
    height: 7.2rem;
    padding: 0 4.8rem;

    position: fixed;
    top: 0;
    width: 100%;
    backdrop-filter: blur(5px);

    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  ul {
    display: flex;
    justify-content: space-around;
  }
  ul li {
    margin: auto 1.6rem;
    list-style-type: none;
  }
  a {
    text-decoration: none;
  }

  .item:link,
  .item:visited {
    color: #f1f3f5;
  }
  .item:hover,
  .item:active {
    color: coral;
  }

  .item {
    width: 110%;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  iconify-icon {
    /* box-sizing: none;
    padding: 0;
    margin: 0; */
    width: 1.8rem;
    height: 1.8rem;
  }
</style>
