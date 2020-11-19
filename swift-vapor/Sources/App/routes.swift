import Fluent
import Vapor

func routes(_ app: Application) throws {
  app.get("health") { _ -> Health in
    return Health()
  }

  try app.register(
    collection: TrackController(
      serviceID: "test_service_name",
      nodeID: UUID().uuidString
    ))
}
