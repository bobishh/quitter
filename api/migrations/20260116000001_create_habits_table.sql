CREATE TABLE IF NOT EXISTS habits (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    icon TEXT NOT NULL,
    theme_color TEXT NOT NULL,
    unit_name TEXT NOT NULL,
    frequency_hours DOUBLE PRECISION NOT NULL
);
