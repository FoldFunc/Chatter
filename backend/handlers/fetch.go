package handlers

import (
	"encoding/json"
	"log"

	"github.com/FoldFunc/Chatter/backend/db"
	"github.com/FoldFunc/Chatter/backend/models"
	"github.com/gofiber/fiber/v3"
	"github.com/gofiber/fiber/v3/middleware/session"
)

func FetchName(c fiber.Ctx, store *session.Store) error {
	log.Println("Called FetchName")

	// Parse request body into models.Name
	name := new(models.Name)
	if err := json.Unmarshal(c.Body(), name); err != nil {
		log.Println("Error while parsing the request:", err.Error())
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Failed to parse the body",
		})
	}

	// Get session
	_, err := store.Get(c)
	if err != nil {
		log.Println("Error while getting the session:", err.Error())
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Failed to get session",
		})
	}

	// Struct to hold only ID and Name
	var results []struct {
		ID   uint   `json:"id"`
		Name string `json:"name"`
	}

	// Fetch only matching users' IDs and Names
	if err := db.DB.
		Model(&models.User{}).
		Select("id, name").
		Where("name LIKE ?", "%"+name.Name+"%").
		Find(&results).Error; err != nil {

		log.Println("Error while fetching the database:", err)
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Failed to fetch from database",
		})
	}

	// Return JSON array of {id, name}
	return c.Status(fiber.StatusOK).JSON(results)
}
