package handlers

import (
	"encoding/json"
	"fmt"
	"log"

	"github.com/FoldFunc/Chatter/backend/db"
	"github.com/FoldFunc/Chatter/backend/models"
	"github.com/gofiber/fiber/v3"
	"github.com/gofiber/fiber/v3/middleware/session"
	"golang.org/x/crypto/bcrypt"
)

// Register handles user registration with password hashing
func Register(c fiber.Ctx) error {
	log.Println("Called Register")

	user := new(models.User)
	if err := json.Unmarshal(c.Body(), user); err != nil {
		log.Println("Error parsing JSON:", err)
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Cannot parse JSON",
		})
	}

	// Hash the password
	hashedPassword, err := bcrypt.GenerateFromPassword([]byte(user.Password), bcrypt.DefaultCost)
	if err != nil {
		log.Println("Error hashing password:", err)
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Failed to hash password",
		})
	}
	user.Password = string(hashedPassword)

	// Save the user in the database
	if err := db.DB.Create(user).Error; err != nil {
		log.Println("Error creating user:", err)
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Failed to create user",
		})
	}

	return c.Status(fiber.StatusCreated).JSON(fiber.Map{
		"message": "Registered successfully",
	})
}

func RequireAuth(c fiber.Ctx, store *session.Store) error {
	fmt.Println("RequireAuth called")
	sess, err := store.Get(c)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": "Failed to get session"})
	}

	userID := sess.Get("user_id")
	if userID == nil {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{"error": "Not logged in"})
	}

	var user models.User
	if err := db.DB.First(&user, userID).Error; err != nil {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{"error": "User not found"})
	}

	return c.Status(fiber.StatusOK).JSON(fiber.Map{
		"message": "Logged in",
	})
}

func Logout(c fiber.Ctx, store *session.Store) error {
	log.Println("Called Logout")

	// Get the session
	sess, err := store.Get(c)
	if err != nil {
		log.Println("Error getting session:", err)
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Failed to get session",
		})
	}

	// Get user ID from session
	userID := sess.Get("user_id")
	if userID != nil {
		// Update the database to set LoggedIn = false
		var user models.User
		if err := db.DB.First(&user, userID).Error; err == nil {
			user.LoggedIn = false
			if err := db.DB.Save(&user).Error; err != nil {
				log.Println("Error updating LoggedIn:", err)
			}
		}
	}

	// Destroy the session cookie
	if err := sess.Destroy(); err != nil {
		log.Println("Error destroying session:", err)
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Failed to destroy session",
		})
	}

	return c.Status(fiber.StatusOK).JSON(fiber.Map{
		"message": "Logged out successfully",
	})
}
func Login(c fiber.Ctx, store *session.Store) error {
	log.Println("Called Login")

	user := new(models.User)
	if err := json.Unmarshal(c.Body(), user); err != nil {
		log.Println("Error parsing JSON:", err)
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Cannot parse JSON",
		})
	}

	userFromDB := new(models.User)
	if err := db.DB.Where("email = ?", user.Email).First(userFromDB).Error; err != nil {
		log.Println("User not found:", err)
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
			"error": "Bad credentials",
		})
	}

	// Compare the hashed password
	if err := bcrypt.CompareHashAndPassword([]byte(userFromDB.Password), []byte(user.Password)); err != nil {
		log.Println("Error: ", err.Error())
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
			"error": "Bad credentials",
		})
	}

	// Set LoggedIn to true in the database
	userFromDB.LoggedIn = true
	if err := db.DB.Save(userFromDB).Error; err != nil {
		log.Println("Error updating LoggedIn:", err)
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Failed to update login status",
		})
	}

	// Create session
	sess, err := store.Get(c)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Could not create a session cookie",
		})
	}
	sess.Set("user_id", userFromDB.ID)
	if err := sess.Save(); err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Could not save session cookie",
		})
	}

	return c.Status(fiber.StatusOK).JSON(fiber.Map{
		"message": "Logged in",
	})
}
