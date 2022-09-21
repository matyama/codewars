# SQL Kata

## 4 kyu
- [Calculating Month-Over-Month Percentage Growth Rate](https://www.codewars.com/kata/589e0837e10c4a1018000028)
	- Exercise involving multiple `WITH` query, `LAG` window function,
    `GROUP BY`, and `ORDER BY`
  - Implemented in script [`monthly_growth_rate`](scpripts/monthly_growth_rate.sql)
- [Challenge: Two actors who cast together the most](https://www.codewars.com/kata/5818bde9559ff58bd90004a2)
	- Exercise involving multiple `WITH` queries, `JOIN`s,
    `GROUP BY`, `ORDER BY` and `LIMIT`
  - Implemented in script [`most_coacting_actors`](scpripts/most_coacting_actors.sql)
- [Dealing With Messy Data](https://www.codewars.com/kata/5821ee33ec380124f1000013)
	- Exercise involving `CREATE FUNCTION`, `OUTER JOIN`, `regexp_match`
    function, `lower` and `CONCAT`, and `GROUP BY`
  - Implemented in script [`messy_data`](scpripts/messy_data.sql)
- [Sum over a database](https://www.codewars.com/kata/609a6ab739660a0056fb4a29)
	- Exercise involving complex `CREATE FUNCTION` with `FOR` condition
    query acting on system tables and `LOOP` body with `EXECUTE`
    statement
  - Implemented in script [`sum_database`](scpripts/sum_database.sql)
- [Challenge: Transpose two-dimensional arrays](https://www.codewars.com/kata/592b1e4c96cc12de1e0000b1)
	- Exercise involving multiple `WITH` queries, `generate_subscripts`
    function, `GROUP BY` with `ARRAY_AGG`, and `ORDER BY`
  - Implemented in script [`transpose_matrices`](scpripts/transpose_matrices.sql)

## 5 kyu
- [Calculating Running Total](https://www.codewars.com/kata/589cf45835f99b2909000115)
	- Basic exercise involving `WITH`, `GROUP BY`, `ORDER BY` and a window
    function to calculate a cumulative `SUM`
  - Implemented in script [`running_total`](scpripts/running_total.sql)
- [Count Weekdays](https://www.codewars.com/kata/58241d05e7a162c5b100010f)
	- Basic exercise involving `CREATE FUNCTION`, `generate_series`
    function, `LEAST` and `GREATEST`, and `EXTRACT` an `isodow`
  - Implemented in script [`count_weekdays`](scpripts/count_weekdays.sql)
- [Counting overlapping events](https://www.codewars.com/kata/5977b6641e250a661a0000f0)
	- Basic exercise involving `JOIN`, `ORDER BY` and `COUNT` as a window
    function
  - Implemented in script [`overlapping_events`](scpripts/overlapping_events.sql)
- [Enumerate consecutive runs](https://www.codewars.com/kata/5f42c19b4c2cc4001037e7cd)
	- Basic exercise involving `WITH` query, window functions (`LAG`,
    `SUM` with conditional `CASE`)
  - Implemented in script [`consecutive_runs`](scpripts/consecutive_runs.sql)
- [Relational division: Find all movies two actors cast in together](https://www.codewars.com/kata/5817b124e7f4576fd00020a2)
	- Basic exercise involving `JOIN`, `GROUP BY` with `HAVING` and
    `ORDER BY`
  - Implemented in script [`relational_division`](scpripts/relational_division.sql)
- [SQLonacci sequence](https://www.codewars.com/kata/59821d485a49f4d71f00000b)
	- Basic exercise involving `WITH RECURSIVE`
  - Implemented in script [`fibonacci_sequence`](scpripts/fibonacci_sequence.sql)
- [SQL Basics: Group By Day](https://www.codewars.com/kata/5811597e9d278beb04000038)
	- Basic exercise involving `GROUP BY`, `ORDER BY` and `DATE`
  - Implemented in script [`basic_group_by_day`](scpripts/basic_group_by_day.sql)
- [SQL Basics: Simple Hierarchical structure](https://www.codewars.com/kata/5812a2a2492760dfca000450)
	- Basic exercise involving `WITH RECURSIVE`
  - Implemented in script [`basic_hierarchical_structure`](scpripts/basic_hierarchical_structure.sql)
- [SQL Basics: Simple PIVOTING data](https://www.codewars.com/kata/58126aa90ea99769e7000119)
	- Basic exercise involving `crosstab` function
  - Note: use of `crosstab` function is required for this Kata
  - Implemented in script [`basic_pivoting`](scpripts/basic_pivoting.sql)
- [SQL Basics: Simple VIEW](https://www.codewars.com/kata/5811527d9d278b242f000006)
	- Basic exercise involving `CRAETE VIEW`, `JOIN`, sub-query in `WHERE`,
    `GROUP BY` with `HAVING`, and `ORDER BY`
  - Implemented in script [`basic_view`](scpripts/basic_view.sql)
- [SQL Statistics: MIN, MEDIAN, MAX](https://www.codewars.com/kata/58167fa1f544130dcf000317)
	- Basic exercise involving `MIN`, `MAX` and `PERCENTILE_COUNT`
  - Implemented in script [`stats_min_median_max`](scpripts/stats_min_median_max.sql)
- [SQL Tuning: Function Calls](https://www.codewars.com/kata/581fb63e70ca28d92500000d)
	- Basic exercise involving `JOIN` with sub-query, and `ORDER BY`
  - Implemented in script [`tuning_function_calls`](scpripts/tuning_function_calls.sql)
- [Using LATERAL JOIN To Get Top N per Group](https://www.codewars.com/kata/5820176255c3d23f360000a9)
	- Basic exercise involving `LATERAL JOIN` and `ORDER BY`
  - Implemented in script [`topn_per_group_lateral_join`](scpripts/topn_per_group_lateral_join.sql)
- [Using Window Functions To Get Top N per Group](https://www.codewars.com/kata/582001237a3a630ce8000a41)
	- Basic exercise involving `OUTER JOIN`, sub-query, `DENSE_RANK` over
    a `WINDOW`, and `ORDER BY`
  - Implemented in script [`topn_per_group`](scpripts/topn_per_group.sql)
- [Vector addition](https://www.codewars.com/kata/6122439b01ef9f00089b299a)
	- Basic exercise involving `CREATE FUNCTION`, `CREATE OPERATOR`,
    `unnest`, `ARRAY_AGG`, `array_remove` function and `UNION ALL`
  - Implemented in script [`vector_addition`](scpripts/vector_addition.sql)

## 6 kyu
- [Analyzing the sales by product and date](https://www.codewars.com/kata/5dac87a0abe9f1001f39e36d)
	- Basic exercise involving `JOIN`, `GROUP BY` with `ROLLUP` and `SUM`,
    and `EXTRACT` from a date
  - Implemented in script [`sales_analysis`](scpripts/sales_analysis.sql)
- [Calculating Batting Average](https://www.codewars.com/kata/5994dafcbddc2f116d000024)
	- Basic exercise involving `WHERE`, `ORDER BY` and `ROUND`
  - Implemented in script [`batting_average`](scpripts/batting_average.sql)
- [Conditional Count](https://www.codewars.com/kata/5816a3ecf54413a113000074)
	- Basic exercise involving `GROUP BY`, `COUNT` with `FILTER`, and
    `ORDER BY`
  - Implemented in script [`conditional_count`](scpripts/conditional_count.sql)
- [Count IP Addresses](https://www.codewars.com/kata/526989a41034285187000de4)
	- Basic exercise involving `LATERAL` join, `string_to_array` function,
    and `unnest` with `ORDINALITY`
  - Implemented in script [`count_ip_addresses`](scpripts/count_ip_addresses.sql)
- [Present JSON data the SQL way](https://www.codewars.com/kata/5daf515c3affec002b2fb921)
	- Basic exercise involving `CRATE TYPE`, JSON support, `ORDER BY`, and
    conditionals (`CASE`, `COALESCE`)
  - Implemented in script [`present_json_data`](scpripts/present_json_data.sql)
- [Present XML data the SQL way](https://www.codewars.com/kata/5db039743affec0027375de0)
	- Basic exercise involving `XMLTABLE` and XML support in general,
    `ORDER BY`, and conditionals (`CASE`, `COALESCE`)
  - Implemented in script [`present_xml_data`](scpripts/present_xml_data.sql)
- [SELECT prime numbers](https://www.codewars.com/kata/59be9f425227ddd60c00003b)
	- Basic exercise involving `WITH` query, `UNION ALL`, `WHERE` with a
    sub-query and `EXISTS`
  - Implemented in script [`prime_numbers`](scpripts/prime_numbers.sql)
- [SQL: Regex AlphaNumeric Split](https://www.codewars.com/kata/594257d4db68b6e99200002c)
	- Basic exercise involving `regexp_replace` function
  - Implemented in script [`regexp_split`](scpripts/regexp_split.sql)
- [SQL Basics: Create a FUNCTION (DATES)](https://www.codewars.com/kata/5811010104adbba24b0002fe)
	- Basic exercise involving `CREATE FUNCTION` and `AGE`
  - Implemented in script [`basic_create_fn`](scpripts/basic_create_fn.sql)
- [SQL Basics - Monsters using CASE](https://www.codewars.com/kata/593ef0e98b90525e090000b9)
	- Basic exercise involving `JOIN`, `ORDER BY` and `CASE`
  - Implemented in script [`basic_monsters_case`](scpripts/basic_monsters_case.sql)
- [SQL Basics: Simple EXISTS](https://www.codewars.com/kata/58113a64e10b53ec36000293)
	- Basic exercise involving `WHERE` and `EXISTS` with a sub-select
  - Implemented in script [`basic_exists`](scpripts/basic_exists.sql)
- [SQL Basics: Simple FULL TEXT SEARCH](https://www.codewars.com/kata/581676828906324b8b00059e)
	- Basic exercise involving `to_tsvector` and `to_tsquery`
  - Implemented in script [`basic_fulltext_search`](scpripts/basic_fulltext_search.sql)
- [SQL Basics: Simple HAVING](https://www.codewars.com/users/matyama/completed_solutions)
	- Basic exercise involving `GROUP BY` with `HAVING`
  - Implemented in script [`basic_having`](scpripts/basic_having.sql)
- [SQL Basics: Simple IN](https://www.codewars.com/kata/58113c03009b4fcc66000d29)
	- Basic exercise involving `WHERE` with `IN` and a sub-select
  - Implemented in script [`basic_in`](scpripts/basic_in.sql)
- [SQL Basics: Simple JOIN and RANK](https://www.codewars.com/kata/58094559c47d323ebd000035)
	- Basic exercise involving `JOIN`, `GROUP BY` and `DENSE_RANK`
  - Implemented in script [`basic_join_rank`](scpripts/basic_join_rank.sql)
- [SQL Basics: Simple NULL handling](https://www.codewars.com/kata/5811315e04adbbdb5000050e)
	- Basic exercise involving `COALESCE` and `NULLIF`
  - Implemented in script [`basic_null_handling`](scpripts/basic_null_handling.sql)
- [SQL Basics: Simple PIVOTING data WITHOUT CROSSTAB](https://www.codewars.com/kata/5982020284a83baf2f00001c)
	- Basic exercise involving `JOIN`, `GROUP BY`, `ORDER BY`, and `COUNT`
    with `CASE`
  - Note: use of `crosstab` function is disallowed for this Kata
  - Implemented in script [`basic_pivot_no_crosstab`](scpripts/basic_pivot_no_crosstab.sql)
- [SQL Basics: Simple table totaling](https://www.codewars.com/kata/5809575e166583acfa000083)
  - Basic exercise on aggregation (`GROUB BY`, `SUM`, `COUNT`),
    windowing (`RANK`) and conditionals (`COALESCE`, `NULLIF`)
  - Implemented in script [`basic_table_totaling`](scpripts/basic_table_totaling.sql)
- [SQL Basics: Simple UNION ALL](https://www.codewars.com/kata/58112f8004adbbdb500004fe)
	- Basic exercise involving `UNION ALL` and `ORDER BY`
  - Implemented in script [`basic_union_all`](scpripts/basic_union_all.sql)
- [SQL Basics: Simple WITH](https://www.codewars.com/kata/5811501c2d35672d4f000146)
	- Basic exercise involving `WITH` and `WHERE` with sub-select
  - Implemented in script [`basic_with`](scpripts/basic_with.sql)
- [SQL Basics: Top 10 customers by total payments amount](https://www.codewars.com/kata/580d08b5c049aef8f900007c)
	- Basic exercise involving `JOIN`, `GROUP BY`, `ORDER BY` and `LIMIT`
  - Implemented in script [`basic_top10`](scpripts/basic_top10.sql)
- [SQL Bug Fixing: Fix the JOIN](https://www.codewars.com/kata/580fb94e12b34dd1c40001f0)
	- Basic exercise on bug fixing a query with `JOIN`
  - Implemented in script [`bugfix_join`](scpripts/bugfix_join.sql)
- [SQL Bug Fixing: Fix the QUERY - Totaling](https://www.codewars.com/kata/582cba7d3be8ce3a8300007c)
	- Basic exercise on bug fixing a query with `JOIN`, `GROUP BY` and
    `ORDER BY`
  - Implemented in script [`bugfix_totaling`](scpripts/bugfix_totaling.sql)
- [Subqueries master](https://www.codewars.com/kata/594323fde53209e94700012a)
	- Basic exercise involving a sub-query with string and array functions
    (`regexp_split_to_array`, `array_to_string`, `array_length`)
  - Implemented in script [`bugfix_join`](scpripts/bugfix_join.sql)

## 7 kyu
- [Countries Capitals for Trivia Night (SQL for Beginners #6)](https://www.codewars.com/kata/5e5f09dc0a17be0023920f6f)
	- Basic exercise involving `SIMILAR TO`, `LIKE`, `ORDER BY`, and
    `LIMIT`
  - Implemented in script [`beginners_capitals`](scpripts/beginners_capitals.sql)
- [Easy SQL: Counting and Grouping](https://www.codewars.com/kata/594633020a561e329a0000a2)
	- Basic exercise involving `GROUP BY`, `COUNT` and `ORDER BY`
  - Implemented in script [`easy_counting_grouping`](scpripts/easy_counting_grouping.sql)
- [GROCERY STORE: Logistic Optimisation](https://www.codewars.com/kata/5a8ec692b17101bfc70001ba)
	- Basic exercise involving `GROUP BY` and `ORDER BY`
  - Implemented in script [`grocery_store`](scpripts/grocery_score.sql)
- [SQL Basics - Position](https://www.codewars.com/kata/59401e0e54a655a298000040)
	- Basic exercise involving `ORDER BY` and `POSITION`
  - Implemented in script [`basic_position`](scpripts/basic_position.sql)
- [SQL Basics: Simple GROUP BY](https://www.codewars.com/kata/58111f4ee10b5301a7000175)
	- Basic exercise involving `GROUP BY` and `COUNT`
  - Implemented in script [`basic_group_by`](scpripts/basic_group_by.sql)
- [SQL Basics: Simple JOIN with COUNT](https://www.codewars.com/kata/580918e24a85b05ad000010c)
	- Basic exercise involving `JOIN`, `GROUP BY` and `COUNT`
  - Implemented in script [`basic_join_count`](scpripts/basic_join_count.sql)
- [SQL Basics: Truncating](https://www.codewars.com/kata/594a8fa5a2db9e5f290000c3)
	- Basic exercise involving `TRUC`
  - Implemented in script [`basic_trunc`](scpripts/basic_trunc.sql)
- [SQL easy regex extraction](https://www.codewars.com/kata/5c0ae69d5f72394e130025f6)
	- Basic exercise involving `substring`
  - Implemented in script [`regex_extract`](scpripts/regex_extract.sql)
- [SQL with LOTR: Elven Wildcards](https://www.codewars.com/kata/5ad90fb688a0b74111000055)
	- Basic exercise involving `LIKE`, and `INITCAP` and `CONCAT`
    functions
  - Implemented in script [`elven_wildcards`](scpripts/elven_wildcards.sql)
- [SQL with Pokemon: Damage Multipliers](https://www.codewars.com/kata/5ab828bcedbcfc65ea000099)
	- Basic exercise involving `JOIN` and `ORDER BY`
  - Implemented in script [`pokemon`](scpripts/pokemon.sql)
- [SQL with Street Fighter: Total Wins](https://www.codewars.com/kata/5ac698cdd325ad18a3000170)
	- Basic exercise involving `JOIN`, `GROUP BY`, `ORDER BY`, and `LIMIT`
  - Implemented in script [`street_fighter`](scpripts/street_fighter.sql)

## 8 kyu
- [Adults only (SQL for Beginners #1)](https://www.codewars.com/kata/590a95eede09f87472000213)
 - Trivial exercise implemented in script [`beginners_adults_only`](scripts/beginners_adults_only.sql)
- [On the Canadian Border (SQL for Beginners #2)](https://www.codewars.com/kata/590ba881fe13cfdcc20001b4)
 - Trivial exercise implemented in script [`beginners_canadian_border`](scripts/beginners_canadian_border.sql)
