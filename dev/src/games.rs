mod sql;
mod theft;
mod vars;
mod void;

use crate::sql::{connect, query_ledger_id};
use crate::theft::{filename, get_to_file};
use crate::vars::gather;
use std::path::Path;

const QUERY_GAME_IDS: &str = {
    "SELECT id \
     FROM schedules \
     WHERE ledger_id = ?1;"
};

fn scrape(
    start: &str,
    end: &str,
    wd: &str,
    id: &str,
    directory: &str,
    url: &str,
) -> String {
    let x: String = filename(wd, directory, id, start, end);
    println!("{}", &x);
    get_to_file(url, Path::new(&x), 1500);
    x
}

fn scrape_both(
    start: &str,
    end: &str,
    wd: &str,
    id: &Option<String>,
) -> Option<(String, String)> {
    let f = |id: &str, directory: &str, url: &str| -> String {
        scrape(start, end, wd, id, directory, url)
    };
    id.as_ref().map(|id| {
        (
            f(
                id,
                "games",
                &format!(
                    "https://statsapi.web.nhl.com/\
                     api/v1/game/{}/feed/live?site=en_nhl",
                    id,
                ),
            ),
            f(
                id,
                "shifts",
                &format!(
                    "http://www.nhl.com/\
                     stats/rest/shiftcharts?cayenneExp=gameId={}",
                    id,
                ),
            ),
        )
    })
}

fn main() {
    if let Some((start, end, wd)) = gather() {
        if let Ok(c) = connect(&wd) {
            if let Some(ledger_id) = query_ledger_id(&start, &end, &c) {
                if let Ok(ids) = c.prepare(QUERY_GAME_IDS).and_then(|mut s| {
                    s.query_map(&[&ledger_id], |r| {
                        let id: String = r.get("id");
                        id
                    })
                    .map(|ids| {
                        let filenames: Vec<Option<(String, String)>> = ids
                            .map(|id| {
                                (scrape_both(&start, &end, &wd, &id.ok()))
                            })
                            .collect();
                        filenames
                    })
                }) {
                    ids.into_iter()
                        .map(|id| {
                            if let Some((_, _)) = id {
                                //
                            }
                        })
                        .collect()
                }
            };
        };
    };
}
