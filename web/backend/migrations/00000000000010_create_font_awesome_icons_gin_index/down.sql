-- Down version of font_awesome_icons_name_trgm_idx
DROP INDEX IF EXISTS font_awesome_icons_name_trgm_idx;

DROP FUNCTION concat_font_awesome_icons_name(text, text);