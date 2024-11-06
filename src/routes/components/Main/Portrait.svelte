<script lang="ts">
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import { join, desktopDir, extname } from '@tauri-apps/api/path';
  import { exists, readFile, create, remove, copyFile } from '@tauri-apps/plugin-fs';
  import { save, open, confirm, message } from '@tauri-apps/plugin-dialog';
  import { onMount } from 'svelte';
  import MdDelete from 'svelte-icons/md/MdDelete.svelte'
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
      filters: [{
        name: 'PNG',
        extensions: ['png'],
      }],
    });

    console.log(await extname(path as string))
    if(path != null && await extname(path as string) != "png"){
      await message($t("common.invalidImage"))
      return
    }

    if(path != null){
      const modImageFile = await join(modPath as string, imageName as string)
      const originImageFile  = await join(originPath as string, imageName as string)
      console.log(modImageFile)
      console.log(originImageFile)

      await copyFile(path, modImageFile)
      await invoke("resize_image", {modImage: modImageFile, originImage: originImageFile})

      replaced = true
      hiddenMenu = !hiddenMenu
      initImage()
    }
  }

  const deleteImage = async() => {
    const modImagePath = await join(modPath as string, imageName as string)
    if(await exists(modImagePath)){
      const result = await confirm($t("common.confirmDelete"))
      if(result){
        await remove(await join(modPath as string, imageName as string))
        initImage()
        hiddenMenu = !hiddenMenu
      }
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
      <button class="btn variant-glass mt-1 w-24 mx-3" onclick={() => {download()}}>{$t("common.download")}</button>
      <button class="btn variant-glass mt-2 w-24 mx-3" onclick={() => {upload()}}>{$t("common.upload")}</button>
      <div class="flex flex-row mx-auto columns-2 mt-5">
        <button class="btn variant-outline-primary w-10" onclick={() => {hiddenMenu = !hiddenMenu}}>✖</button>
        {#if replaced}
          <button class="btn variant-filled-error ml-4 w-32" onclick={() => {deleteImage()}}><MdDelete/></button>
        {/if}
      </div>
    </div>
  {/if}
</div>