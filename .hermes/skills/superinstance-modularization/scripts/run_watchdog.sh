#!/usr/bin/env bash
# Helper to start the watchdog vectorization script with proper Hermes flags
# Usage: run_watchdog.sh

hermes terminal "python watchdog_vectorization.py" background=true notify_on_complete=true
