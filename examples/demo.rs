use std::time::Duration;

use rust3270::server::{color::Color, extended_field_attributes::ExtendedFieldAttribute, highlighting::Highlighting, transparency::Transparency, wcc::{FieldAttribute, WCC}};
use structopt::StructOpt;

use rust3270::server::screen::{Screen, Field};

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(short="h", long = "host", default_value="::1")]
    host: String,
    #[structopt(short="p", long = "port", default_value="3270")]
    port: u16,
}

static RUSTLOGO: [&'static str; 4] = [
  r#"     _~^~^~_     "#,
  r#" \) /  o o  \ (/ "#,
  r#"   '_   ¬   _'   "#,
  r#"   / '-----' \   "#,
];

static INTROLOGORUST: [&'static str; 6] = [
    r#" _______    _____  _____  _______ _________ "#,
    r#"|_   __ \  |_   _||_   _|/  ___  |  _   _  |"#,
    r#"  | |__) |   | |    | | |  (__ \_|_/ | | \_|"#,
    r#"  |  __ /    | '    ' |  '.___\-.    | |    "#,
    r#" _| |  \ \_   \ \--' /  |\\____) |  _| |_   "#,
    r#"|____| |___|   \.__.'   |_______.' |_____|  "#,
];

static INTROLOGO3270: [&'static str; 6] = [
    r#"  ______     _____    _______     ____   "#,
    r#" / ____ \.  / ___ \. |  ___  |  .'    '. "#,
    r#" \'  __) | |_/___) | |_/  / /  |  .--.  |"#,
    r#" _  |__ '.  .'____.'     / /   | |    | |"#,
    r#"| \____) | / /____      / /    |  \--'  |"#,
    r#" \______.' |_______|   /_/      '.____.' "#,
];

fn intro_screen(session: &mut rust3270::server::Session) -> anyhow::Result<()> {
    use rust3270::server::stream::*;
    let bufsz = BufferAddressCalculator { width: 80, height: 24 };
    let mut record = WriteCommand {
        command: WriteCommandCode::Write,
        wcc: WCC::RESET | WCC::KBD_RESTORE | WCC::RESET_MDT,
        orders: vec![
            WriteOrder::SetBufferAddress(0),
            WriteOrder::EraseUnprotectedToAddress(bufsz.last_address()),
            WriteOrder::SetBufferAddress(bufsz.encode_address(1, 31)),
            WriteOrder::StartFieldExtended(vec![
                ExtendedFieldAttribute::FieldAttribute(FieldAttribute::PROTECTED),
                ExtendedFieldAttribute::ForegroundColor(Color::Red),
            ]),
            WriteOrder::SendText("Hello from Rust!".into()),
            WriteOrder::SetBufferAddress(bufsz.encode_address(20, 10)),
            WriteOrder::StartFieldExtended(vec![
                ExtendedFieldAttribute::FieldAttribute(FieldAttribute::PROTECTED),
                ExtendedFieldAttribute::ForegroundColor(Color::Green),
                ExtendedFieldAttribute::Transparency(Transparency::Xor),
                ExtendedFieldAttribute::ExtendedHighlighting(Highlighting::Reverse)
            ]),
            WriteOrder::SendText("Jumping to next screen in a few seconds!".into()),
            WriteOrder::SetBufferAddress(bufsz.encode_address(20, 50)),
            WriteOrder::StartField(FieldAttribute::PROTECTED),

            WriteOrder::SetBufferAddress(bufsz.encode_address(24, 10)),
            WriteOrder::StartFieldExtended(vec![
                ExtendedFieldAttribute::ForegroundColor(Color::Green),
                ExtendedFieldAttribute::ExtendedHighlighting(Highlighting::Blink)
            ]),
            WriteOrder::SendText("Please wait...".into()),
            WriteOrder::SetBufferAddress(bufsz.encode_address(24, 25)),
            WriteOrder::StartField(FieldAttribute::PROTECTED),
        ],
    };

    for (i, line) in RUSTLOGO.iter().enumerate() {
        record.orders.push(WriteOrder::SetBufferAddress(bufsz.encode_address(4
            +i as u16, 50)));
        record.orders.push(WriteOrder::StartFieldExtended(vec![
            ExtendedFieldAttribute::FieldAttribute(FieldAttribute::PROTECTED),
            ExtendedFieldAttribute::ForegroundColor(Color::Red),
        ]));
        record.orders.push(WriteOrder::SendText((*line).into()));
    }

    for (i, line) in INTROLOGORUST.iter().enumerate() {
        record.orders.push(WriteOrder::SetBufferAddress(bufsz.encode_address(3+i as u16, 1)));
        record.orders.push(WriteOrder::StartFieldExtended(vec![
            ExtendedFieldAttribute::FieldAttribute(FieldAttribute::PROTECTED),
            ExtendedFieldAttribute::ForegroundColor(Color::Red),
        ]));
        record.orders.push(WriteOrder::SendText((*line).into()));
    }

    for (i, line) in INTROLOGO3270.iter().enumerate() {
    // Cycle through a set of colors for a rainbow effect
    let colors = [
        Color::Red,
        Color::Yellow,
        Color::Green,
        Color::Blue,
        Color::Pink,
        Color::Purple,
    ];
    let color = colors[i % colors.len()];
    record.orders.push(WriteOrder::SetBufferAddress(bufsz.encode_address(10 + i as u16, 25)));
    record.orders.push(WriteOrder::StartFieldExtended(vec![
        ExtendedFieldAttribute::FieldAttribute(FieldAttribute::PROTECTED),
        ExtendedFieldAttribute::ForegroundColor(color),
    ]));
    record.orders.push(WriteOrder::SendText((*line).into()));
}

    session.send_record(&record)?;
    session.send_record(&WriteCommand{
        command: WriteCommandCode::Write,
        wcc: WCC::RESET_MDT | WCC::KBD_RESTORE,
        orders: vec![],
    })?;

    let record = session.receive_record(None)?;

    if let Some(record) = record {
        eprintln!("Incoming record: {:?}", hex::encode(&record));
        eprintln!("Decoded: {:#?}", IncomingRecord::parse_record(record.as_slice()))
    } else {
        eprintln!("No record");
    }

    Ok(())
}


fn hlapi_demo(session: &mut rust3270::server::Session) -> anyhow::Result<()> {
    let mut name = "        ".to_string();
    let mut passwd = "        ".to_string();

    let result = Screen {
        fields: vec![
            Field::at(1, 32).ro_text("Please enter your data"),
            Field::at(3, 10).ro_text("Name: "),
            Field::at(3, 20).rw_text(&mut name),
            Field::at(4, 10).ro_text("Password: "),
            Field::at(4, 20).rw_text(&mut passwd)
                .with_attr(ExtendedFieldAttribute::FieldAttribute(FieldAttribute::NON_DISPLAY)),
        ],
    }.present(&mut *session)?;

    let aid = format!("{:?}", result.aid);
    Screen {
        fields: vec![
          Field::at(1, 32).ro_text("Your data"),
          Field::at(3, 10).ro_text("Name: "),
          Field::at(3, 20).ro_text(name.as_str()),
          Field::at(4, 10).ro_text("Password: "),
          Field::at(4, 20).ro_text(passwd.as_str()),
            Field::at(5, 10).ro_text("You pressed: "),
            Field::at(5, 25).ro_text(aid.as_str()),
            Field::at(23, 32).ro_text("Press ENTER to exit"),
        ],
    }.present(&mut *session)?;

    Ok(())
}

fn run(mut session: rust3270::server::Session) -> anyhow::Result<()> {
    let _ = intro_screen(&mut session);
    std::thread::sleep(Duration::from_secs(5));
    let _ = hlapi_demo(&mut session);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let options: Cli = Cli::from_args();
    let server = std::net::TcpListener::bind((options.host.as_str(), options.port))?;

    println!("Listening on {}:{}", options.host, options.port);

    for client in server.incoming() {
        let client = client?;
        std::thread::spawn(move || {
            let session = match rust3270::server::Session::new(client) {
                Ok(session) => {
                    eprintln!("New session established.");
                    session
                }
                Err(err) => {
                    eprintln!("Error accepting session: {}", err);
                    return;
                }
            };

            if let Err(err) = run(session) {
                eprintln!("Error in session: {}", err);
            }

        });
    }
    Ok(())
}