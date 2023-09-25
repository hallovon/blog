<script>
  const showOnPx = 100;
  let hidden = true;

  const goTop = () => {
    document.body.scrollIntoView();
  };
  const scrollContainer = () => {
    return document.documentElement || document.body;
  };

  const handleOnScroll = () => {
    if (!scrollContainer()) {
      return;
    }

    if (scrollContainer().scrollTop > showOnPx) {
      hidden = false;
    } else {
      hidden = true;
    }
  };
</script>

<svelte:window on:scroll={handleOnScroll} />

{#if !hidden}
  <button class="back-to-top" on:click={goTop}>
    <iconify-icon icon="ic:outline-keyboard-double-arrow-up"/>
  </button>
{/if}

<style scoped>
  .back-to-top {
    font-size: 2.4rem;
    width: 3.6rem;
    height: 3.6rem;
    background-color: #0b7285;
    color: #f1f3f5;
    border: none;
    border-radius: 50%;

    position: fixed;
    right: 1rem;
    bottom: 2rem;
  }
</style>
