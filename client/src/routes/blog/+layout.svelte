<script>
  import MusicPlayer from "$lib/component/MusicPlayer.svelte";
</script>

<div class="article-wrapper">
  <slot />
</div>
<div class="music-player">
  <MusicPlayer />
</div>

<style>
  .music-player {
    position: fixed;
    left: 2.4rem;
    bottom: 2.4rem;
    /* width: 8.4rem; */
    height: 16rem;
  }

  .article-wrapper {
    width: 55%;
    margin: 8.4rem auto 4.2rem;
  }
</style>
