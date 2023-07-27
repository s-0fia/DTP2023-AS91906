class Classroom {
    constructor(res) {
        this.UID = res.uid;
        this.name = res.name;
        this.teacherUIDs = res.teachers_uids;
        this.teacherName = res.teacher_name;
    }
}

const className = document.getElementById("classname");
const teacherName = document.getElementById("teachername");
const titleTag = document.getElementsByTagName('title')[0];

const classuid = window.location.href.split('/c/')[1].replace('/', '');

let classroom = new Classroom(JSON.parse(httpQuery(classuid, "get_class").text));

console.log(classroom);

className.innerHTML += classroom.name;
teacherName.innerHTML += classroom.teacherName;
titleTag.innerHTML = `${classroom.name} - ${classroom.teacherName}`;