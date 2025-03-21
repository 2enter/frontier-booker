SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: cargo_status; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cargo_status AS ENUM (
    'shipping',
    'delivered',
    'launched'
);


--
-- Name: cargo_type; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cargo_type AS ENUM (
    'water',
    'spring',
    'stair',
    'star',
    'cake',
    'diamond'
);


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: cargo; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cargo (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    paint_time integer NOT NULL,
    type public.cargo_type NOT NULL,
    status public.cargo_status DEFAULT 'shipping'::public.cargo_status NOT NULL,
    name character varying(255),
    description character varying(255),
    pending boolean DEFAULT false
);


--
-- Name: news; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.news (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    title text NOT NULL,
    hype integer DEFAULT 0 NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.schema_migrations (
    version character varying(128) NOT NULL
);


--
-- Name: news news_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.news
    ADD CONSTRAINT news_pkey PRIMARY KEY (id);


--
-- Name: cargo payload_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cargo
    ADD CONSTRAINT payload_pkey PRIMARY KEY (id);


--
-- Name: schema_migrations schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- PostgreSQL database dump complete
--


--
-- Dbmate schema migrations
--

INSERT INTO public.schema_migrations (version) VALUES
    ('20250202075456'),
    ('20250202080539'),
    ('20250202081403'),
    ('20250202090306'),
    ('20250202090911'),
    ('20250202113745'),
    ('20250211075035'),
    ('20250316071646'),
    ('20250316072934'),
    ('20250320094612'),
    ('20250320095456');
