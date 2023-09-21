<script>
  const showOnPx = 100;
  let hidden = true;

  // const cubic = (val) => Math.pow(val, 3);
  // const easeInOutCubic = (val) =>
  //   val < 0.5 ? cubic(val * 2) / 2 : 1 - cubic((1 - val) * 2) / 2;

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

<button class="back-to-top" on:click={goTop} class:hidden>
  <ion-icon name="arrow-up-outline" />
</button>

<style>
  .back-to-top {
    font-size: 2.4rem;
    width: 3.6rem;
    height: 3.6rem;
    /* background-color: #0c8599; */
    background-color: #0b7285;
    color: #f1f3f5;
    border: none;
    border-radius: 50%;

    position: fixed;
    right: 1rem;
    bottom: 2rem;
  }

  .hidden {
    display: none;
  }
</style>
