# -*- mode: ruby -*-
# vi: set ft=ruby ts=2 sw=2 expandtab :

PROJECT = "city-builder"

ENV['VAGRANT_NO_PARALLEL'] = 'yes'
ENV['VAGRANT_DEFAULT_PROVIDER'] = 'docker'
Vagrant.configure(2) do |config|

  config.ssh.insert_key = false
  config.vm.define "dev", primary: true do |app|
    app.vm.provider "docker" do |d|
      d.image = "jean553/rust-dev-docker"
      d.name = "#{PROJECT}_dev"
      d.has_ssh = true
      d.env = {
        "HOST_USER_UID" => Process.euid,
      }
    end
    app.ssh.username = "vagrant"
  end
end
