# TODO: for now it does not work, doing that manually
# build server image whenver code changes
# resource "docker_image" "sandwhich_bot_server" {               
#   name = "sandwhich_bot_server"
#   build {  
#     context = "."
#     dockerfile = "Dockerfile"
#   }
#   TODO: watch for ../server
#   triggers = {
#     dir_sha1 = sha1(join("", [for f in fileset(path.module, "src/*") : filesha1(f)]))
#   }
# }