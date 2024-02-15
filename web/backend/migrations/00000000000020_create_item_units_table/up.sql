-- SQL defining the item_units table.
-- An item unit is a unit of measure for an item. For example, an item may be measured in
-- grams, milliliters, or meters. This table defines the units of measure that are used to
-- measure items. Some items may be measured reasonably in different units, and this table
-- allows for the definition of the units of measure that are used to measure items.
CREATE TABLE item_units (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    item_id BIGINT NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    unit_id BIGINT NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    UNIQUE (item_id, unit_id)
);