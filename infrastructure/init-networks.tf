resource "google_compute_network" "vpc_network" {
  name                    = "${var.gcp_project}-${var.environment}-network"
  auto_create_subnetworks = false
}

resource "google_compute_subnetwork" "default" {
  name          = "${var.gcp_project}-${var.environment}-subnet"
  ip_cidr_range = "10.0.1.0/24"
  region        = var.gcp_region
  network       = google_compute_network.vpc_network.id
}

# allow access by ssh
module "firewall_rules" {
  depends_on = [ google_compute_network.vpc_network ]
  source       = "terraform-google-modules/network/google//modules/firewall-rules"
  version      = "7.3.0"
  project_id   = var.gcp_project
  network_name = "${var.gcp_project}-${var.environment}-network"

  rules = [{
    name                    = "allow-ssh-ingress"
    description             = "Allow default ssh access"
    direction               = "INGRESS"
    priority                = 1000
    destination_ranges      = ["10.0.0.0/8"]
    source_ranges           = ["0.0.0.0/0"]
    allow = [{
      protocol = "tcp"
      ports    = ["22"]
    }]
    deny = []
    log_config = {
      metadata = "INCLUDE_ALL_METADATA"
    }
  }]
}

# allow access to internet
resource "google_compute_router" "nat-router" {
  depends_on = [ google_compute_network.vpc_network ]
  name    = "${var.gcp_project}-${var.environment}-nat-router-${var.gcp_region}"
  region  = var.gcp_region
  network  = "${var.gcp_project}-${var.environment}-network"
}

resource "google_compute_router_nat" "nat-config" {
  depends_on = [ google_compute_router.nat-router ]
  name                               = "${var.gcp_project}-${var.environment}-nat-config"
  router                             = "${google_compute_router.nat-router.name}"
  region                             = var.gcp_region
  nat_ip_allocate_option             = "AUTO_ONLY"
  source_subnetwork_ip_ranges_to_nat = "ALL_SUBNETWORKS_ALL_IP_RANGES"
}

# to increase speed of ssh 
# https://cloud.google.com/iap/docs/using-tcp-forwarding#increasing_the_tcp_upload_bandwidth