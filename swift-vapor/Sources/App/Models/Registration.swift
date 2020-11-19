import Fluent
import Vapor

final class Registration: Model, Content {
  static let schema = "registrations"

  @ID()
  var id: UUID?

  @Field(key: "service_id")
  var serviceID: String?

  @Field(key: "node_id")
  var nodeID: String?

  init(serviceID: String, nodeID: String) {
    self.serviceID = serviceID
    self.nodeID = nodeID
  }

  init() {}
}
