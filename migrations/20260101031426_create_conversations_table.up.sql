-- Add up migration script here
DROP TABLE IF EXISTS public.conversations;
CREATE TABLE public.conversations(
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    title TEXT NULL,
    summary TEXT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_conversations_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);