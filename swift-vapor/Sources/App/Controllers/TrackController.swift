import Fluent
import Vapor

struct TrackController: RouteCollection {
  let serviceID: String
  let nodeID: String

  enum Errors: Error {
    case badRequest
  }

  struct TrackResponse: Content {
    let keyPath: String
    let output: String
    static func from(track: Track) -> Self {
      TrackResponse(keyPath: track.keyPath, output: track.outputValue)
    }
  }

  func boot(routes: RoutesBuilder) throws {
    let tracks = routes.grouped("tracks")
    tracks.get(":keyPath", use: getTrackByKeyPath)
  }

  func getTrackByKeyPath(req: Request) -> EventLoopFuture<TrackResponse> {
    guard let keyPath = req.parameters.get("keyPath") else {
      return req.eventLoop.makeFailedFuture(Errors.badRequest)
    }

    return Track.query(on: req.db)
      .filter(\.$keyPath, .equal, keyPath)
      .filter(\.$serviceID, .equal, serviceID)
      .all()
      .map { tracks in tracks.first }
      .flatMap { track in
        guard let track = track else {
          return req.eventLoop.makeFailedFuture(Abort(.notFound, reason: "resouce not found"))
        }
        let response = TrackResponse.from(track: track)
        return Trace(
          serviceID: serviceID,
          clientID: req.headers.first(name: "x-client-id"),
          nodeID: nodeID,
          requestID: req.headers.first(name: "x-request-id"),
          keyPath: keyPath,
          outputValue: track.outputValue,
          createdAt: Date()
        )
        .save(on: req.db)
        .map { response }
      }
  }
}
