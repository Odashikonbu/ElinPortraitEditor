<script lang="ts">
  import { onMount } from "svelte";
  import { exists, writeTextFile, mkdir, readDir } from "@tauri-apps/plugin-fs";
  import { join } from '@tauri-apps/api/path';
  import { invoke } from '@tauri-apps/api/core';
  import { getXml } from "../config";
  import { t } from "$lib/translations/translations";
  import Portrait from "./Main/Portrait.svelte";

  const { gamePath } = $props();
  let modPath:string = $state("")
  let modPortraitPath:string = $state("")
  let originPath:string = $state("")

  // svelte-ignore non_reactive_update
  let fileList:String[] = $state([])

  const updateFileList = async() => {
    if(await exists(originPath)){
      fileList = (await readDir(originPath)).map(v => v.name) as String[]
    }
  }

  const openFolder = async() => {
    console.log(modPortraitPath+"\\")
    await invoke("open_folder", {path:modPortraitPath+"\\_mod_folder"})
  }

  onMount(async()=>{
    if(gamePath != ''){
      modPath= await join(gamePath as string,"Package","Mod_Portrait")
      modPortraitPath = await join(modPath,"Portrait")
      originPath= await join(gamePath as string,"Package","_Elona","Portrait")
      if (!(await exists(modPath))) {
        await mkdir(modPath)
      }
      if (!(await exists(modPortraitPath))) {
        await mkdir(modPortraitPath)
      }
      if (!(await exists(modPath + "\\package.xml"))) {
        await writeTextFile(modPath + "\\package.xml", getXml())
      }

      await writeTextFile(modPortraitPath + "\\_mod_folder", '')
      await updateFileList();
    }
  })
</script>

<h1 class="text-xl text-gray-300 mx-auto text-center my-5">
  {$t("common.Portraits")}
  <button class="btn variant-outline-primary ml-3" onclick={() => {openFolder()}}>
    {$t("common.openFolder")}
  </button>
</h1>
<section class="mx-auto container flex columns-12 flex-wrap gap-2">
  {#await onMount}
    <p>ファイル読み込み中</p>
  {:then}
    {#each fileList as file}
      <Portrait imageName={file} modPath={modPortraitPath} originPath={originPath}/>
    {/each}    
  {/await}
</section>
