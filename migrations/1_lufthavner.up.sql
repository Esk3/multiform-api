create table lufthavner (
  -- https://en.wikipedia.org/wiki/International_Air_Transport_Association
  iata_code varchar(4) primary key,
  airport_type text not null,
  name text not null,
  elevation_ft real,
  continent varchar(6) not null,
  -- https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes
  iso_country varchar(2) not null,
  iso_region varchar(7) not null,
  municipality text not null,
  gps_code varchar(12),
  local_code varchar(10),
  coordinates text not null
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
