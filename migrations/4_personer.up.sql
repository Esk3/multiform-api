create table personer (
  person_id serial primary key,
  fornavn varchar(64) not null,
  etternavn varchar(64) not null,
  adresse varchar(128) not null,
  postnummer int not null,
  epost varchar(64) not null,
  telefonnummer int not null
);
