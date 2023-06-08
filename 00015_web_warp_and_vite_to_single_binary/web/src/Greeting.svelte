<script>
    import { onMount } from 'svelte';

    const url = import.meta.env.PROD ? '/hello/svelte' : "http://localhost:3030/hello/svelte";

    let greeting = "";

    async function getHello() {
        const opts = {
            method: "get"
        };
        const response = await fetch(url, opts);
        const result = await response.text();
        console.log("Result:", result);

        greeting = result;
    }

    onMount(() => getHello())
</script>

<h1>{greeting}</h1>
