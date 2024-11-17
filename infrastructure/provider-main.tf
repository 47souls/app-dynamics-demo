terraform {
  required_version = "~> 1.0"

  required_providers {
    google = {
      source = "hashicorp/google"
      version = "4.11.0"
    }
    docker = {
      source  = "kreuzwerker/docker"
      version = "3.0.2"
    }
  }

  backend "local" {
    path = ".state/terraform.tfstate"
  }
}

provider "docker" {
  host = "unix:///var/run/docker.sock"
}

provider "google" {
  credentials = file(var.gcp_auth_file)
  project     = var.gcp_project
  region      = var.gcp_region
  zone        = var.gcp_zone
}