<script>
  import { Link } from "@inertiajs/svelte";
  
  export let user

  let count = 0
  const increment = () => {
    count += 1
  }
</script>

User: {user}
<Link href="/">Home</Link>

<button on:click={increment}>
  count is {count}
</button>
