package models

import "gorm.io/gorm"
type Name struct {
	Name string `json:"name"`
}
type User struct {
	gorm.Model
	Name     string `json:"name"`
	Email    string `json:"email"`
	Password string `json:"password`
	LoggedIn bool `gorm:"default:false"`
}
