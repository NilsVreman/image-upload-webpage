<template>
  <HamburgerMenu />

  <section class="entertainment-page">
    <h1>🎉 More Fun & Games</h1>

    <h2>🎲 Games</h2>
    <article
      v-for="game in gameList"
      :key="game.title"
      class="card"
    >
      <h3>{{ game.title }}</h3>
      <p>{{ game.summary }}</p>
      <ul>
        <li
          v-for="rule in game.rules"
          :key="rule"
        >
          {{ rule }}
        </li>
      </ul>
    </article>

    <h2>🎶 Songs</h2>

    <h3>🇬🇧 English crowd-pleasers</h3>
    <article
      v-for="song in songList.filter(s => s.lang === 'en')"
      :key="song.title"
      class="card"
    >
      <h4>{{ song.title }}</h4>
      <p><strong>Melody:</strong> {{ song.melody }}</p>
      <pre class="lyrics">{{ song.text.join("\n") }}</pre>
    </article>

    <h3>🇸🇪 Svenska klassiker</h3>
    <article
      v-for="song in songList.filter(s => s.lang === 'sv')"
      :key="song.title"
      class="card"
    >
      <h4>{{ song.title }}</h4>
      <p><strong>Melodi:</strong> {{ song.melody }}</p>
      <pre class="lyrics">{{ song.text.join("\n") }}</pre>
    </article>
  </section>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import games from "@/data/games.json";
import songs from "@/data/songs.json";

interface Game {
  title: string;
  summary: string;
  rules: string[];
}

interface Song {
  title: string;
  melody: string;
  lang: "en" | "sv";
  text: string[];
}

const gameList = games as Game[];
const songList = songs as Song[];

onMounted(() => window.scrollTo({ top: 0, behavior: "smooth" }));
</script>

<style scoped>
.entertainment-page {
  max-width: 65ch;
  margin: 0 auto;
  padding: 1rem 1rem;
  line-height: 1.5;
}
.card h3 {
  margin: 0 0 0.5rem;
}
.lyrics {
  white-space: pre-wrap;
  font-family: inherit;
  padding: 0.25em 0;
}

ul li {
  display: list-item;
  padding: 0.4rem;
  text-align: left;
}
p {
  font-style: italic;
}
</style>
