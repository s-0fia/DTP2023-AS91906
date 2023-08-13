// The classroom class which is equivelant to ResponseClassroom
class Classroom {
    constructor(res) {
        this.UID = res.uid;
        this.name = res.name;
        this.teacherUIDs = res.teachers_uids;
        this.teacherName = res.teacher_name;
    }
}

// Get the user from web.js
let user = getSignInCookie();

// Get the constant dom element and adjust it
const welcome = document.getElementById("welcomeMessage");
const addClassScreen = document.getElementById("new-class");
const classId = document.getElementById("classid");
welcome.innerText = `Welcome ${user.firstName}! Your id is: "${user.id}"`;

// Signs the user out and clears the cookies
function signOutClick() {
    [ // Iterate and clear each respective field
        "id",
        "email",
        "firstName",
        "lastName",
        "pictureUrl"
    ].forEach(clearCookieField);
    
    // redirect to the signin page
    window.location.href = "./welcome";
}

// Handles clicking on any classroom element
function classClick(classUID) {
    // Shows the loading screen and redircts to provide smooth transition
    showLoader();
    setTimeout(() => {
        window.location.href = `/c/${classUID}`;
    }, 500);
}

// Loads in all the classrooms
function loadClasses() {
    JSON.parse( // Parse the query result
        // Query the server
        httpQuery(user.id, "get_classes").text
    ).map( // Map the shapeless objs to Classroom objects
        obj => new Classroom(obj)
    ).forEach(classroom => { // Iterate over the classrooms
        // Create a new div for each class with relevant information
        let fragment = create(
            `<div id="${classroom.UID}" class="class-child" onclick="classClick('${classroom.UID}');">` +
                `<h1><a>${classroom.name}</a></h1>` +
                `<h2>Teacher: ${classroom.teacherName}</h2>` +
            `</div>`
        );
        // Add it to the DOM
        document.getElementById('classes')
            .appendChild(fragment);
    });
}

// Creates a DOM fragment
const create = (htmlStr) => {
    // The new fragment and the temporary div
    var frag = document.createDocumentFragment(),
        temp = document.createElement('div');
    // Set the innerHTML to the given fragment
    temp.innerHTML = htmlStr;
    while (temp.firstChild) {
        frag.appendChild(temp.firstChild);
    }
    // Return the fragment
    return frag;
}

// Joins a user into a class
function joinClass() {
    // Put the request in to join
    let id = classId.value;
    let response = httpQuery(user.id, `add_class${id}`).status.toString();
    switch (response) {
        case "200": // Success!
            toggleAddClassScreen(); // Hide the add class screen
            refreshClasses(); // Refresh the classes so it shows
            break;
        default: // Catch all errors. Possible errors: 400 & 500
            alert(`Error Code: ${response}. \nAn error occurred, please try again!`); // Tell the user to try again
            break;
    }
}

// Shows/hides the add class screen
function toggleAddClassScreen() {
    addClassScreen.style.display = (addClassScreen.style.display == "") ? "flex" : "";
}

// Refreshes the classes so new classes can be seen after being added
function refreshClasses() {
    showLoader(); // Show the loader as this can take time
    // Clear the classes
    setTimeout(() => {
        let classes = document.getElementsByClassName("class-child");
        for (let i = classes.length - 1; i > 0; i--)
            classes[i].remove();
        // Fetch classes again
        loadClasses();
    }, 500);
    // Hide the loader
    hideLoader();
}

// Load the classes in
loadClasses();