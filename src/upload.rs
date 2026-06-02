/*!
spool, a Rust written CDN uploader
Copyright (C) 2026, Tuxzilla T. Penguin

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/#AGPL>.
 */

use axum::{Router, extract::Multipart, http::StatusCode, routing::post};
//use futures_util::stream::{StreamExt, TryStreamExt};
//use std::path::Path;
//use tokio::fs::File;
//use tokio::io::AsyncWriteExt;

pub async fn upload(mut multipart: Multipart) -> Result<StatusCode, StatusCode> {
    // irony is this downloads files yet is named upload
    // check api key here

    // uhh wait what else do we do
    // oh yeah after auth accept n download file

    // add to database here
    // ogfilename
    // filename
    // size (in bytes)
    // timestamp
    // uploaded_by

    // send the thing below!
    Ok(StatusCode::OK)
}
