let blur = document.getElementById("blur");
let cookiesAgreement = document.getElementById("cookiesAgreement");

if (!document.cookie.includes("cookiesAccepted")) {
    blur.style.display = "block";
    cookiesAgreement.style.display = "block";
}

function cookiesAccepted() {
    blur.style.display = "none";
    cookiesAgreement.style.display = "none";

    // Set the expiry for the cookies accepted to 1 year (in milliseconds) from now.
    let date = new Date();
    date.setTime(date.getTime() + 31536000000);
    let expiry = date.toUTCString();

    document.cookie = `cookiesAccepted=true; expires=${expiry}; path=/;`;
}

function signInClick() {
    // redirect to the signin page
    window.location.href = "./signin";
}