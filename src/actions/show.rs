use input;
use utils::fileutil;
use KhResult;

pub fn do_show(_args: &[&str]) -> KhResult<()> {
  info!("do_show");
  let lines = input::default_input_khlines()?;

  for line in lines {
    let output = fileutil::read_file_to_string(line.get_path()).unwrap();
    khprintln!("{}", output);
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  use testutils::*;
  use utils::stdioutils::*;

  #[test]
  fn test_() {
    let _testdir = prepare_testdir("testdir_with_seq");

    do_show(&[]).unwrap();

    let stdout = test_stdout_clear();
    assert_eq!(784, stdout.len());
    assert_eq!(32, stdout.lines().count());
  }
}
