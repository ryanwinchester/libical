use chrono::*;
use std::path::PathBuf;
use utils;
use defaults;

struct SelectFilters {
  from: Option<Date<Local>>,
  to: Option<Date<Local>>,
}

impl SelectFilters {
  pub fn parse_from_args(args: &[String]) -> Result<Self, String> {
    let mut fromarg: Option<Date<Local>> = None;
    let mut toarg: Option<Date<Local>> = None;

    for chunk in args.chunks(2) {
      if chunk.len() == 2 {
        let mut datearg = match utils::date_from_str(&chunk[1]) {
          Ok(datearg) => datearg,
            Err(error) => {
              return Err(format!("{}", error))
            }
        };

        match chunk[0].as_str() {
          "from" => fromarg = Some(datearg),
          "to"   => toarg = Some(datearg),
          _      => return Err("select [from|to parameter]+".to_string())
        }
      } else {
        return Err("select [from|to parameter]+".to_string());
      }
    }
    Ok(SelectFilters {from: fromarg, to: toarg})
  }

  pub fn predicate_path_is_not_from(&self) -> impl Fn(&PathBuf) -> bool + '_ {
    move |path| {
      let filename = path.file_name().expect(&format!("{:?} not a file", path));
      match &self.from {
        Some(from) => filename < utils::get_bucket_for_date(from).as_str(),
        None => false
      }
    }
  }

  pub fn predicate_path_is_to<'a>(&'a self) -> impl Fn(&PathBuf) -> bool + 'a {
    move |path| {
      let filename = path.file_name().expect(&format!("{:?} not a file", path));
      match &self.to {
        Some(to) => filename <= utils::get_bucket_for_date(to).as_str(),
        None => true
      }
    }
  }

  pub fn predicate_line_is_from(&self) -> impl Fn(&String) -> bool + '_ {
    move |kline| {
      let cal = utils::read_khaleesi_line(kline).unwrap();  //expect sth...
      match &self.from {
        Some(from) => {
          let event = cal.get_principal_event();
          let pred_dtstart = event.get_dtstart().map_or(true, |dtstart| from <= &dtstart.date() );
          let pred_dtend = event.get_dtend().map_or(true, |dtend| from <= &dtend.date());
          pred_dtstart || pred_dtend
        }
        None => true
      }
    }
  }

  pub fn predicate_line_is_to(&self) -> impl Fn(&String) -> bool + '_ {
    move |kline| {
      let cal = utils::read_khaleesi_line(kline).unwrap();  //expect sth...
      match &self.to {
        Some(to) => {
          let event = cal.get_principal_event();
          let pred_dtstart = event.get_dtstart().map_or(true, |dtstart| &dtstart.date() <= to);
          let pred_dtend = event.get_dtend().map_or(true, |dtend| &dtend.date() <= to);
          pred_dtstart || pred_dtend
        }
        None => true
      }
    }
  }
}

pub fn select_by_args(args: &[String]) {
  let filters: SelectFilters;

  match SelectFilters::parse_from_args(args) {
    Err(error) => {
      println!("{}", error);
      return
    },
    Ok(parsed_filters) => filters = parsed_filters,
  }

  let indexdir = defaults::get_indexdir();

  let mut buckets: Vec<PathBuf> = utils::file_iter(&indexdir)
    .collect();
  buckets.sort_unstable();
  let buckets = buckets.into_iter().skip_while( filters.predicate_path_is_not_from() )
    .take_while( filters.predicate_path_is_to() );

  let mut lines: Vec<String> = buckets.map(|bucket| utils::read_lines_from_file(&bucket))
    .filter_map(|lines| lines.ok())
    .flatten()
    .filter( filters.predicate_line_is_from() )
    .filter( filters.predicate_line_is_to() )
    .collect();

  lines.sort_unstable();
  lines.dedup();

  for line in lines {
    println!("{}", line);
  }
}
