// Given userData, it sets the values in a cookie so that the site can access it later
// takes userData: { email: String, id: String, name: String, pictureUrl: String }
function setSignInCookie(userData) {
    // Set the cookie expiry to 6 hours (21600000 milliseconds) from now, which provides easy of use,
    // but ensures that the user will not stay signed in for too long
    let date = new Date();
    date.setTime(date.getTime() + 21600000);
    let expiry = date.toUTCString();

    document.cookie = `email=${userData.email}; expires=${expiry}; path=/;`;
    document.cookie = `id=${userData.id}; expires=${expiry}; path=/;`;
    document.cookie = `name=${userData.name}; expires=${expiry}; path=/;`;
    document.cookie = `pictureUrl=${userData.pictureUrl}; expires=${expiry}; path=/;`;
}

// Gets userData from a cookie in a usable format, or returns null if the user isn't signed in
// returns userData: { email: String, id: String, name: String, pictureUrl: String };
function getSignInCookie() {
    if (!cookieIsValid()) {
        return null;
    }
    let parts = document.cookie.split(";");
    let userData = {
        email: parts[0].substring(6),
        id: parts[1].substring(4),
        name: parts[2].substring(6),
        pictureUrl: parts[3].substring(12)
    }

    return userData;
}

// Checks if a given cookie has all the necessary fields and is therefore valid
// returns Boolean;
function cookieIsValid() {
    return document.cookie.includes("email")
        && document.cookie.includes("id")
        && document.cookie.includes("name")
        && document.cookie.includes("pictureUrl");
}

// Checks if a user is signed in by checking if the document cookie is valid
// returns Boolean;
function isSignedIn() {
    return cookieIsValid();
}