# Habit Tracker API

An API that can be used with a frontend to create a habit tracker app.

## Tech Stack

**Server:** Axum Rust

**Database:** Sqlx, Sqlite

## Run Locally

Clone the project

```bash
  git clone https://github.com/that1Kid878/habit-tracker-api
```

Go to the project directory

```bash
  cd my-project
```

Compose docker

```bash
  docker-compose up --build
```

## API Reference

#### Create new habit

```http
  POST /habits/mnew
```

Body:

```
{
    "type": "object",
    "properties": {
        "username": {
            "type": "string"
        },
        "name": {
            "type": "string"
        },
        "description": {
            "type": "string"
        },
        "priority": {
            "type": "integer"
        },
        "days": {
            "type": "array",
            "items": {
                "type": "integer"
            }
        }
    }
}
```

#### Get habit

```http
  GET /habits
```

Param:

| Key      | Type   |
| -------- | ------ |
| username | string |
| limit    | int    |
| id       | int    |
| name     | string |
| day      | int    |
| priority | int    |

#### Edit existing habit

```http
  PUT /habits
```

Body:

```

    "type": "object",
    "properties": {
        "id": {
            "type": "integer"
        },
        "name": {
            "type": "string"
        },
        "description": {
            "type": "string"
        },
        "priority": {
            "type": "integer"
        },
        "days": {
            "type": "array",
            "items": {
                "type": "integer"
            }
        }
    }
}
```

#### Delete an existing habit

```http
  DELETE /habits/{id}
```

#### Create a new habit log

```http
  POST /log/new
```

Body:

```
{
    "type": "object",
    "properties": {
        "habit_id": {
            "type": "integer"
        },
        "completed": {
            "type": "boolean"
        }
    }
}
```

#### Get a habit log

```http
  GET /logs
```

Param:

| Key      | Type   |
| -------- | ------ |
| username | string |
| limit    | int    |
| id       | int    |
| habit_id | int    |
| from     | Date   |
| to       | Date   |

#### Delete a habit log

```http
  DELETE /log/{id}
```

## License

[MIT](https://choosealicense.com/licenses/mit/)
