-- public."user" definition

-- Drop table

-- DROP TABLE public."user";

CREATE TABLE public."user" (
	id uuid NOT NULL DEFAULT gen_random_uuid(), -- 用户id
	phone varchar(15) NULL, -- 用户手机号
	"token" varchar(64) NULL, -- 用户第三方登录token
	create_time timestamptz(0) NOT NULL DEFAULT now(), -- 创建时间
	modify_time timestamptz(0) NOT NULL DEFAULT now(), -- 更新时间
	CONSTRAINT user_pk PRIMARY KEY (id)
);
CREATE UNIQUE INDEX user_phone_idx ON public."user" USING btree (phone);
CREATE UNIQUE INDEX user_token_idx ON public."user" USING btree (token);
COMMENT ON TABLE public."user" IS '用户信息表。用户注册时会插入记录';

-- Column comments

COMMENT ON COLUMN public."user".id IS '用户id';
COMMENT ON COLUMN public."user".phone IS '用户手机号';
COMMENT ON COLUMN public."user"."token" IS '用户第三方登录token';
COMMENT ON COLUMN public."user".create_time IS '创建时间';
COMMENT ON COLUMN public."user".modify_time IS '更新时间';
