drop trigger før_utløpt_billett_oppdatering_trigger on billetter;
drop function fn_før_utløpt_billett_oppdatering;

drop view ledige_seter;
drop view kan_bli_bekreftet_billetter;
drop view utløpt_billetter;
drop view bekreftet_billetter;
drop view ubekreftet_billetter;

drop table billetter;

drop type status;
drop type billett_type;
