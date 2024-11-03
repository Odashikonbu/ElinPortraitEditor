<script>
  import { AppBar } from '@skeletonlabs/skeleton';
  import { open } from '@tauri-apps/plugin-dialog'
  import { homeDir } from '@tauri-apps/api/path';
  import { createEventDispatcher } from 'svelte';
  import { t } from '$lib/translations/translations';
  import {langs} from '../config'

  const {gamePath, currentLang} = $props()

  const dispatch = createEventDispatcher();
  const languages = langs()

  let selectedLang = $state(currentLang)

  const selectFolder = async() => {
    const selected = await open({multiple: false, directory: true, defaultPath: await homeDir()})
    if(selected != null){
      dispatch('selectFolder', {path: selected})
    }
  }
</script>

<AppBar class="bg-primary-900 h-25 pt-5 w-full" slotTrail="place-content-end">
  <h1 class="text-lg font-extrabold font-serif">{$t('common.tooltitle')}</h1>
  <select class="select mt-4 w-20 h-10" bind:value={selectedLang} onchange={() => {dispatch("changeLang", {selectedLang: selectedLang})}} >
    {#each languages as language}
      <option value={language} selected={currentLang == language}>{language}</option>
    {/each}
  </select>
  <svelte:fragment slot="trail">
    <div class="my-auto">
      <button type="button" class="btn variant-filled" onclick={selectFolder}>
        {$t('common.selectfolder')}
      </button>
      <p class="truncate w-[200px] text-sm">{gamePath}</p>
    </div>
  </svelte:fragment>
</AppBar>