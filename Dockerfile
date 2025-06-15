FROM ghcr.io/astral-sh/uv:python3.13-bookworm-slim

WORKDIR /app
COPY ./pyproject.toml ./uv.lock ./

RUN --mount=type=ssh uv venv \
    && uv sync --frozen

COPY ./main.py /app/main.py

WORKDIR /app

ENV PYTHONUNBUFFERED=1

CMD uv run main.py
