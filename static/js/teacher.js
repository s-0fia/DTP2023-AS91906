// Constant DOM elements
const className = document.getElementById("name");
const teacherName = document.getElementById("teachername");

// Creates a new Classroom
function createClass() {
    // The entered fields
    let cname = className.value;
    let tname = teacherName.value;

    // Puts in the create class query and gets the status code of the response
    let response = httpQuery(user.id, `new_class${JSON.stringify([cname, tname])}`).status.toString();
    switch (response) {
        case "200": // Success!
            toggleAddClassScreen(); // Hide the add class screen
            refreshClasses(); // Refresh the classes so it shows
            break;
        case "403": // Forbidden! How are you here? Redirect to student dashboard
            toggleAddClassScreen(); // Hides the add class screen
            alert("You do not have permissions to do this action!"); // Telegraph to the user what is happening
            showLoader(); // Show the loader
            window.location.href = "/"; // Redirect the user
            break;
        default: // Catch all other errors. Possible errors: 400 & 500
            alert(`Error Code: ${response}. \nAn error occurred, please try again!`); // Tell the user to try again
            break;
    }
}