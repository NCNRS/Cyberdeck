export async function getSecure() {
    let res = await fetch('/secure');
    let secureResponse = await res.json();
    return JSON.stringify(secureResponse.user);
} 

export async function getServices() {
    let res = await fetch('/services');
    return await res.json();
}

export async function getServiceGroups() {
    let res = await fetch('/servicegroups');
    return await res.json();
}

export async function getApi(api_token) {
    let res = await fetch('/api', {
        headers: {
            'Authorization': 'Bearer '+ api_token,
            Accept: "application/json", 
        },
    });
    return await res.json();
} 