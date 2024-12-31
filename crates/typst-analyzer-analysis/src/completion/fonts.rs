use fontdb::FaceInfo;

use crate::{typ_logger, OkSome};

pub fn get_fonts() -> OkSome<Vec<FaceInfo>> {
    let mut db = fontdb::Database::new();
    let mut fonts = Vec::new();
    db.load_system_fonts();
    for font in db.faces() {
        fonts.push(font.to_owned());
    }
    typ_logger!("fonts: {:#?}", &fonts);
    Ok(Some(fonts))
}
