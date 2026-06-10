/*!
spool, a Rust written CDN uploader
Copyright (C) 2026, Tuxzilla T. Penguin

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/#AGPL>.
 */

use axum::{
    extract::{Multipart, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use sea_orm::DatabaseConnection;

pub async fn upload(
    State(_db): State<DatabaseConnection>,
    headers: HeaderMap,
    _multipart: Multipart,
) -> impl IntoResponse {
    // irony is this downloads files yet is named upload.. maybe i should rename it to like download or something
    let _spool_key = match headers.get("X-spool-key") {
        Some(value) => match value.to_str() {
            Ok(str_val) => str_val,
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid spool key!").into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, "missing X-spool-key header!").into_response(),
    };

    // TODO: check spool key against database lmao

    // if spool key is valid, add og filename and scrambled filename to database
    // accept file n download (need to.. somehow have the client stream it. or something)
    // add more file details here to database

    // send the thing below!
    (
        StatusCode::OK,
        "upload successful! thank you for choosing spool",
    )
        .into_response()
}
