// UserData class which stores all the relevant client information
class UserData {
    // Default all the constructors if you want them to be empty
    constructor(email, id, firstName, pictureUrl) {
        this.email = email;
        this.id = id;
        this.firstName = firstName;
        this.pictureUrl = pictureUrl;
    }
}

class Response {
    constructor(xmlHttp) {
        this.status = xmlHttp.status;
        this.text = xmlHttp.responseText;
    }
}

function httpQuery(uid, field)
{
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.open("GET", `/?uid=${uid}&q=${field}`, false); // false for synchronous request
    xmlHttp.send(null);
    return new Response(xmlHttp);
}

// Given userData, it sets the values in a cookie so that the site can access it later
function setSignInCookie(userData) {
    // Set the cookie expiry to 6 hours (21600000 milliseconds) from now, which provides easy of use,
    // but ensures that the user will not stay signed in for too long
    let date = new Date();
    date.setTime(date.getTime() + 21600000);
    let expiry = date.toUTCString();

    document.cookie = `id=${userData.id}; expires=${expiry}; path=/;`;
    document.cookie = `email=${userData.email}; expires=${expiry}; path=/;`;
    document.cookie = `firstName=${userData.firstName}; expires=${expiry}; path=/;`;
    document.cookie = `pictureUrl=${userData.pictureUrl}; expires=${expiry}; path=/;`;
}

// Gets userData from a cookie in a usable format, or returns null if the user isn't signed in
function getSignInCookie() {
    if (!cookieIsValid()) {
        return null;
    }

    let parts = document.cookie.split(";");
    let email, id, firstName, pictureUrl;
    for (let i = 1; i < parts.length; i++) {
        let key = parts[i].split("=")[0].substring(1);
        let value = parts[i].split("=")[1];
        if (key == "email")      email      = value;
        if (key == "id")         id         = value;
        if (key == "firstName")  firstName  = value;
        if (key == "pictureUrl") pictureUrl = value;
    }
    
    return new UserData(
        email,
        id,
        firstName,
        pictureUrl
    );
}

// Checks if a given cookie has all the necessary fields and is therefore valid
// returns Boolean;
function cookieIsValid() {
    return document.cookie.includes("email")
        && document.cookie.includes("id")
        && document.cookie.includes("firstName")
        && document.cookie.includes("pictureUrl");
}

// Checks if a user is signed in by checking if the document cookie is valid
// returns Boolean;
function isSignedIn() {
    return cookieIsValid();
}