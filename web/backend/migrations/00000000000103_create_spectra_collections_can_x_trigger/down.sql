-- Drop the `can_edit_spectra_collections` function and trigger on the spectra_collections table.

DROP TRIGGER can_edit_spectra_collections ON spectra_collections;
DROP FUNCTION IF EXISTS can_edit_spectra_collections_trigger();
DROP FUNCTION IF EXISTS can_edit_spectra_collections(INTEGER, INTEGER);
-- Drop the `can_delete_spectra_collections` function and trigger on the spectra_collections table.

DROP FUNCTION IF EXISTS can_delete_spectra_collections(INTEGER, INTEGER);
-- Drop the `can_view_spectra_collections` function and trigger on the spectra_collections table.

DROP FUNCTION IF EXISTS can_view_spectra_collections(INTEGER, INTEGER);
