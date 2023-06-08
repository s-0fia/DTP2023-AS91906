let user = getSignInCookie();
let welcome = document.getElementById("welcomeMessage");
welcome.innerText = `Welcome ${user.firstName}!`;

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