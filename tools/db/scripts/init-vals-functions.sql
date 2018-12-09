CREATE OR REPLACE FUNCTION user.find (user.account.id%TYPE)
    RETURNS user.account
    AS $$
        SELECT * FROM user.accounts WHERE id = $1;
    $$ LANGUAGE SQL
    STABLE
    STRICT;

CREATE OR REPLACE FUNCTION user.find_by_email (user.account.email%TYPE)
    RETURNS user.account
    AS $$
        SELECT * FROM user.accounts WHERE email = $1;
    $$ LANGUAGE SQL
    STABLE
    STRICT;

CREATE OR REPLACE FUNCTION user.find_by_phone (user.account.phone%TYPE)
    RETURNS user.account
    AS $$
        SELECT * FROM user.accounts WHERE phone = $1;
    $$ LANGUAGE SQL
    STABLE
    STRICT;
