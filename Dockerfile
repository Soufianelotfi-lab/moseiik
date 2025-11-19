FROM rust:1.85-slim
WORKDIR /usr/src/moseiik

# Nous avons télechargé le curl etunzip pourqu'on puisse télechargé les vignettes
RUN apt-get update && apt-get install -y --no-install-recommends \
curl unzip && \
rm -rf /var/lib/apt/lists/*

# Télecharger les vignettes à partir du lien donner dans le readme et les récuperer dans moseiik_test_images.zip
RUN curl -L "https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip" -o moseiik_test_images.zip
# Nous avons dézipé le fichier sur moseiik_test_images
RUN unzip moseiik_test_images.zip -d moseiik_test_images && rm moseiik_test_images.zip

COPY Cargo.toml Cargo.lock ./

# On copie tout le dossier assets, src et tests
COPY assets ./assets
COPY src ./src
COPY tests ./tests
# Nous allons mettre les vignettes dans le répertoire assets
RUN mv moseiik_test_images/* ./assets/

# nous téléchargons les dépendances et on compile en release
RUN cargo build --release

ENTRYPOINT [ "cargo", "test", "--release", "--" ]
