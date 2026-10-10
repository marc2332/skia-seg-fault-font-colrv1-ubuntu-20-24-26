use skia_safe::{
    shaper::icu,
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle, TypefaceFontProvider},
    surfaces, Data, Font, FontMgr, TextBlob,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    icu::init();

    let font_data = Data::new_copy(include_bytes!("../assets/NotoColorEmoji-Regular.ttf"));
    let typeface = FontMgr::new()
        .new_from_data(font_data, None)
        .ok_or("could not load emoji typeface")?;
    let expected_typeface_id = typeface.unique_id();
    if typeface.unichar_to_glyph('🔍' as i32) == 0 {
        return Err("bundled font does not contain the emoji glyph".into());
    }
    let font = Font::new(typeface.clone(), 36.0);
    let mut text_blob = TextBlob::from_str("🔍", &font).ok_or("could not create emoji text blob")?;

    let mut provider = TypefaceFontProvider::new();
    provider.register_typeface(typeface, "Noto Color Emoji");
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(FontMgr::from(provider));
    font_collection.set_default_font_manager(FontMgr::new(), None);

    let mut text_style = TextStyle::new();
    text_style.set_font_families(&["Noto Color Emoji"]);
    text_style.set_font_size(36.0);
    let mut paragraph_style = ParagraphStyle::new();
    paragraph_style.set_text_style(&text_style);
    let mut builder = ParagraphBuilder::new(&paragraph_style, font_collection);
    let text = "Emoji paragraph: 🔍";
    builder.add_text(text);
    let mut paragraph = builder.build();
    paragraph.layout(600.0);
    let emoji_offset = text.split_once('🔍').ok_or("emoji missing from paragraph")?.0.encode_utf16().count();
    let emoji_typeface = paragraph.get_font_at_utf16_offset(emoji_offset).typeface();
    if emoji_typeface.unique_id() != expected_typeface_id {
        return Err(format!("paragraph used {} instead of the bundled emoji font", emoji_typeface.family_name()).into());
    }
    println!("paragraph emoji typeface: {} (matches bundled TTF)", emoji_typeface.family_name());
    let mut surface = surfaces::raster_n32_premul((640, 120)).ok_or("could not create surface")?;
    println!("painting paragraph");
    paragraph.paint(surface.canvas(), (10.0, 10.0));
    println!("finished painting paragraph");

    println!("paragraph height: {}", paragraph.height());
    println!("emoji blob is color font or bitmap: {}", paragraph.contains_color_font_or_bitmap(&mut text_blob));
    println!("painted paragraph with emoji without crashing");
    Ok(())
}
