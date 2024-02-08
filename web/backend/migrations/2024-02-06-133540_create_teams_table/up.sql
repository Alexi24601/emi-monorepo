-- SQL to define the teams table.
-- The teams table is used to describe an equipe of users that work together on a project.
-- A team may be composed of one or more users, and a user may be part of one or more teams.
-- The team name is unique. A team may have a parent team, and a team may have one or more child teams.
-- The team abstraction is primarily used to manage access to projects, so to avoid having to
-- manage access to each user individually. The created_at and updated_at columns are used to store
-- the creation and last update time of the record.
CREATE TABLE teams (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE,
  parent_team_id INT DEFAULT NULL,
  state INT NOT NULL DEFAULT 0,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (parent_team_id) REFERENCES teams (id) ON DELETE CASCADE
);