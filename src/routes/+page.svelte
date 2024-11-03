<script lang="ts">
  import { onMount } from "svelte";
  import { exists, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
  import { resourceDir } from "@tauri-apps/api/path";
  import { message } from "@tauri-apps/plugin-dialog";
  import { load } from "js-yaml";
  import { loadTranslations } from "$lib/translations/translations";

  import { type config } from "./config";
  import "./config";
  import { t } from "$lib/translations/translations";

  import Header from "./components/Header.svelte";
  import Main from "./components/Main.svelte";


  let gamePath: string = "";
  let currentLang = "";

  const getConfigPath = async () => {
    return (await resourceDir()) + "\\config.yml";
  };

  const updateYml = async () => {
    await writeTextFile(
      await getConfigPath(),
      `language: '${currentLang}'\ngame_path: '${gamePath}'`
    );
  };

  const checkGamePath = async () => {
    if (gamePath != "" && !(await exists(gamePath + "\\Elin.exe"))) {
      message($t("common.invalidgamepath"));
      gamePath = "";
    }
    await updateYml()
  };

  onMount(async () => {
    let configPath = await getConfigPath();
    if (!(await exists(configPath))) {
      await message($t("common.ymlnotfound"));
      await writeTextFile(configPath, "language: 'ja'\ngame_path: ''");
      await message($t("common.ymlcreated"));
    }

    const configFile = await readTextFile(configPath);
    const config: config = load(configFile) as config;

    gamePath = config.game_path;
    currentLang = config.language;
    await checkGamePath();
    loadTranslations(currentLang, "/");
  });
</script>

<Header
  {gamePath}
  {currentLang}
  on:changeLang={(event) => {
    currentLang = event.detail.selectedLang;
    loadTranslations(currentLang, "/");
    updateYml()
  }}
  on:selectFolder={(event) => {
    gamePath = event.detail.path;
    checkGamePath();
  }}
/>
{#if gamePath != ''}
  <Main gamePath={gamePath}/>
{/if}