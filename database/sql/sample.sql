-- Insert sample data
insert into todos (id, title, description) values ('01GDT91DZ0FDZ7YJ426PB189V1', 'todo 1', 'init test data.');
insert into todos (id, title, description) values ('01GDT91MB0SGG49T974GX2A5G9', 'todo 2', '');
update todos set status_id = '01GE4ZQPD3V5AYHZ4WFWHV9Y9S', updated_at = current_timestamp where id = '01GDT91DZ0FDZ7YJ426PB189V1';
