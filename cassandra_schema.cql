CREATE KEYSPACE IF NOT EXISTS lmd_chat WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};
CREATE TABLE lmd_chat.users (
	id uuid,
	username text,
	email text,
	phone text,
	password text,
	display_name text,
	display_image text,
	bio text,
	status tinyint,
	updated_at timestamp,
	created_at timestamp,
	PRIMARY KEY (id, created_at)
) WITH CLUSTERING ORDER BY ( created_at DESC )
AND comment = 'server user table';
CREATE INDEX user_email ON lmd_chat.users (email);
CREATE INDEX user_username ON lmd_chat.users (username);

CREATE TABLE lmd_chat.apps (
	user_id uuid,
	id bigint,
	app_secret text,
	app_name text,
	about text,
	status tinyint,
	updated_at timestamp,
	created_at timestamp,
	PRIMARY KEY (user_id, id)
) WITH CLUSTERING ORDER BY (id DESC);

CREATE TABLE lmd_chat.app_users(
    app_id bigint,
    app_user_id text,
	id uuid,
    display_name text,
    display_image text,
    status tinyint,
    updated_at timestamp,
    created_at timestamp,
    PRIMARY KEY(app_id, app_user_id)
);
CREATE INDEX app_users_id ON lmd_chat.app_users (id);

CREATE TABLE lmd_chat.chat_rooms(
    app_id bigint,
    room_id text,
    room_name text,
    room_owner text,
    app_user_id text,
    room_user_id text,
    banner text,
    about text,
    room_type tinyint,
    is_private boolean,
    unread_msg int,
    last_msg map<text,text>,
    status tinyint,
    updated_at timestamp,
    created_at timestamp,
    PRIMARY KEY(app_id, room_user_id)
);
CREATE INDEX chat_room_room_id ON lmd_chat.chat_rooms (room_id);
CREATE INDEX chat_room_app_user_id ON lmd_chat.chat_rooms (app_user_id);
CREATE INDEX chat_room_type ON lmd_chat.chat_rooms (room_type);

CREATE TABLE IF NOT EXISTS lmd_chat.chat_room_messages(
    app_id bigint,
    room_id text,
    msg_owner text,
    owner_name text,
    msg_id uuid,
    reply_on_id uuid,
    content text,
    url text,
    message_type tinyint,
    system_message boolean,
    read_by_users set<text>,
    status tinyint,
    send_at timestamp,
    updated_at timestamp,
    created_at timestamp,
    PRIMARY KEY(room_id, send_at)
) WITH CLUSTERING ORDER BY (send_at DESC);
CREATE INDEX message_msg_id ON lmd_chat.chat_room_messages (msg_id);
CREATE INDEX message_status ON lmd_chat.chat_room_messages (status);
CREATE INDEX message_msg_owner ON lmd_chat.chat_room_messages (msg_owner);
