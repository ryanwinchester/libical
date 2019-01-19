use calendars;
use KhResult;

pub fn action_get(args: &[String]) -> KhResult<()> {
  if args.is_empty() {
    Err("get calendars")?;
  }
  match args[0].as_str() {
    "calendars" => action_get_calendars(),
    _ => Err("Unknown get parameter!")?
  }
}

pub fn action_get_calendars() -> KhResult<()> {
  for calendar in calendars::calendar_list() {
    khprintln!("{}", calendar);
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  use testutils;

  #[test]
  fn test_get_calendars() {
    let _testdir = testutils::prepare_testdir("testdir_two_cals");

    action_get(&["calendars".to_string()]).unwrap();

    assert_eq!("first\nsecond\nsecond/second_sub\n", testutils::test_stdout_clear());
  }
}