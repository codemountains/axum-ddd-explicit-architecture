-- Show constraint name
select
    table_name,
    constraint_name,
    constraint_type
from
    information_schema.table_constraints
where
    table_name = 'todos';
