FROM gitpod/workspace-full:2024-06-10-10-39-01

USER gitpod

RUN sudo apt-get -q update \
    && sudo apt-get install -yq librust-alsa-sys-dev \
    && sudo rm -rf /var/lib/apt/lists/*