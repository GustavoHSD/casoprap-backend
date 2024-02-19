DROP TABLE IF EXISTS Animal;

CREATE TABLE Animal (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    race VARCHAR(255) NOT NULL,
    a_type VARCHAR(255) NOT NULL,
    age INTEGER,
    rescue_location VARCHAR(255) NOT NULL,
    is_adopted BOOLEAN NOT NULL DEFAULT true,
    responsible_volunteer INTEGER NOT NULL,
    FOREIGN KEY (responsible_volunteer) REFERENCES Volunteer(id)
);

