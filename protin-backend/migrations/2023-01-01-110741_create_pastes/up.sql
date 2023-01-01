CREATE TABLE pastes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    expires_at TIMESTAMP NOT NULL DEFAULT now() + interval '1 day'
)
