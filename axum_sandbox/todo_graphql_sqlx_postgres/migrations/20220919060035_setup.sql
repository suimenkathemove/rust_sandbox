CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE todos (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(30) NOT NULL
);