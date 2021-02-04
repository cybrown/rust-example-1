-- Your SQL goes here

CREATE TABLE public.users (
	id serial NOT NULL,
	"name" varchar NOT NULL,
	CONSTRAINT users_pkey PRIMARY KEY (id)
);
