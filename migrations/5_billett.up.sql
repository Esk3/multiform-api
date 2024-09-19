create type status as enum ('voksen', 'barn', 'honnør');
create type billett_type as enum ('billig', 'flex', 'luxus');

create table billetter (
  billett_id serial primary key,
  reise_id int,
  person_id int,
  bekreftet bool default false not null constraint kan_bekrefte check (bekreftet = false or (reise_id is not null and person_id is not null)), 
  status status not null,
  billett_type billett_type not null,
  timestamp timestamp default current_timestamp not null,
  foreign key (reise_id) references reiser(reise_id),
  foreign key (person_id) references personer(person_id)
);

create view ledige_seter as 
select r.reise_id, r.fly_id,
  count( case when b.billett_type = 'luxus' then 1 end) as luxus_billetter,
  (select f.luxus_seter from fly f where f.fly_id = r.fly_id),
  count( case when b.billett_type = 'flex' then 1 end) as flex_billetter,
  (select f.flex_seter from fly f where f.fly_id = r.fly_id),
  count( case when b.billett_type = 'billig' then 1 end) as billig_billetter,
  (select f.billig_seter from fly f where f.fly_id = r.fly_id)
  from reiser r left join
  billetter b on b.reise_id = r.reise_id
  group by (r.reise_id);

create view totalt_ledige_seter as
  select reise_id, fly_id,
    luxus_billetter + flex_billetter + billig_billetter as totalt_billetter,
    luxus_seter + flex_seter + billig_seter as totalt_seter
  from ledige_seter;

create view bekreftet_billetter as
  select *
  from billetter
  where bekreftet = true;

create view ubekreftet_billetter as
  select *
  from billetter
  where bekreftet = false
  and timestamp > now() - interval '5 day';

create view kan_bli_bekreftet_billetter as
  select *
  from ubekreftet_billetter
  where reise_id is not null
  and person_id is not null;

create view utløpt_billetter as
  select *
  from billetter
  where bekreftet = false
  and timestamp < now() - interval '5 day';

create function fn_før_utløpt_billett_oppdatering()
returns trigger as $$
begin
  if old.bekreftet = false and new.bekreftet = true and old.timestamp < now() - interval '5 day' then
    raise exception 'Billett er utløpt og kan ikke bli bekreftet';
  end if;
  return new;
end;
$$ language plpgsql;

create trigger før_utløpt_billett_oppdatering_trigger
before update of bekreftet on billetter
for each row
execute function fn_før_utløpt_billett_oppdatering();
