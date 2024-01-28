# Используем образ Golang в качестве базового образа
FROM rust:latest

# Копируем файлы проекта внутрь контейнера
COPY . /app

# Переходим в директорию с проектом
WORKDIR /app

# Собираем исполняемый файл
#RUN cargo install --path /src
#RUN rustc ./src/main.rs -o ./main

# Запускаем приложение при старте контейнера
#CMD ["./main"]

RUN cargo run
