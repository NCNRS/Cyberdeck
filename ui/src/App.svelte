<script>
	import { user } from "./js/store.js";
	import { checkCookie } from "./js/auth.js";
	import NavBar from "./component/Navbar.svelte";
	import LogIn from "./pages/Login.svelte";
	import LogOut from "./pages/Logout.svelte";
	import Secure from "./pages/Secure.svelte";
	import Apicheck from "./pages/Apicheck.svelte";
	import { onMount } from "svelte";
    import Overview from "./pages/Overview.svelte";

	let menu;
	$: loggedin = $user !== null;

	// check if logged in
	onMount(checkCookie);

	const set_menu_items = (loggedin) => {
		if (loggedin) {
			return [
				{ label: "About", id: 1 },
				{ label: "Overview", id: 6 },
				{ label: "Secure", id: 3 },
				{ label: "API Check", id: 5 },
				{ label: "Logout", id: 4 },
			];
		} else {
			return [
				{ label: "About", id: 1 },
				{ label: "API Check", id: 5 },
				{ label: "Login", id: 2 },
			];
		}
	};
</script>

<!-- MENNU BAR ON TOP -->
<NavBar navItems={set_menu_items(loggedin)} bind:menu />

<!-- PAGE LOADING -->
{#if menu === 1}
	<div>
		<container>
			{#if !loggedin}
				<h4>Requires Login</h4>
			{:else}
				<h4>Logged In as {$user}</h4>
			{/if}
			<p>ABOUT</p>
		</container>
	</div>
{:else if menu === 2}
	<LogIn />
{:else if menu === 3}
	<Secure />
{:else if menu === 4}
	<LogOut />
{:else if menu === 5}
	<Apicheck />
{:else if menu === 6}
	<Overview />
{:else}
	<h2>404 Page Not Found</h2>
{/if}

<style>
	div {
		margin: 25px;
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
	}
</style>
