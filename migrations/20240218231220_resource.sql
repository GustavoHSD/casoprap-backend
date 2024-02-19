DROP TABLE IF EXISTS Resource;

CREATE TABLE Resource (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    description VARCHAR(255) NOT NULL,
    price FLOAT NOT NULL,
    volunteer_id INTEGER NOT NULL,
    FOREIGN KEY (volunteer_id) REFERENCES Volunteer(id)
);
