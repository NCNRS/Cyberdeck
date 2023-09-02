<script>
    import { Node, Svelvet, Group } from 'svelvet';
    import { getServices, getServiceGroups } from "./../js/fetch.js";
    import { onMount } from "svelte";

    let serviceResponse, serviceGroupResponse, info;
    let groupMap = new Map();
    let serviceMap = new Map();
    let errorMessage = "";
    let loading = true;

    onMount(async () => {
        serviceResponse = await getServices();
        serviceGroupResponse = await getServiceGroups();
        if (serviceGroupResponse.result == "error") {
            errorMessage = serviceGroupResponse.message;
        } else {
            groupMap = new Map(Object.entries(serviceGroupResponse.services));
        }

        if (serviceResponse.result == "error") {
            errorMessage = serviceResponse.message;
        } else {
            serviceMap = new Map(Object.entries(serviceResponse.services));
        }
        loading = false;

    });

    function handleClick(e) {
        const { detail } = e;
        const id = detail.node.id.split('-');
        console.log("hello");
        console.log(id);
        console.log(serviceMap);
        info = JSON.stringify(serviceMap.get(id[1]));
    }


</script>
  
  <div id="overview">
    <container class="graph">
      {#if loading == true}
      <div>loading...</div>
      {:else}
      <Svelvet id="graph" minimap controls theme="dark">
          {#each [...groupMap] as [server, services]}
        <Group color="#0F131A" groupName="{server}" position={{x: 0, y: 100}} width={600} height={200}>
              {#each services as service}
                  <Node  useDefaults id='{service.id}' position={{x: 10, y: 25}} on:nodeClicked="{handleClick}">
                      <div class='nodeWrapper'>
                      <div id='container'>
                      <div id='heading'>{service.name}</div>
                      <table id="infoTable">
                          <tr>
                              <td>Status</td> 
                              <td>{service.status}</td>     
                          </tr>
                      </table>
                      </div>  
                  </div>  
                </Node>
              {/each}
          </Group>
          {/each}
      </Svelvet>
      {/if}
    </container>
    <container class="info">
      {info}
    </container>
  </div>
  
<style>
#overview {     
  display: flex;
  flex-flow: row wrap;
  align-items: flex-start;
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
  padding: 10px;
  flex: 25%;
}

.nodeWrapper{
      box-sizing: border-box;
      width:fit-content;
      border-radius: 8px;
      height: fit-content;
      position: relative;
      pointer-events: auto;
      display: flex;
      flex-direction: column;
      padding: 1px;
      gap: 10px;
    }
  
    #heading{
          display: flex;
          justify-content: center;
          background-color: #E6B450;
          padding: 10px;
          font-size: 18px;
          font-weight: 600;
          border-top-right-radius: 8px;
          border-top-left-radius: 8px;
      }
      
    #infoTable td{
        width: 70px;
          margin: 0px;
          padding: 8px;
          justify-content: space-evenly;
          border-bottom: 1px solid gray;
          border-right: 1px solid gray;
    }
  
    #infoTable td:last-child {       
          border-right: none;
      }
</style>
  