import Fluent
import Vapor

final class Track: Model, Content {
  static let schema = "tracks"

  @ID()
  var id: UUID?

  @Field(key: "key_path")
  var keyPath: String

  @Field(key: "service_id")
  var serviceID: String

  @Field(key: "output_value")
  var outputValue: String

  init(keyPath: String, serviceID: String, outputValue: String) {
    self.keyPath = keyPath
    self.serviceID = serviceID
    self.outputValue = outputValue
  }

  init() {}
}
