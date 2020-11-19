import Fluent
import Vapor

final class Trace: Model, Content {
  static let schema = "traces"

  @ID()
  var id: UUID?

  @Field(key: "service_id")
  var serviceID: String

  @Field(key: "client_id")
  var clientID: String?

  @Field(key: "node_id")
  var nodeID: String?

  @Field(key: "request_id")
  var requestID: String?

  @Field(key: "key_path")
  var keyPath: String

  @Field(key: "output_value")
  var outputValue: String

  @Field(key: "created_at")
  var createdAt: Date

  init(
    serviceID: String, clientID: String?, nodeID: String?, requestID: String?, keyPath: String,
    outputValue: String, createdAt: Date
  ) {
    self.serviceID = serviceID
    self.clientID = clientID
    self.nodeID = nodeID
    self.requestID = requestID
    self.keyPath = keyPath
    self.outputValue = outputValue
    self.createdAt = createdAt
  }

  init() {}
}
