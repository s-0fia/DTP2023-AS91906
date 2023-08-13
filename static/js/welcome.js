// Get elements from the DOM
const blur = document.getElementById("blur");
const cookiesAgreement = document.getElementById("cookiesAgreement");

// If there is no cookiesAccepted cookie, show the cookies acceptance page
if (!document.cookie.includes("cookiesAccepted")) {
    blur.style.display = "block"; // Blur the background
    cookiesAgreement.style.display = "block"; // Show the cookiesAgreement element
}

// Once the cookies are accepted
function cookiesAccepted() {
    // Hide everything
    blur.style.display = "none";
    cookiesAgreement.style.display = "none";

    // Set the expiry for the cookies accepted to 1 year (in milliseconds) from now.
    let date = new Date();
    date.setTime(date.getTime() + 31536000000);
    let expiry = date.toUTCString();
    document.cookie = `cookiesAccepted=true; expires=${expiry}; path=/;`;
}

// When the sign in button is clicked
function signInClick() {
    // redirect to the signin page
    window.location.href = "./signin";
}