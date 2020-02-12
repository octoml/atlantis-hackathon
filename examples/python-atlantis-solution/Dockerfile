FROM python:3.7

RUN apt update && \
    apt install ca-certificates python3-pip python3-venv -y && \
    rm -rf /var/lib/apt/lists/*

RUN pip3 install poetry

WORKDIR /app

COPY poetry.lock pyproject.toml ./

# Project initialization:
RUN poetry config virtualenvs.create false \
    && poetry install --no-interaction --no-ansi

# Creating folders, and files for a project:
COPY *.py ./

CMD ["python", "main.py"]
