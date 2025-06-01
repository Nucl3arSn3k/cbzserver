@echo off
start cmd /k "cd frontend && npm run dev"
start cmd /k "cargo run 2^> errors.txt"