-- Add up migration script here
DROP TABLE IF EXISTS public.messages;
CREATE TABLE public.messages(
    id UUID PRIMARY KEY,
    conversation_id UUID NOT NULL,
    role VARCHAR(100),
    content TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_messages_conversation
        FOREIGN KEY (conversation_id)
        REFERENCES conversations(id)
        ON DELETE CASCADE
);