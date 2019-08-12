V 1.0.0
Initial release.

V 1.0.1
*Fixed a HTTP 500 server error. When a server was added that did not exist unwrap was called on a None Option type. The application will now redirect to an error template with a message.