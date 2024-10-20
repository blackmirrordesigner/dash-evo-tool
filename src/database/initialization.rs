use crate::database::Database;

impl Database {
    pub fn initialize(&self) -> rusqlite::Result<()> {
        // Create the settings table
        self.execute(
            "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            network TEXT NOT NULL,
            start_root_screen INTEGER NOT NULL
        )",
            [],
        )?;

        // Create the wallet table
        self.execute(
            "CREATE TABLE IF NOT EXISTS wallet (
        seed BLOB NOT NULL PRIMARY KEY,
        alias TEXT,
        is_main INTEGER,
        password_hint TEXT,
        network TEXT NOT NULL
    )",
            [],
        )?;

        // Create the identities table
        self.execute(
            "CREATE TABLE IF NOT EXISTS identity (
                id BLOB PRIMARY KEY,
                data BLOB,
                is_local INTEGER NOT NULL,
                alias TEXT,
                info TEXT,
                identity_type TEXT,
                network TEXT NOT NULL
            )",
            [],
        )?;

        // Create the composite index for faster querying
        self.execute(
            "CREATE INDEX IF NOT EXISTS idx_identity_local_network_type
     ON identity (is_local, network, identity_type)",
            [],
        )?;

        // Create the contested names table
        self.execute(
            "CREATE TABLE IF NOT EXISTS contested_name (
                normalized_contested_name TEXT NOT NULL,
                locked_votes INTEGER,
                abstain_votes INTEGER,
                awarded_to BLOB,
                end_time INTEGER,
                locked INTEGER NOT NULL DEFAULT 0,
                last_updated INTEGER,
                network TEXT NOT NULL,
                PRIMARY KEY (normalized_contested_name, network)
            )",
            [],
        )?;

        // Create the contestants table
        self.execute(
            "CREATE TABLE IF NOT EXISTS contestant (
                normalized_contested_name TEXT NOT NULL,
                identity_id BLOB NOT NULL,
                name TEXT,
                votes INTEGER,
                created_at INTEGER,
                created_at_block_height INTEGER,
                created_at_core_block_height INTEGER,
                document_id BLOB,
                network TEXT NOT NULL,
                PRIMARY KEY (normalized_contested_name, identity_id, network),
                FOREIGN KEY (normalized_contested_name, network) REFERENCES contested_name(normalized_contested_name, network) ON DELETE CASCADE
            )",
            [],
        )?;

        // Create the contracts table
        self.execute(
            "CREATE TABLE IF NOT EXISTS contract (
                contract_id BLOB,
                contract BLOB,
                name TEXT,
                network TEXT NOT NULL,
                PRIMARY KEY (contract_id, network)
            )",
            [],
        )?;

        self.execute(
            "CREATE INDEX IF NOT EXISTS idx_name_network ON contract (name, network)",
            [],
        )?;

        Ok(())
    }
}
