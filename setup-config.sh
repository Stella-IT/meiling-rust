#!/bin/bash
echo "Welcome to Codename Meiling Configuration Utility"
echo
echo "Copyright (c) Stella IT Co, Ltd. and Meiling Project Contributors"
echo

input=""

get_input() {
  echo -n "$1";
  read -r input;
}

get_password_input() {
  echo -n "$1";
  read -rs input;
  echo
}

if_no_exit() {
  tmp_choice="$1"
  test -n "$tmp_choice" || tmp_choice='Y'

  tmp_choice=$(echo "$1" | awk '{print tolower($0)}')

  test "$tmp_choice" == "n" && echo "Ending session..."
  test "$tmp_choice" == "n" && exit 1
}

if [ -f ".env" ]; then
  echo "Error: dotenv file already exists."
  echo "If you want to reconfigure, please delete the dotenv file"
  exit 1
fi

# === Database part ===

echo "Database Configuration:"
get_input "  Database Host: "
db_host="$input"
get_input "  Database User: "
db_user="$input"
get_password_input "  Database Password: "
db_pass="$input"
get_input "  Database name: "
db_name="$input"
echo

echo "Database Configuration:"
echo "  Database Host: $db_host"
echo "  Database User: $db_user"
echo "  Database Password: [REDACTED]"
echo "  Database name: $db_name"
echo

get_input "Are these correct? (Y/n): "
if_no_exit "$input"

echo

# === Webserver part ===

echo "WebServer Configuration:"
get_input "  Bind Host: "
bind_host="$input"
get_input "  Bind Port: "
bind_port="$input"
echo

echo "WebServer Configuration:"
echo "  Bind Host: $bind_host"
echo "  Bind Port: $bind_port"
echo

get_input "Are these correct? (Y/n): "
if_no_exit "$input"
echo

# === Writing Config ===

echo "Writing Config..."
touch .env
echo "DATABASE_URL=\"mysql://$db_user:$db_pass@$db_host/$db_name\"" >> .env
echo "BIND_ADDRESS=\"$bind_host:$bind_port\"" >> .env

echo "Configuration Complete!"
