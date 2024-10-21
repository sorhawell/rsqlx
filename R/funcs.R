

#' Call My Hello World function
#'
#' @return NULL
#' @export
#'
#' @examples
#' say_hello_world()
say_hello_world = function() {
  rsqlx:::hello_world()
}


#' Simple read table
#'
#' @return NULL
#' @export
#' @details
#' This impl creates ad-hoc connection which has significant overhead
#'
#'
#' @examples
#' # none so far
read_table = function(conn_str, name) {
  rsqlx:::r_read_table(conn_str, name)
}
