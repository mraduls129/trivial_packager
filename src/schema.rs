// @generated automatically by Diesel CLI.

diesel::table! {
    courses (id) {
        id -> Varchar,
        anatomia -> Nullable<Varchar>,
        english -> Nullable<Varchar>,
        biologia -> Nullable<Varchar>,
        castellano -> Nullable<Varchar>,
        clasica -> Nullable<Varchar>,
        dibuix -> Nullable<Varchar>,
        ed_fisica -> Nullable<Varchar>,
        filosofia -> Nullable<Varchar>,
        fisica_quimica -> Nullable<Varchar>,
        frances -> Nullable<Varchar>,
        historia -> Nullable<Varchar>,
        grec -> Nullable<Varchar>,
        informatica -> Nullable<Varchar>,
        literatura -> Nullable<Varchar>,
        llati -> Nullable<Varchar>,
        mates -> Nullable<Varchar>,
        musica -> Nullable<Varchar>,
        orientacio -> Nullable<Varchar>,
        plastica -> Nullable<Varchar>,
        religio -> Nullable<Varchar>,
        tecnologia -> Nullable<Varchar>,
        valencia -> Nullable<Varchar>,
        etica -> Nullable<Varchar>,
    }
}

diesel::table! {
    questions (id) {
        id -> Integer,
        subject -> Varchar,
        level -> Integer,
        question -> Varchar,
        hide -> Bool,
        answers -> Varchar,
        tries -> Integer,
        time -> Integer,
        image -> Varchar,
        bigger -> Bool,
        created_at -> Timestamp,
        verified -> Bool,
        creator -> Varchar,
    }
}

diesel::table! {
    students_questions (id) {
        id -> Integer,
        course_creator -> Varchar,
        name_creator -> Varchar,
        subject -> Varchar,
        level -> Integer,
        question -> Varchar,
        answers -> Varchar,
        tries -> Integer,
        time -> Integer,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (email) {
        name -> Varchar,
        email -> Varchar,
        hash -> Varchar,
        created_at -> Timestamp,
        gender -> Varchar,
        role -> Varchar,
        password_changed -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    courses,
    questions,
    students_questions,
    users,
);
