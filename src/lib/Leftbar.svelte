<script lang="ts">
    import PacketOverview from "./PacketOverview/PacketOverview.svelte";
    let files;

	$: if (files) {
		// Note that `files` is of type `FileList`, not an Array:
		// https://developer.mozilla.org/en-US/docs/Web/API/FileList
		console.log(files);

		for (const file of files) {
            file.arrayBuffer().then(item => {
                const array = new Uint8Array(item);
                console.log(array)
            })
            const file_array = Uint8Array.from(file.stream())
            console.log(file_array)
			console.log(`${file.name}: ${file.size} bytes`);
		}
	}
</script>
<div class="pl-3 basis-1/4">
    <input bind:files id='replaySelect' type='file' hidden/>
    <input class="btn gap-2" type='button' value='Open Replay' on:click={() => document.getElementById("replaySelect").click()}/>
    <div class="divider" />
    <div class="heading">Replay Overview</div>
    <div class="p-4 flex flex-col justify-start">
        <div><span class="font-bold">Tank: </span>60 TP</div>
        <div><span class="font-bold">Map: </span>Murovanka</div>
    </div>
    <div class="mt-8 heading">Packet Overview</div>
    <PacketOverview />
</div>