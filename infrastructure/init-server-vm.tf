data "template_file" "linux-metadata" {
template = <<EOF
sudo apt-get update; 
sudo apt-get --assume-yes install ca-certificates curl gnupg;
sudo install -m 0755 -d /etc/apt/keyrings;
curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg;
sudo chmod a+r /etc/apt/keyrings/docker.gpg;
echo \
  "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \
  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null;
sudo apt-get update
sudo apt-get --assume-yes install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin;
sudo usermod -aG docker akliuiko
EOF
}

resource "google_compute_instance" "default" {
  name         = "app-dynamics-${var.environment}-server"
  machine_type = var.server_machine_type
  zone         = var.gcp_zone
  tags         = ["ssh"]

  boot_disk {
    initialize_params {
      image = var.linux_instance_type
    }
  }

  metadata_startup_script = data.template_file.linux-metadata.rendered
  
  network_interface {
    subnetwork = google_compute_subnetwork.default.id
  }
}
