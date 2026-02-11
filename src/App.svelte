<script lang="ts">
  import wasmInit, { generate, init, prepare} from "qart"
  import { onMount } from "svelte";

  let canvas1: HTMLCanvasElement;
  let canvas2: HTMLCanvasElement;
  let canvas3: HTMLCanvasElement;
  let image: HTMLImageElement;

  let ctx1: CanvasRenderingContext2D
  let ctx2: CanvasRenderingContext2D
  let ctx3: CanvasRenderingContext2D

  onMount(() => {
    ctx1 = canvas1.getContext('2d')!
    ctx2 = canvas2.getContext('2d')!
    ctx3 = canvas3.getContext('2d')!
  })

  let version = $state(20)
  let sideLen = $derived(version * 4 + 17)
  let padding = $state(10)
  let scale = $state(4)
  let finalSideLen = $derived(sideLen * scale + 2 * padding)
  let url = $state("https://qart.baetylboy.biz")
  let files: undefined | FileList = $state()
  let brightnessThreshold = $state(140)
  let random = $state(false)
  let working = $state(false)

  function updateImage() {
    ctx1.clearRect(0,0,sideLen,sideLen)
    ctx2.clearRect(0,0,sideLen,sideLen)
    ctx3.clearRect(0,0,finalSideLen,finalSideLen)
    
    if(files && files.length > 0) {
      image.src = URL.createObjectURL(files[0])
    }
  }

  function updatePreview() {
    ctx1.drawImage(image, 0, 0, sideLen, sideLen)

    let imgdata = ctx1.getImageData(0,0,sideLen,sideLen)

    let resized = new Uint8Array(imgdata.data.buffer)

    let preview = new Uint8ClampedArray(prepare(version, resized, brightnessThreshold, random));

    ctx2.putImageData(new ImageData(preview, sideLen, sideLen), 0, 0)
  }

  async function generateCode() {
    working = true 

    setTimeout(async () => {
      let imgdata = ctx1.getImageData(0,0,sideLen,sideLen)
  
      let resized = new Uint8Array(imgdata.data.buffer)
  
      let code = await generate(version, url, resized, brightnessThreshold, random, false, padding, scale)
  
      let codeData = new Uint8ClampedArray(code);
  
      ctx3.putImageData(new ImageData(codeData, finalSideLen, finalSideLen), 0, 0)
  
      working = false
    }, 10)
  }

  function saveImage() {
    const anchor = document.createElement('a');
    anchor.href = canvas3.toDataURL()
    anchor.download = "qart code";
    anchor.click();
    anchor.remove();
  }

  onMount(async () => {
    await wasmInit()
    init();
    updatePreview()
    generateCode()
  })

</script>

<main class=" grid lg:grid-cols-[auto_auto] w-fit gap-2 lg:gap-5 grid-rows-[auto_auto] max-h-screen">
  <div class="flex flex-col gap-5 justify-center items-center">
    <div class="flex lg:flex-row flex-col gap-2 lg:gap-5 justify-center">
      <label for="target-url" hidden>Target URL</label>
      <input class="input" id="target-url" type="url" bind:value={url} placeholder="https://example.com"/>
      <label for="upload" hidden>Upload Image</label>
      <input class="file-input file:bg-stone-300 file:mr-4" id="upload" type="file" accept=".png,.jpeg,.webp" bind:files={files} required onchange={updateImage}/>
    </div>
    <div class="flex flex-row gap-5 justify-center">
      <!-- svelte-ignore a11y_missing_attribute -->
      <img class="lg:w-45 lg:h-45 w-25 h-25 bg-stone-300" src="IMG_1145-min.JPEG" bind:this={image} onload={updatePreview}>
    
      <canvas class="lg:w-45 lg:h-45 w-25 h-25 bg-stone-300" bind:this={canvas1} width={sideLen} height={sideLen}></canvas>
  
      <canvas class="lg:w-45 lg:h-45 w-25 h-25 bg-stone-300" bind:this={canvas2} width={sideLen} height={sideLen}></canvas>
    </div>
  
  
    <div class="flex flex-col gap-2 lg:gap-5 justify-center w-min">
      <label class="label">
        QR Code Version
        <input type="range" min="1" max="40" step="1" bind:value={version} onchange={updatePreview}>
      </label>
      <label class="label">
        Brightness Threshold
        <input type="range" min="0" max="256" step="0.5" bind:value={brightnessThreshold} oninput={updatePreview}>
      </label>
      <label class="label">
        Randomly Distribute Free Modules
        <input class="checkbox" type="checkbox" bind:checked={random} onchange={updatePreview}>
      </label>
      <label class="label">
        Padding
        <input class="input w-fit" type="number" step="1" bind:value={padding}/>
      </label>
      <label class="label">
        Scale
        <input class="input w-fit" type="number" step="1" bind:value={scale}/>
      </label>
    </div>
  </div>

  <div class="flex flex-col gap-5 justify-center items-center">
    <canvas class="h-40 lg:h-100 bg-stone-500" bind:this={canvas3} width={finalSideLen} height={finalSideLen}></canvas>
    <div class="flex flex-row gap-5 justify-center">
      {#if working}
        <p class="label">working...</p>
      {:else}
        <button class="bg-stone-300 btn" onclick={generateCode}>Generate</button>
        <button class="bg-stone-300 btn" onclick={saveImage}>Save Image</button>
      {/if}
    </div>
  </div>
</main>

<footer class="lg:pt-0 pt-5 lg:absolute lg:bottom-5 lg:left-0 w-full text-center">
  <p class="text-sm lg:text-base">Created by <a href="https://baetylboy.biz">Andrew Yurovchak</a>. Technique by <a href="https://research.swtch.com/qart">Russ Cox</a></p>
</footer>

<style>
  canvas {
    image-rendering: pixelated;
  }
</style>