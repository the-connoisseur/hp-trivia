-- Creates the table to track answered qurstions for state persistence.
-- Each row represents a selected (category, question) pair.
CREATE TABLE answered_questions (
id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
category INTEGER NOT NULL,
question INTEGER NOT NULL,
UNIQUE(category, question)
) ;
