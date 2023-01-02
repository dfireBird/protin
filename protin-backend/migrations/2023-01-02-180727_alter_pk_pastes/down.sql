ALTER TABLE public.pastes RENAME COLUMN id TO file;
ALTER TABLE public.pastes RENAME COLUMN file_path TO id;
ALTER TABLE public.pastes DROP CONSTRAINT pastes_pkey;
ALTER TABLE public.pastes ADD CONSTRAINT pastes_pkey PRIMARY KEY (id);
