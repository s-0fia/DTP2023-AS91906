// Given userData, it sets the values in a cookie so that the site can access it later
// takes userData: { email: String, id: String, firstName: String, lastName: String, pictureUrl: String }
function setSignInCookie(userData) {
    // Set the cookie expiry to 6 hours (21600000 milliseconds) from now, which provides easy of use,
    // but ensures that the user will not stay signed in for too long
    let date = new Date();
    date.setTime(date.getTime() + 21600000);
    let expiry = date.toUTCString();

    document.cookie = `id=${userData.id}; expires=${expiry}; path=/;`;
    document.cookie = `email=${userData.email}; expires=${expiry}; path=/;`;
    document.cookie = `firstName=${userData.firstName}; expires=${expiry}; path=/;`;
    document.cookie = `lastName=${userData.lastName}; expires=${expiry}; path=/;`;
    document.cookie = `pictureUrl=${userData.pictureUrl}; expires=${expiry}; path=/;`;
}

// Gets userData from a cookie in a usable format, or returns null if the user isn't signed in
// returns userData: { email: String, id: String, firstName: String, lastName: String, pictureUrl: String };
function getSignInCookie() {
    if (!cookieIsValid()) {
        return null;
    }
    let parts = document.cookie.split(";");
    var data = (str) => str.split("=")[1];
    let userData = {
        email: data(parts[1]), // start at 1 as 0 is "cookiesAccepted"
        id: data(parts[2]),
        firstName: data(parts[3]),
        lastName: data(parts[4]),
        pictureUrl: data(parts[5])
    }

    return userData;
}

// Checks if a given cookie has all the necessary fields and is therefore valid
// returns Boolean;
function cookieIsValid() {
    return document.cookie.includes("email")
        && document.cookie.includes("id")
        && document.cookie.includes("firstName")
        && document.cookie.includes("lastName")
        && document.cookie.includes("pictureUrl");
}

// Checks if a user is signed in by checking if the document cookie is valid
// returns Boolean;
function isSignedIn() {
    return cookieIsValid();
}