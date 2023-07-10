<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let output: string;
  let source: string;

  async function run(input: string) {
    output = await invoke("run", { input })
  }
</script>

<h1 class="h1 text-center mt-8">Python editor</h1>

<div class="w-96 h-96 mx-auto mt-8">
  <textarea class="textarea w-full h-full resize-none" bind:value={source}></textarea>

  <button
    on:click={() => run(source)}
    disabled={source === undefined}
    class="btn variant-filled-primary w-full mt-4">
    Run code!
  </button>
  
  {#if output}
	<pre class="pre mt-4">{output}</pre>
  {/if}
</div>
