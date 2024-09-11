  -- https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes
  -- https://en.wikipedia.org/wiki/International_Air_Transport_Association

create table lufthavner (
  iata_code varchar(4) primary key,
  airport_type text not null,
  name text not null,
  elevation_ft real,
  continent varchar(6) not null,
  iso_country varchar(2) not null,
  iso_region varchar(7) not null,
  municipality text not null,
  gps_code varchar(12),
  local_code varchar(10),
  coordinates text not null
);

create type status as enum ('voksen', 'barn', 'honnør');
create type billett_type as enum ('billig', 'flex', 'luxus');

create table billett (
  billett_id serial primary key,
  fra_iata_code varchar(4) not null constraint fra_ikke_erlik_til check (fra_iata_code != til_iata_code),
  til_iata_code varchar(4) not null,
  timestamp timestamp default current_timestamp not null,
  status status not null,
  billett_type billett_type not null,
  foreign key(fra_iata_code) references lufthavner(iata_code),
  foreign key(til_iata_code) references lufthavner(iata_code)
);

create table lufthavner_csv (
  ident varchar(8) primary key,
  "type" text not null,
  name text not null,
  elevation_ft real,
  continent varchar(6) not null,
  iso_country varchar(2) not null,
  iso_region varchar(7) not null,
  municipality text,
  gps_code varchar(12),
  iata_code varchar(4),
  local_code varchar(10),
  coordinates text not null
);

copy lufthavner_csv
from '../airport-codes.csv'
delimiter ','
csv header;

insert into lufthavner
select iata_code, "type" as airport_type, name, elevation_ft, continent, iso_country, iso_region, municipality, gps_code, local_code, coordinates
from lufthavner_csv
where iata_code is not null and
municipality is not null;

drop table lufthavner_csv;
