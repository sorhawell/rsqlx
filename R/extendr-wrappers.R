# Generated by extendr: Do not edit by hand

# nolint start

#
# This file was created with the following call:
#   .Call("wrap__make_rsqlx_wrappers", use_symbols = TRUE, package_name = "rsqlx")

#' @usage NULL
#' @useDynLib rsqlx, .registration = TRUE
NULL

r_read_table <- function(conn_str, table) .Call(wrap__r_read_table, conn_str, table)

hello_world <- function() .Call(wrap__hello_world)

RSQLXConnection <- new.env(parent = emptyenv())

RSQLXConnection$new <- function(conn_str) .Call(wrap__RSQLXConnection__new, conn_str)

RSQLXConnection$fetch_qeury <- function(query) .Call(wrap__RSQLXConnection__fetch_qeury, self, query)

RSQLXConnection$fetch_qeury_background <- function(query) .Call(wrap__RSQLXConnection__fetch_qeury_background, self, query)

RSQLXConnection$drop <- function() invisible(.Call(wrap__RSQLXConnection__drop, self))

#' @export
`$.RSQLXConnection` <- function (self, name) { func <- RSQLXConnection[[name]]; environment(func) <- environment(); func }

#' @export
`[[.RSQLXConnection` <- `$.RSQLXConnection`

RSQLXFuture <- new.env(parent = emptyenv())

RSQLXFuture$join <- function() .Call(wrap__RSQLXFuture__join, self)

RSQLXFuture$is_done <- function() .Call(wrap__RSQLXFuture__is_done, self)

#' @export
`$.RSQLXFuture` <- function (self, name) { func <- RSQLXFuture[[name]]; environment(func) <- environment(); func }

#' @export
`[[.RSQLXFuture` <- `$.RSQLXFuture`


# nolint end
