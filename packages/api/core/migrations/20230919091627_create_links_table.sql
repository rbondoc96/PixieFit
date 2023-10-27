CREATE TABLE IF NOT EXISTS links (
    id SERIAL PRIMARY KEY NOT NULL,
    ulid VARCHAR UNIQUE DEFAULT generate_ulid() NOT NULL,
    model_name VARCHAR NOT NULL,
    model_id SMALLSERIAL NOT NULL,
    type VARCHAR NOT NULL,
    format VARCHAR NOT NULL,
    label VARCHAR NOT NULL,
    description TEXT,
    src TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL
);

CREATE INDEX links_model_type_model_id_idx ON links (model_name, model_id);
