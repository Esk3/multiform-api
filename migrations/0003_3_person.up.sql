create table person (
  id serial primary key,
  fornavn varchar(64) not null,
  etternavn varchar(64) not null,
  adresse varchar(128) not null,
  postnummer int not null,
  epost varchar(64) not null,
  telefonnummer int not null
);

create table bestilling_person (
  bestilling_id int not null,
  person_id int not null,
  primary key (bestilling_id, person_id),
  foreign key(bestilling_id) references bestilling(id),
  foreign key(person_id) references person(id)
);
