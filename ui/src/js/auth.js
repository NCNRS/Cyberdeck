import {user} from './store.js';

export async function checkCookie() {
    const res = await fetch("/secure/check", {});
    if (res.status == 200) {
        let cookieResponse = await res.json();
        user.set(cookieResponse.user);
    } else {
        user.set(null);
    }
}

export async function postLogin(username, password) {
    const res = await fetch("/auth/login", {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ username: username, password: password }),
    });
    return await res.json();
}

export async function getLogout(username, password) {
    const res = await fetch("/auth/logout", {credentials: 'same-origin'});

    let logoutResponse = await res.json();
    if (logoutResponse.result == "error") {
        // may want to return an error here
    }else {
        user.set(null);
    }
}