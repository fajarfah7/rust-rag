-- Add up migration script here
DROP TABLE IF EXISTS public.documents;
CREATE TABLE public.documents(
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    original_filename TEXT NOT NULL,
    storage_path TEXT NOT NULL,     -- path / key
    status TEXT NOT NULL DEFAULT 'uploaded', -- uploaded | processed | failed
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_documents_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);