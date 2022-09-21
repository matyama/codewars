CREATE FUNCTION agecalculator(
    d date
) RETURNS int RETURNS NULL ON NULL INPUT AS $$ 
  BEGIN
    RETURN EXTRACT(year FROM AGE(d));
  END;
$$ LANGUAGE plpgsql;
