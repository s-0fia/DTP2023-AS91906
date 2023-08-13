let user = getSignInCookie();
let welcome = document.getElementById("welcomeMessage");
welcome.innerText = `Welcome ${user.firstName}!`;
let p_uids = document.getElementById("p-uids");

function signOutClick() {
    let expiry = "Thu, 01 Jan 1970 00:00:00 UTC";

    document.cookie = `id=; expires=${expiry}; path=/;`;
    document.cookie = `email=; expires=${expiry}; path=/;`;
    document.cookie = `firstName=; expires=${expiry}; path=/;`;
    document.cookie = `lastName=; expires=${expiry}; path=/;`;
    document.cookie = `pictureUrl=; expires=${expiry}; path=/;`;

    // redirect to the signin page
    window.location.href = "./welcome";
}

function httpQuery(field)
{
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.open("GET", `/?uid=${user.id}&q=${field}`, false); // false for synchronous request
    xmlHttp.send(null);
    console.log(xmlHttp.responseText);
    return xmlHttp.responseText;
}

function getUIDs() {
    let uids = httpQuery('class_uid');
    p_uids.innerText = `Class UIDS: ${uids}`;
}