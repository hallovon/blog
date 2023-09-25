<script>
  import Slider from "./Slider.svelte";
  import {
    status,
    isPlaying,
    audioPlayer,
    index,
    trackList,
  } from "$lib/js/stores.js";
  import { format } from "../js/utility";
  import PlayButton from "./PlayButton.svelte";
  import { onMount } from "svelte";

  let duration = 0;
  let currentTime = 0;
  let formattedTime = format(currentTime);
  let paused = true;
  let volume = 0.5;

  let slider;
  let rAF = null;

  let title = $trackList[$index].title;
  let artist = $trackList[$index].artist;
  let src = $trackList[$index].file;

  function whilePlaying() {
    slider.value = audio.currentTime;
    currentTime = slider.value;
    rAF = requestAnimationFrame(whilePlaying);
  }

  function loadTrack($index) {
    title = $trackList[$index].title;
    artist = $trackList[$index].artist;
    $audioPlayer.src = $trackList[$index].file;
    $audioPlayer.load();
  }

  function playTrack() {
    $isPlaying = true;
    requestAnimationFrame(whilePlaying);
    $audioPlayer.play();
  }

  function pauseTrack() {
    $isPlaying = false;
    cancelAnimationFrame(rAF);
    $audioPlayer.pause();
  }

  function movePosition() {
    time = slider.value;
    if (!audio.paused) {
      cancelAnimationFrame(rAF);
    }
  }

  function updatePosition() {
    audio.currentTime = slider.value;
    if (!audio.paused) {
      requestAnimationFrame(whilePlaying);
    }
  }

  function previousTrack() {
    $isPlaying = false;
    currentTime = 0;
    if ($index > 0) {
      $index -= 1;
    } else {
      $index = $trackList.length - 1;
    }
    loadTrack($index);
    playTrack();
  }

  function nextTrack() {
    $isPlaying = false;
    currentTime = 0;
    if ($index < $trackList.length - 1) {
      $index += 1;
    } else {
      $index = 0;
    }
    loadTrack($index);
    playTrack();
  }

  onMount(() => {
    $audioPlayer.load();
  });
</script>

<!-- svelte-ignore a11y-media-has-caption -->
<!-- <audio
  bind:this={$audioPlayer}
  bind:duration
  bind:currentTime
  bind:paused
  bind:volume
  on:canplay={() => ($status = "can play some")}
  on:canplaythrough={() => ($status = "can play all")}
  on:waiting={() => ($status = "waiting")}
  on:timeupdate={() => ($status = "playing")}
  on:seeking={() => ($status = "seeking")}
  on:ended={() => {
    $isPlaying = false;
    currentTime = 0;
  }}
  {src}
/> -->

<div class="box">
  <div class="info">
    {title}
  </div>

  <div class="buttons">
    <button class="prev" on:click={previousTrack}>
      <iconify-icon icon="ic:outline-skip-previous" />
    </button>

    <PlayButton controls />

    <button class="next" on:click={nextTrack}>
      <iconify-icon icon="ic:outline-skip-next" />
    </button>
  </div>

  <div class="progress">
    <span class="time">{format(currentTime)}</span>

    <div class="progress-slider">
      <Slider
        bind:this={slider}
        min={0}
        bind:value={currentTime}
        max={duration}
        step={0.01}
        precision={2}
        formatter={(v) => format(v)}
        on:input={movePosition}
        on:change={updatePosition}
      />
    </div>

    <span class="duration">{format(duration)}</span>
  </div>
</div>

<style>
  button {
    border-radius: 50%;
    border: none;
    width: 3.2rem;
    height: 3.2rem;
    background-color: #238091;

    display: flex;
    justify-content: center;
    align-items: center;
  }

  .box {
    border: 2px solid #15aabf;
    border-radius: 5px;
    padding: 1.2rem 1.2rem;

    display: flex;
    flex-direction: column;
    row-gap: 1.2rem;
    justify-content: center;
  }

  .buttons {
    margin: 0 1.2rem;

    display: flex;
    justify-content: space-around;
    align-items: center;
  }

  .prev,
  .next {
    font-size: 2.4rem;
    color: #f1f3f5;
  }

  .progress {
    display: flex;
    column-gap: 1.2rem;
  }
</style>
