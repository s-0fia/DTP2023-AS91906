// The classroom class which is equivelant to ResponseClassroom
class Classroom {
    constructor(res) {
        this.UID = res.uid;
        this.name = res.name;
        this.teacherUIDs = res.teachers_uids;
        this.teacherName = res.teacher_name;
    }
}

// Redirects users back to the dashboard
function dashboardClick() {
    showLoader();
    window.location.href = '/';
}

// The DOM elements
const className = document.getElementById("classname");
const teacherName = document.getElementById("teachername");
const classCode = document.getElementById("classcode");
const titleTag = document.getElementsByTagName('title')[0];

// The classuid from the URL
const classuid = window.location.href.split('/c/')[1].replace('/', '');

// Construct a classroom from a query
let classroom = new Classroom(
    JSON.parse(
        httpQuery(classuid, "get_class").text
    )
);

// Adjust the HTML elements to suit it
className.innerHTML += classroom.name;
teacherName.innerHTML += classroom.teacherName;
classCode.innerHTML += classuid;
titleTag.innerHTML = `${classroom.name} - ${classroom.teacherName}`;