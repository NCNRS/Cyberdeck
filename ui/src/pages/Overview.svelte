<script>
    import { onMount } from "svelte";
    import { user } from "./../js/store.js";
    import { getSecure } from "./../js/fetch.js";
    import Graph from "./../component/Graph.svelte"
  
    let response;
  
    onMount(async () => {
      response = await getSecure();
    });
</script>
  
  <div>
    <container class="graph">
        <Graph />
    </container>
    <container class="info">
        
    </container>
  </div>
  
<style>
div {     
  display: flex;
  flex-flow: row wrap;
  align-items: center;
}

@media only screen and (max-width: 620px) {
    container.graph, container.info {
    flex: 100%;
  }
}

container.graph {
  padding: 10px;
  flex: 70%;
  height: 600px;
}

container.info {
  background-color: dodgerblue;
  padding: 10px;
  flex: 25%;
}
</style>
  