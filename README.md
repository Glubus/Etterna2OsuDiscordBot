# Discord Bot in Rust

A modern Discord bot built with Serenity in Rust, featuring **slash commands** and advanced osu! integration.

## 🏗️ Modern Architecture

The project uses Discord slash commands with a modular architecture and service-oriented design:

```
src/
├── main.rs              # Main entry point
├── bot.rs               # Main bot logic with event handlers
├── config.rs            # Configuration management
├── commands/            # Slash commands module
│   ├── mod.rs          # Command modules definition
│   ├── ping.rs         # /ping command
│   ├── calc.rs         # /calc command
│   ├── help.rs         # /help command
│   └── rating.rs       # /rating command for osu! beatmaps
├── handlers/            # Business logic handlers
│   ├── mod.rs          # Handler modules
│   ├── osu_handler.rs  # osu! beatmap processing
│   └── etterna_rating.rs # Etterna rating calculations
├── services/            # Service layer
│   ├── mod.rs          # Service modules
│   ├── osu_api.rs      # osu! API integration with rosu-v2
│   └── etterna_rating.rs # MSD calculation service
├── helpers/             # Utility modules
│   ├── mod.rs          # Helper modules
│   ├── link.rs         # Link transformation utilities
│   └── osu_api.rs      # Legacy osu! API helpers
└── embed/               # Discord embed builders
    └── rating.rs        # Rating embed creation
```

## 🎯 Features

- ✅ **Modern slash commands** - Uses Discord's modern API
- ✅ **osu! Integration** - Beatmap analysis and SSR ratings
- ✅ **Service-oriented architecture** - Clean separation of concerns
- ✅ **Parallel processing** - Efficient async operations
- ✅ **Chart generation** - Radar charts for skill visualization
- ✅ **Robust error handling** - Comprehensive error management
- ✅ **Configuration management** - Environment-based configuration
- ✅ **Modular design** - Easy to extend and maintain

## 🚀 Quick Start

### Prerequisites

- Rust (latest stable version)
- Discord Bot Token
- osu! API credentials

### Configuration

1. **Create a Discord Bot**:
   - Go to [Discord Developer Portal](https://discord.com/developers/applications)
   - Create a new application
   - In the "Bot" tab, create a bot
   - Copy the bot token

2. **Get osu! API Credentials**:
   - Go to [osu! API v2](https://osu.ppy.sh/docs/index.html)
   - Create a new OAuth application
   - Get your Client ID and Client Secret

3. **Configure Environment**:
   Create a `.env` file in the project root:
   ```env
   DISCORD_TOKEN=your_discord_bot_token_here
   OSU_CLIENT_ID=your_osu_client_id_here
   OSU_CLIENT_SECRET=your_osu_client_secret_here
   COMMAND_PREFIX=.
   ```

4. **Invite the Bot**:
   - In the "OAuth2" > "URL Generator" tab
   - Select "bot" in scopes
   - Select required permissions (Send Messages, Use Slash Commands, Attach Files)
   - Use the generated URL to invite the bot

### Installation and Running

```bash
# Install dependencies
cargo build

# Run the bot
cargo run
```

## 📋 Available Commands

### `/calc <expression>`
Calculates mathematical expressions
- Examples: `/calc 2 + 2`, `/calc 10 * 5`, `/calc (3 + 4) * 2`
- Supported operators: `+`, `-`, `*`, `/`, `(`, `)`

### `/ping`
Tests bot latency

### `/help`
Displays help with all available commands

### `/rating <beatmap_link>`
Analyzes an osu! beatmap and provides SSR ratings
- Supports beatmapset links from osu.ppy.sh
- Generates radar charts for skill visualization
- Shows ratings for different rates (0.9 to 1.5)
- Displays beatmap metadata (artist, title, mapper, etc.)

## 🏛️ Architecture Overview

### Service Layer (`services/`)
- **`OsuApiService`**: Handles all osu! API interactions using `rosu-v2`
- **`etterna_rating`**: MSD (Mania Skill Difficulty) calculations

### Handler Layer (`handlers/`)
- **`OsuHandler`**: Main business logic for osu! beatmap processing
- Parallel execution of API calls and beatmap loading
- Chart generation and embed creation

### Helper Layer (`helpers/`)
- **`link.rs`**: Link transformation utilities
- **`osu_api.rs`**: Legacy API helpers (being phased out)

### Command Layer (`commands/`)
- Slash command definitions and interaction handling
- Parameter validation and user feedback

## 🔧 Technical Stack

- **Rust** - Core language
- **Serenity** - Discord API wrapper
- **rosu-v2** - Modern osu! API client
- **minacalc-rs** - SSR calculation engine
- **plotters** - Chart generation library
- **tokio** - Async runtime
- **anyhow** - Error handling
- **serde** - Serialization/deserialization

## 🎨 Chart Generation

The bot generates radar charts for osu! beatmap skills:
- **Stream** - Stream difficulty
- **Jumpstream** - Jumpstream patterns
- **Handstream** - Handstream patterns
- **Jackspeed** - Jack speed
- **Stamina** - Stamina requirements
- **Chordjack** - Chord jack patterns

Charts are normalized to a maximum of 40.0 for visualization while displaying actual values in labels.

## 🔄 Development Workflow

1. **Service Development**: Add new services in `services/`
2. **Handler Logic**: Implement business logic in `handlers/`
3. **Command Integration**: Create slash commands in `commands/`
4. **Testing**: Use Discord's test environment for development

## 📝 Contributing

1. Fork the repository
2. Create a feature branch
3. Implement your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Acknowledgments

- [Serenity](https://github.com/serenity-rs/serenity) - Discord API wrapper
- [rosu-v2](https://github.com/rosu-rs/rosu-v2) - osu! API client
- [minacalc-rs](https://github.com/etternagame/minacalc-rs) - SSR calculation engine
