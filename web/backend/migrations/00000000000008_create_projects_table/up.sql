-- SQL to create the projects table.
--
-- A project is primarely characterized by its name and its description,
-- the user who created it, and then secondary metadata such as the date
-- when it was created and when it was last updated. Finally a project can
-- be currently read-only, and can be in several states such as "active",
-- "archived", "discontinued", "completed", etc. These states are available
-- in the "project_states" table, and may be extended in the future by an administator.
-- A project may also have a parent project.
-- A project has a budget, which is the amount of money allocated to the project. Subprojects
-- may have their own budget, and the sum of the budget of the subprojects should not exceed
-- the budget of the parent project. The budget is stored in the currency of the project,
-- and the currency is stored in the currency column.
-- A project may also have a predicted end date, such as when the funding for the project ends.
-- A project may also be public or private, and in the latter case, the access to the project
-- is managed by the project_editors, project_admins, and project_viewers tables.
-- The created_at and updated_at columns are used to store the creation and last update time of the record.
-- A project also has an optional URL for when the project has a separate website.
-- Finally, the project also has an optional path for the project logo.

CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    public BOOLEAN DEFAULT FALSE,
    state_id INTEGER NOT NULL REFERENCES project_states(id),
    parent_project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE,
    budget DECIMAL(10, 2) DEFAULT NULL,
    expenses DECIMAL(10, 2) DEFAULT NULL,
    currency VARCHAR(3) DEFAULT NULL,
    expected_end_date TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    end_date TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    website_url VARCHAR(255) DEFAULT NULL,
    logo_id INT DEFAULT NULL REFERENCES documents(id) ON DELETE SET NULL,
    editable_id INTEGER NOT NULL REFERENCES editables(id) ON DELETE CASCADE
);
