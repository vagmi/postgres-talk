# PostgreSQL experiments

Do you know that postgresql can do some awesome things? This repo explores some
of the awesome capabilities of PostgreSQL in real world scenarios.

Ths repository is used as supplementary material to my ConFoo 2023 talk.

We do this and more such awesome work at [Tarka Labs](https://tarkalabs.com/).

## Pivot Table

This example uses the `tablefunc` extension with the crosstab method to cross
tabulate results across two dimensions.

```
cargo run --bin pivot_table
psql -d pivot_table -f src/pivot_table/sql/00_query_simple.sql
psql -d pivot_table -f src/pivot_table/sql/01_query_crosstab.sql
```

## Window functions

Window functions perform aggregation on a set of rows without having to use
group by. In this example we use a model that has videos and views and 
try to show the view statistics inline.

```
cargo run --bin window_functions
psql -d window_functions -f src/window_functions/sql/query.sql
```

## Lateral Joins

Lateral joins are the equivalent of for each row run a query. But since postgresql
is handling it, it is often more peformant than the application code.

```
cargo run --bin lateral_join

psql -d lateral_join -f src/lateral_join/sql/00_query_visits.sql
psql -d lateral_join -f src/lateral_join/sql/01_query_conversions.sql
psql -d lateral_join -f src/lateral_join/sql/02_query_conversions_agg.sql
```

## JSON Operations

This example uses the [OMDB postgresql](https://github.com/credativ/omdb-postgresql)
repo to setup the database. I constructs the JSON response directly from the database.

```
cargo build --release --bin json_ops &&  target/release/json_ops

http localhost:3000/movies/603
```

## Full Text Search

This continues the example from the json ops and generates a full text search on
both the title and the abstract

```
http localhost:3000/movies/search?q=shawshank
```

## Row Level security

This example sets up a users table and a set of roles at the postgresql level.
We then use the combination of the postgresql role and the `set_config` &
`current_setting` to establish a user context per transaction to apply
user specified rules for the database.

```
cargo build --release --bin rls &&  target/release/rls
```

An anonymous user will not be able to see anything.

```
http localhost:3000/users
```

An admin will be able to see and update anything.

```
http localhost:3000/users user:admin
```

A logged in user will be able to only view themself.

```
http localhost:3000/users user:admin
```

A logged in user will only be able to update their details

```
http localhost:3000/users/id user:<username> email=new@email.com
```
