# Rust Basic To-Do App

## Features

> **CLI is not available yet. SORRY!!**

* CLI access.
* API access.
* Authentication on CLI and API.
* Database integration for CLI and API.
* Docker build available.

## How to Run

Running this is very simple.

* Rename the `.env.SAMPLE` to `.env` and update accordingly if you need.
* Run `docker compose up --build -d`

## API data Sample

```json
{
    "code": "A2000",
    "data": [
        {
            "created_at": "2024-07-16T17:57:24.613793",
            "description": "This is something else.",
            "id": 2,
            "is_checked": false,
            "label_id": 1,
            "title": "Test Done",
            "updated_at": "2024-07-16T23:57:44.446268"
        }
    ],
    "msg": "Request process successfully."
}
```

## Conclusion

**Notice:** This is my first project in rust. Please share your thoughts(Good/Bad) by creating issues here. PR is most
welcome. But I really appreciate create an issue before you send me a PR.

Thank you
