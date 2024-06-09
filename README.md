# iRacing Tracks Fetcher

This Rust script authenticates with the iRacing API and fetches track information. The track details are then written to a CSV file for easy access and analysis.

# Features

- Authenticates with the iRacing API.
- Fetches track details including track ID, name, configuration name, location, length, and category.
- Outputs the track details to a CSV file with a timestamped filename.

# Usage

### Step 1: Download the Executable

Download the appropriate executable for your operating system from the [releases page](https://github.com/brandonjjon/iracing-tracks-fetcher/releases).

### Step 2: Setup Environment Variables

Create a `.env` file in the same directory as the executable with the following content:

```env
IRACING_EMAIL=your_email@domain.com
IRACING_PASSWORD=your_password
```

Alternatively, you can set these environment variables directly in your system.

### Step 3: Run the Executable

Open a terminal and navigate to the directory containing the executable and the .env file. Run the executable:

```bash
./iracing-tracks-fetcher
```

On Windows, you can double-click the executable or run it via the Command Prompt:

```cmd
iracing-tracks-fetcher.exe
```

The script will authenticate with the iRacing API, fetch the track details, and write them to a CSV file in the output directory.

# Contributing

## Using DevContainer

This project includes a devcontainer configuration to facilitate development within a consistent environment.

### Step 1: Clone the Repository

```bash
git clone https://github.com/brandonjjon/iracing-tracks-fetcher.git
cd iracing-tracks-fetcher
```

### Step 2: Open in VS Code

Open the project in VS Code. If you have the Remote - Containers extension installed, you will be prompted to reopen the project in a container. Accept the prompt.

### Step 3: Setup Environment Variables

Inside the devcontainer, create a .env file in the root directory with the following content:

```env
IRACING_EMAIL=your_email@domain.com
IRACING_PASSWORD=your_password
```

### Step 4: Build and Run the Script

Inside the devcontainer terminal, build and run the script:

```bash
cargo build
cargo run
```

The script will authenticate with the iRacing API, fetch the track details, and write them to a CSV file in the output directory.

## Project Structure

```bash
src/
├── auth.rs         // Handles authentication with the iRacing API
├── config.rs       // Manages configuration settings
├── main.rs         // Main entry point for the script
├── tracks.rs       // Handles fetching track details from the API
└── utils.rs        // Contains utility functions for file operations
.env                // Environment variables file (not included in the repository)
Cargo.toml          // Rust project file
README.md           // Project README file
```

## Contributions

Contributions are welcome! Please fork the repository and submit a pull request.