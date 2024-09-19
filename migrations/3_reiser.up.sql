create table reiser (
  reise_id serial primary key,
  fra_iata_code varchar(4) not null constraint fra_ikke_erlik_til check (fra_iata_code != til_iata_code),
  til_iata_code varchar(4) not null,
  fly_id int not null,
  avgang timestamp not null constraint ankomst_er_etter_avgang check (avgang < ankomst),
  ankomst timestamp not null,
  foreign key(fra_iata_code) references lufthavner(iata_code),
  foreign key(til_iata_code) references lufthavner(iata_code),
  foreign key(fly_id) references fly(fly_id)
);
