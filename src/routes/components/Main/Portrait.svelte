<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { join, desktopDir } from '@tauri-apps/api/path';
  import { exists, readFile, create } from '@tauri-apps/plugin-fs';
  import { save, open } from '@tauri-apps/plugin-dialog';
  import { onMount } from 'svelte';
  import { t } from "$lib/translations/translations";

  const {imageName, modPath, originPath} = $props()
 
  
  let ImageURL:string = $state("")
  let replaced:boolean = $state(false)
  let hiddenMenu:boolean = $state(true)

  const initImage = async() => {
    ImageURL = await convertFileSrc(await join(originPath as string, imageName as string))
    const modImagePath = await join(modPath as string, imageName as string)
    if(await exists(modImagePath)){
      ImageURL = await convertFileSrc(modImagePath)
      replaced = true
    }else{
      replaced = false
    }
  }
  onMount(() => initImage())

  const download = async() => {
    const binary = await readFile(await join(!replaced ? originPath as string : modPath as string, imageName as string))
    const path = await save({
      defaultPath: await join(await desktopDir(), imageName as string),
      filters: [
        {
          name: 'Download Image',
          extensions: ['png', 'jpeg'],
        },
      ],
    })
    if(path != null){
      const file = await create(path)
      file.write(binary)
      hiddenMenu = !hiddenMenu
    }
  }

  const upload = async() => {
    const path = await open({
      multiple: false,
      directory: false,
    });

    if(path != null){
      const binary = await readFile(path)
      const file = await create(await join(modPath as string, imageName as string))

      await file.write(binary)
      replaced = true
      hiddenMenu = !hiddenMenu
      initImage()
    }

  }


</script>
<div class="card w-[120px] h-[160px] mb-5">
  {#if hiddenMenu}
    <button onclick={() => {hiddenMenu = !hiddenMenu}}>
      <img src={ImageURL} alt="img" class={replaced ? "border border-yellow-300" : "border border-black"}/>
      <h2>{imageName}</h2>
    </button>
  {:else}
    <div class={hiddenMenu ? "hidden" : "mx-auto"}>
      <button class="btn variant-glass mt-4 w-24 mx-3" onclick={() => {download()}}>{$t("common.download")}</button>
      <button class="btn variant-glass mt-2 w-24 mx-3" onclick={() => {upload()}}>{$t("common.upload")}</button>
      <button class="btn variant-outline-primary mt-6 w-24 h-5 mx-3" onclick={() => {hiddenMenu = !hiddenMenu}}>✖</button>
    </div>
  {/if}
</div>