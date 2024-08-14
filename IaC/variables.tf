# Our service name - this is used as prefix for most of the resource names
variable "service_name" {
  default = "kyykka_api"
}

# AWS region
variable "region" {
  default = "eu-north-1"
}

variable "stage" {
  type = string
}

variable "log_retention_in_days" {
  type = number
  default = 30
}

variable "log_level" {
  type = string
  default = "info"
}
