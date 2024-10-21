library(RPostgres)

system.time({
  rpostgres_con <- dbConnect(RPostgres::Postgres(),
     , host = 'localhost'
     , port = '5432'
     , dbname = 'mydatabase'
     , user   = 'myuser'
     , password = "mypassword"
  )
})

#RPostgres::dbWriteTable(rpostgres_con,name = "mtcars",mtcars)

system.time({
  RPostgres::dbReadTable(rpostgres_con,name = "mtcars")
})
system.time({
  RPostgres::dbGetQuery(rpostgres_con,"SELECT * FROM mtcars")
})

con_str = "postgres://myuser:mypassword@localhost:5432/mydatabase"
system.time({
  rsqlx_con = unwrap(rsqlx:::RSQLXConnection$new(con_str))
})
system.time({
  res = unwrap(rsqlx_con$fetch_qeury("SELECT * FROM mtcars"))
})


system.time({
  res = unwrap(rsqlx_con$fetch_qeury("SELECT PG_SLEEP(1.5);"))
})


system.time({
  handle = rsqlx_con$fetch_qeury_background("SELECT * FROM mtcars")
  res = unwrap(handle$join())
})

system.time({
  handle1 = rsqlx_con$fetch_qeury_background("SELECT PG_SLEEP(1.5);")
  handle2 = rsqlx_con$fetch_qeury_background("SELECT PG_SLEEP(1.5);")
  handle3 = rsqlx_con$fetch_qeury_background("SELECT PG_SLEEP(1.5);")
  res1 = unwrap(handle1$join())
  res2 = unwrap(handle2$join())
  res3 = unwrap(handle3$join())
})

