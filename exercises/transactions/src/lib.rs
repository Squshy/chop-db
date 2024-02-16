mod db;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    BeginTx,
    AbortTx,
    CommitTx,
    SetTx,
    GetTx,
    Get,
    Set,
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match value {
            "BEGIN_TX" => Self::BeginTx,
            "ROLLBACK_TX" => Self::AbortTx,
            "COMMIT_TX" => Self::CommitTx,
            "SET_TX" => Self::SetTx,
            "GET_TX" => Self::GetTx,
            "GET" => Self::Get,
            "SET" => Self::Set,
            _ => unreachable!("Do not goof up input pls: {}", value),
        }
    }
}

#[derive(Debug)]
struct Command {
    action: Action,
    tx_id: Option<usize>,
    key: Option<String>,
    value: Option<String>, // TODO: Refactor
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let splits = value.split(" ").collect::<Vec<_>>();

        if splits[0].ends_with("TX") {
            return Self {
                action: splits[0].into(),
                tx_id: splits.get(1).map(|s| s.parse::<_>().unwrap()),
                key: splits.get(2).map(|s| s.to_string()),
                value: splits.get(3).map(|s| s.to_string()),
            };
        }

        Self {
            action: splits[0].into(),
            key: splits.get(1).map(|s| s.to_string()),
            value: splits.get(2).map(|s| s.to_string()),
            tx_id: None,
        }
    }
}

#[cfg(test)]
mod test {
    use self::db::TransactionError;

    use super::*;

    fn parse_commands_from_str(str: &str) -> Vec<Command> {
        str.lines()
            .filter(|line| !line.trim().starts_with('#') && !line.trim().is_empty())
            .map(|line| line.into())
            .collect::<Vec<_>>()
    }

    fn assert_set_tx(
        db: &mut db::Database,
        command: &Command,
        res: Option<String>,
    ) -> Result<(), TransactionError> {
        assert_eq!(
            db.set_tx(
                command.key.clone().unwrap(),
                command.value.clone().unwrap(),
                command.tx_id.unwrap(),
            )?,
            res
        );

        Ok(())
    }

    fn assert_get_tx(
        db: &db::Database,
        command: &Command,
        res: Option<String>,
    ) -> Result<(), TransactionError> {
        assert_eq!(
            db.get_tx(command.key.as_ref().unwrap(), command.tx_id.unwrap())?,
            res
        );

        Ok(())
    }

    fn assert_get(db: &db::Database, command: &Command, res: Option<String>) {
        assert_eq!(db.get(&command.key.clone().unwrap()), res);
    }

    #[test]
    fn sample_input() {
        let mut tx_counter = 0;
        let commands = parse_commands_from_str(include_str!("../sample_input.txt"));
        let mut db = db::Database::new();
        assert_eq!(commands.len(), 23);

        let mut commands = commands.iter();
        // Begin tx
        let _ = commands.next().unwrap();
        tx_counter += 1;
        db.begin_tx_with_id(tx_counter).unwrap();

        // Set tx 1 x 10, expect None
        let command = commands.next().unwrap();
        assert_set_tx(&mut db, command, None).unwrap();

        // Get tx 1 x, Expect 10
        let command = commands.next().unwrap();
        assert_get_tx(&db, command, Some("10".into())).unwrap();

        // Set tx 1 x 11, expect 10
        let command = commands.next().unwrap();
        assert_set_tx(&mut db, command, Some("10".into())).unwrap();

        // Get tx 1 x, expect 11
        let command = commands.next().unwrap();
        assert_get_tx(&db, command, Some("11".into())).unwrap();

        // Get x, expect None
        let command = commands.next().unwrap();
        assert_get(&db, command, None);

        // Commit
        let command = commands.next().unwrap();
        db.commit_tx(command.tx_id.clone().unwrap()).unwrap();
        // Check it moved into DB
        assert_eq!(db.get(&"x".into()), Some("11".into()));

        // Begin 2
        let _ = commands.next().unwrap();
        tx_counter += 1;
        db.begin_tx_with_id(tx_counter).unwrap();

        // Begin 3
        let _ = commands.next().unwrap();
        tx_counter += 1;
        db.begin_tx_with_id(tx_counter).unwrap();

        // Get tx 2 x, expect 11
        let command = commands.next().unwrap();
        println!("{:?}", command);
        assert_get_tx(&db, command, Some("11".into())).unwrap();

        // Set tx 2 x 12, expect 11
        let command = commands.next().unwrap();
        println!("{:?}", command);
        assert_set_tx(&mut db, command, Some("11".into())).unwrap();

        // Get tx 3 x, expect 11
        let command = commands.next().unwrap();
        println!("{:?}", command);
        assert_get_tx(&db, command, Some("11".into())).unwrap();

        // Set tx 3 x 13, expect 11
        let command = commands.next().unwrap();
        println!("{:?}", command);
        assert_set_tx(&mut db, command, Some("11".into())).unwrap();

        // Abort tx 2
        let command = commands.next().unwrap();
        println!("{:?}", command);
        assert_eq!(db.abort_tx(command.tx_id.unwrap()).unwrap(), true);

        // Commit tx 3
        let command = commands.next().unwrap();
        println!("{:?}", command);
        db.commit_tx(command.tx_id.clone().unwrap()).unwrap();

        // Get x, expect 13
        let command = commands.next().unwrap();
        println!("{:?}", command);
        assert_get(&db, command, Some("13".into()));
    }
}
