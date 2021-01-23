-- Your SQL goes here

CREATE TABLE public.posts (
	id serial NOT NULL,
	title varchar NOT NULL,
	body text NOT NULL,
	published bool NOT NULL DEFAULT false,
	CONSTRAINT posts_pkey PRIMARY KEY (id)
);
