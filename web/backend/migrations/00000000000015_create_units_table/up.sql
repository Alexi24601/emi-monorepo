-- SQL defining the units table.
-- A unit is a standard of measurement for a physical quantity. This table defines the
-- units of measurement that are used in the system, following the International System of Units (SI).
-- It contains the following columns:
-- - id: the unique identifier of the unit.
-- - name: the name of the unit.
-- - symbol: the symbol of the unit.
-- - description: a description of the unit.
-- - created_at: the date and time when the unit was created.
-- - updated_at: the date and time when the unit was last updated.
-- - created_by: the unique identifier of the user who created the unit.
-- - updated_by: the unique identifier of the user who last updated the unit.
CREATE TABLE units (
  id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE,
  symbol VARCHAR(255) NOT NULL
);