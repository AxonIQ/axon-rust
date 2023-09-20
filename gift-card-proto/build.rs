use std::io::Result;
fn main() -> Result<()> {
    let mut config = prost_build::Config::new();
    config.type_attribute(
        "gift_card.commands.IssueGiftCard",
        "#[derive(serde::Serialize)]",
    );
    config.type_attribute(
        "gift_card.commands.RedeemGiftCard",
        "#[derive(serde::Serialize)]",
    );
    config.type_attribute(
        "gift_card.commands.CancelGiftCard",
        "#[derive(serde::Serialize)]",
    );
    config.type_attribute(
        "gift_card.queries.GiftCardSummary",
        "#[derive(serde::Serialize)]",
    );
    config.type_attribute(
        "gift_card.queries.OneGiftCard",
        "#[derive(serde::Serialize)]",
    );
    config.type_attribute(
        "gift_card.queries.MultipleGiftCards",
        "#[derive(serde::Serialize)]",
    );
    config.compile_protos(
        &[
            "protos/commands.proto",
            "protos/events.proto",
            "protos/queries.proto",
        ],
        &["protos/"],
    )?;
    Ok(())
}
