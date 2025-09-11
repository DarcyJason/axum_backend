#!/bin/bash

echo "Checking if Postgres is running..."

if pg_isready -h localhost -p 5432 -U postgres >/dev/null 2>&1; then
  echo "✅ Postgres is running!"
else
  echo "❌ Postgres is not ready!"
  echo "⚠️ Please google it to start postgres on your system!"
  exit 1
fi
