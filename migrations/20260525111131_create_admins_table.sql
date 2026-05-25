-- Up Migration

-- 1. Création de la table Admin
CREATE TABLE admins (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);

-- 2. Création de la table Person
CREATE TABLE persons (
    id BIGSERIAL PRIMARY KEY,
    admin_id BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    surname VARCHAR(255) NOT NULL,
    sex VARCHAR(50) NOT NULL,
    birthdate VARCHAR(100) NOT NULL, -- Note : Considère le type DATE si tu veux valider les dates
    birthplace VARCHAR(255) NOT NULL,
    class VARCHAR(100) NOT NULL,
    number VARCHAR(50) NOT NULL,
    parent_name VARCHAR(255) NOT NULL,
    parent_number VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL,

    CONSTRAINT fk_admin FOREIGN KEY (admin_id) REFERENCES admins(id) ON DELETE CASCADE
);

-- 3. Création de la table SocialMedia
CREATE TABLE social_medias (
    id BIGSERIAL PRIMARY KEY,
    person_id BIGINT NOT NULL,
    platform VARCHAR(100) NOT NULL,
    pseudo VARCHAR(100) NOT NULL,

    CONSTRAINT fk_person FOREIGN KEY (person_id) REFERENCES persons(id) ON DELETE CASCADE
);

-- Down Migration (Optionnel, pour les rollbacks de SQLx)
-- DROP TABLE IF EXISTS social_medias;
-- DROP TABLE IF EXISTS persons;
-- DROP TABLE IF EXISTS admins;
