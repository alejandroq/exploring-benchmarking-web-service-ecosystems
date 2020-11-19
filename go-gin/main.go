package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/jmoiron/sqlx"
	"github.com/joho/godotenv"

	_ "github.com/go-sql-driver/mysql"
)

const serviceName = "test_service_name"
const nodeID = "temp_node_id"

//Track ...
type Track struct {
	KeyPath     string `json:"keyPath" db:"key_path"`
	OutputValue string `json:"outputValue" db:"output_value"`
}

func main() {
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	db, err := sqlx.Connect("mysql", os.Getenv("DATABASE_URL"))
	if err != nil {
		log.Fatal(err)
	}
	db.SetConnMaxLifetime(time.Minute * 3)
	db.SetMaxOpenConns(20)
	db.SetMaxIdleConns(20)

	router := gin.Default()
	router.
		GET("health", func(c *gin.Context) {
			c.JSON(200, gin.H{
				"message": "healthy",
			})
		}).
		GET("/tracks/:keyPath", func(c *gin.Context) {
			keyPath := c.Param("keyPath")
			clientID := c.GetHeader("x-client-id")
			requestID := c.GetHeader("x-request-id")

			var track Track

			err := db.Get(&track,
				`
				SELECT
					key_path,
					output_value
				FROM
					tracks
				WHERE
					key_path = ?
					AND service_id = ?
				LIMIT 1
				`,
				keyPath,
				serviceName,
			)

			if err != nil && strings.Contains(err.Error(), "no rows in result set") {
				c.JSON(404, gin.H{
					"error":  true,
					"reason": "resource not found",
				})
				return
			}

			if err != nil {
				c.JSON(500, gin.H{
					"error":  true,
					"reason": err.Error(),
				})
				return
			}

			go func() {
				_, err = db.ExecContext(
					c,
					`
					INSERT INTO
						traces
					VALUES
					(
							?,
							?,
							?,
							?,
							?,
							?,
							NOW()
					)
					`,
					serviceName,
					clientID,
					nodeID,
					requestID,
					track.KeyPath,
					track.OutputValue,
				)
				if err != nil {
					fmt.Println(err)
				}
			}()

			c.JSON(200, track)
		})
	router.Run(":8080")
}
