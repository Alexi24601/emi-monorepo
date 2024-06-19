-- The function `can_view_organisms` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_view function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the organisms_users_roles table or
-- the organisms_teams_users table with an appropriate role id.
CREATE FUNCTION can_view_organisms(author_user_id INTEGER, this_organisms_id UUID)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_sample_id UUID;
    this_nameplate_id INTEGER;
    this_project_id INTEGER;
    this_created_by INTEGER;
    this_updated_by INTEGER;
BEGIN
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sample_id, nameplate_id, project_id, created_by, updated_by, 1 INTO this_sample_id, this_nameplate_id, this_project_id, this_created_by, this_updated_by, canary FROM organisms WHERE organisms.id = this_organisms_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN FALSE;
    END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = this_created_by THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the updated_by of the row.
    IF author_user_id = this_updated_by THEN
        RETURN TRUE;
    END IF;
-- If the parent column is not NULL, we call the can_view function of the parent column to determine whether the user can edit the row.
    IF this_sample_id IS NOT NULL THEN
        IF NOT can_view_samples(author_user_id, this_sample_id) THEN
            RETURN FALSE;
        END IF;
    END IF;
        IF NOT can_view_nameplates(author_user_id, this_nameplate_id) THEN
            RETURN FALSE;
        END IF;
        IF NOT can_view_projects(author_user_id, this_project_id) THEN
            RETURN FALSE;
        END IF;
    RETURN TRUE;
END;
$$
LANGUAGE plpgsql PARALLEL SAFE;

-- The function `can_admin_organisms` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_admin function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the organisms_users_roles table or
-- the organisms_teams_users table with an appropriate role id.
CREATE FUNCTION can_admin_organisms(author_user_id INTEGER, this_organisms_id UUID)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_sample_id UUID;
    this_nameplate_id INTEGER;
    this_project_id INTEGER;
    this_created_by INTEGER;
    this_updated_by INTEGER;
BEGIN
-- If the author_user_id is NULL, we return FALSE.
    IF author_user_id IS NULL THEN
        RAISE EXCEPTION 'The author_user_id cannot be NULL.';
    END IF;
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sample_id, nameplate_id, project_id, created_by, updated_by, 1 INTO this_sample_id, this_nameplate_id, this_project_id, this_created_by, this_updated_by, canary FROM organisms WHERE organisms.id = this_organisms_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN FALSE;
    END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = this_created_by THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the updated_by of the row.
    IF author_user_id = this_updated_by THEN
        RETURN TRUE;
    END IF;
-- If the parent column is not NULL, we call the can_admin function of the parent column to determine whether the user can edit the row.
    IF this_sample_id IS NOT NULL THEN
        IF NOT can_admin_samples(author_user_id, this_sample_id) THEN
            RETURN FALSE;
        END IF;
    END IF;
        IF NOT can_admin_nameplates(author_user_id, this_nameplate_id) THEN
            RETURN FALSE;
        END IF;
        IF NOT can_admin_projects(author_user_id, this_project_id) THEN
            RETURN FALSE;
        END IF;
    RETURN FALSE;
END;
$$
LANGUAGE plpgsql PARALLEL SAFE;

-- The function `can_update_organisms` takes a user ID (INTEGER) and the primary keys
-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability
-- may depend on the parent column, this function retrieves the value of the parent column from the row
-- and calls the parent column's can_update function if the parent column is not NULL. Otherwise, the function
-- checks if the row was created by the user or if the user is found in either the organisms_users_roles table or
-- the organisms_teams_users table with an appropriate role id.
CREATE FUNCTION can_update_organisms(author_user_id INTEGER, this_organisms_id UUID)
RETURNS BOOLEAN AS $$
DECLARE
    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.
    this_sample_id UUID;
    this_nameplate_id INTEGER;
    this_project_id INTEGER;
    this_created_by INTEGER;
    this_updated_by INTEGER;
BEGIN
-- If the author_user_id is NULL, we return FALSE.
    IF author_user_id IS NULL THEN
        RAISE EXCEPTION 'The author_user_id cannot be NULL.';
    END IF;
-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).
    SELECT sample_id, nameplate_id, project_id, created_by, updated_by, 1 INTO this_sample_id, this_nameplate_id, this_project_id, this_created_by, this_updated_by, canary FROM organisms WHERE organisms.id = this_organisms_id;
-- If the row does not exist, we return FALSE.
    IF canary IS NULL THEN
        RETURN FALSE;
    END IF;
-- We check whether the user is the created_by of the row.
    IF author_user_id = this_created_by THEN
        RETURN TRUE;
    END IF;
-- We check whether the user is the updated_by of the row.
    IF author_user_id = this_updated_by THEN
        RETURN TRUE;
    END IF;
-- If the parent column is not NULL, we call the can_update function of the parent column to determine whether the user can edit the row.
    IF this_sample_id IS NOT NULL THEN
        IF NOT can_update_samples(author_user_id, this_sample_id) THEN
            RETURN FALSE;
        END IF;
    END IF;
        IF NOT can_update_nameplates(author_user_id, this_nameplate_id) THEN
            RETURN FALSE;
        END IF;
        IF NOT can_update_projects(author_user_id, this_project_id) THEN
            RETURN FALSE;
        END IF;
    RETURN FALSE;
END;
$$
LANGUAGE plpgsql PARALLEL SAFE;

-- The function `can_update_organisms_trigger` is a trigger function that checks whether the user can update the row.
CREATE FUNCTION can_update_organisms_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        IF NOT can_update_organisms(NEW.updated_by, NEW.id) THEN
            RAISE EXCEPTION 'unauthorized_update' USING DETAIL = 'Unauthorized to update this row of the `organisms` table.';
        END IF;
    END IF;
-- We check whether the user can update the row.
    IF TG_OP = 'INSERT' AND NEW.sample_id IS NOT NULL THEN
        IF NOT can_update_samples(NEW.created_by, NEW.sample_id) THEN
            RAISE EXCEPTION 'unauthorized_update' USING DETAIL = 'Unauthorized to update this row of the `organisms` table.';
        END IF;
    END IF;
-- We check whether the user can update the row.
    IF TG_OP = 'INSERT' THEN
        IF NOT can_update_nameplates(NEW.created_by, NEW.nameplate_id) THEN
            RAISE EXCEPTION 'unauthorized_update' USING DETAIL = 'Unauthorized to update this row of the `organisms` table.';
        END IF;
    END IF;
-- We check whether the user can update the row.
    IF TG_OP = 'INSERT' THEN
        IF NOT can_update_projects(NEW.created_by, NEW.project_id) THEN
            RAISE EXCEPTION 'unauthorized_update' USING DETAIL = 'Unauthorized to update this row of the `organisms` table.';
        END IF;
    END IF;
    RETURN NEW;
END;
$$
LANGUAGE plpgsql PARALLEL SAFE;

-- We create a trigger that calls the `can_update_organisms` function before each INSERT or UPDATE.
CREATE TRIGGER can_update_organisms
BEFORE INSERT OR UPDATE ON organisms
FOR EACH ROW
EXECUTE FUNCTION can_update_organisms_trigger();
