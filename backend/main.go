package main

import (
	"log"

	"github.com/FoldFunc/Chatter/backend/handlers"
	"github.com/gofiber/fiber/v3"
	"github.com/gofiber/fiber/v3/middleware/cors"
	"github.com/gofiber/fiber/v3/middleware/session"
)

var store *session.Store

func main() {
	store = session.NewStore(session.Config{
		CookieHTTPOnly: true,
		CookieSecure:   false,
		CookieSameSite: "Lax",
	})
	server := fiber.New()
	server.Use(cors.New(cors.Config{
		AllowOrigins:     []string{"http://localhost:5173"},
		AllowCredentials: true,
	}))
	server.Post("/api/register", handlers.Register)
	server.Post("/api/login", func(c fiber.Ctx) error {
		return handlers.Login(c, store)
	})
	server.Get("/api/profile", func(c fiber.Ctx) error {
		return handlers.RequireAuth(c, store)
	})
	server.Get("/api/logout", func(c fiber.Ctx) error {
		return handlers.Logout(c, store)
	})
	server.Post("/api/fetch/name", func(c fiber.Ctx) error {
		return handlers.FetchName(c, store)
	})

	log.Println("Server running on port: 42069")
	log.Fatal(server.Listen(":42069"))
}
