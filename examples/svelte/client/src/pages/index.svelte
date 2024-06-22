<script>
    import { Link } from "@inertiajs/svelte";

    export let user
    export let message
</script>

Hey {user}, how are you?
{message}

<Link href="/counter">Counter</Link>