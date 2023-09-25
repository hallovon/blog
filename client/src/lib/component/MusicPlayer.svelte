<script>
  import { onMount } from "svelte";
  import Controls from "./Controls.svelte";

  const getPlayList = async (id) => {
    return await fetch(`http://music.163.com/api/playlist/detail?id=${id}`)
      .then((resp) => {
        if (resp.ok) {
          return resp.json();
        }
      })
      .catch((err) => {
        console.warn(err);
      });
  };

  const getLyric = async (id) => {
    return fetch(
      `http://music.163.com/api/song/lyric?os=pc&id=${id}&lv=-1&kv=-1&tv=-1`
    )
      .then((resp) => {
        if (resp.ok) {
          return resp.json();
        }
      })
      .catch((err) => {
        console.warn(err);
      });
  };

  let playList;

  onMount(() => {
    try {
      const data = getPlayList("2119983629");
      if (data.code === 200) {
        playList = data.result.tracks.map((item) => ({
          id: item.id,
          name: item.name,
          artist: item.artists.map((el) => el.name).join(","), //由于歌手是一个数组，这里我们把它转换成字符串拼接
          url: `https://music.163.com/song/media/outer/url?id=${item.id}.mp3`, //歌曲地址
          cover: item.album.picUrl.replace(/http:/, "https:"),
          lrc: null,
        }));
      }
    } catch (err) {
      console.warn(err);
    }

    try {
      const lyric = getLyric(id);
      ctx.body = lyric.lrc.lyric; //返回指定部分
    } catch (error) {
      ctx.body = "";
    }

    console.log();
  });
</script>

<div class="app">
  <Controls />
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 1rem;
  }

  :global(p) {
    font-size: 12px;
  }

  :global(strong) {
    font-size: 14px;
  }

  :global(button) {
    font-size: 14px;
  }

  .app {
    margin: 0;
    padding: 0;
    display: grid;
    row-gap: 1rem;
  }
</style>
