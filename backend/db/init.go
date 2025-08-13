package db

import (
	"log"

	"github.com/FoldFunc/Chatter/backend/models"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

var DB *gorm.DB

func init() {
    var err error
    DB, err = gorm.Open(sqlite.Open("app.db"), &gorm.Config{})
    if err != nil {
        log.Fatal("Failed to connect database:", err)
    }
	DB.AutoMigrate(&models.User{})
}

