import Fluent
import FluentMySQLDriver
import Vapor

// configures your application
public func configure(_ app: Application) throws {
  app.databases.use(
    .mysql(
      hostname: Environment.get("DATABASE_HOST") ?? "localhost",
      port: Environment.get("DATABASE_PORT").flatMap(Int.init(_:))
        ?? MySQLConfiguration.ianaPortNumber,
      username: Environment.get("DATABASE_USERNAME") ?? "vapor_username",
      password: Environment.get("DATABASE_PASSWORD") ?? "vapor_password",
      database: Environment.get("DATABASE_NAME") ?? "vapor_database",
      tlsConfiguration: .forClient(certificateVerification: .none),
      maxConnectionsPerEventLoop: 30
    ), as: .mysql)

  // register routes
  try routes(app)
}
