ALTER TABLE public.pastes RENAME COLUMN id TO file_path;
ALTER TABLE public.pastes RENAME COLUMN file TO id;
ALTER TABLE public.pastes DROP CONSTRAINT pastes_pkey;
ALTER TABLE public.pastes ADD CONSTRAINT pastes_pkey PRIMARY KEY (id);
