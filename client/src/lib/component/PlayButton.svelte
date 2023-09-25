<script>
  import {
    status,
    isPlaying,
    audioPlayer,
    index,
    trackList,
    addTrack,
  } from "$lib/js/stores.js";

  export let controls = false;
  export let track = false;
  export let title = "";
  export let artist = "";
  export let file = "";

  function playTrack() {
    $audioPlayer.play();
    $isPlaying = true;
  }

  function pauseTrack() {
    $audioPlayer.pause();
    $isPlaying = false;
  }

  function loadTrack($index) {
    title = $trackList[$index].title;
    artist = $trackList[$index].artist;
    $audioPlayer.src = $trackList[$index].file;
    $audioPlayer.load();
  }

  function addAndPlayTrack() {
    addTrack({ title, artist, file });
    $index = $trackList.length - 1;
    // 		$currentTime = 0;
    // Load and play the new track
    loadTrack($index);
    playTrack();
  }
</script>

{#if controls}
  {#if $isPlaying === false}
    <button class="play-button controls" on:click={playTrack}>
      <iconify-icon icon="ic:outline-play-arrow" />
    </button>
  {:else if $isPlaying === true && ($status === "waiting" || $status === "loading" || $status === "can play some" || $status === "can play all")}
    <button class="play-button controls" on:click={pauseTrack}>
      <iconify-icon icon="ic:round-replay" />
    </button>
  {:else if $isPlaying === true}
    <button class="play-button controls" on:click={pauseTrack}>
      <iconify-icon icon="ic:sharp-pause" />
    </button>
  {/if}
{:else if track}
  {#if title !== $trackList[$index].title}
    <button class="play-button track" on:click={addAndPlayTrack}>
      <iconify-icon icon="ic:outline-play-arrow" />
    </button>
  {:else if title === $trackList[$index].title && $isPlaying === true && ($status === "loading" || $status === "can play some" || $status === "can play all" || $status === "waiting")}
    <button class="play-button track playing" on:click={pauseTrack}>
      <iconify-icon icon="ic:round-replay" />
    </button>
  {:else if title === $trackList[$index].title && $isPlaying === true}
    <button class="play-button track playing" on:click={pauseTrack}>
      <iconify-icon icon="ic:sharp-pause" />
    </button>
  {:else if title === $trackList[$index].title && $isPlaying === false}
    <button class="play-button track playing" on:click={playTrack}>
      <iconify-icon icon="ic:outline-play-arrow" />
    </button>
  {/if}
{/if}

<style scoped>
  .play-button {
    height: 4.8rem;
    width: 4.8rem;
    border: none;
    border-radius: 50%;
    background-color: #0b7285;

    display: flex;
    justify-content: center;
    align-items: center;
  }

  iconify-icon {
    font-size: 3.6rem;
    color: #f1f3f5;
  }
</style>
