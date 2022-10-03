use semi_structured_logs::*;

#[test]
fn log_1() {
  let m = log(LogLevel::Info, "Run success");
  assert_eq!("[INFO]: Run success", m);
}

#[test]
fn info_1() {
  let m = info("Run success");
  assert_eq!("[INFO]: Run success", m);
}

#[test]
fn warn_1() {
  let m = warn("System Warning");
  assert_eq!("[WARNING]: System Warning", m);
}

#[test]
fn error_1() {
  let m = error("System Error");
  assert_eq!("[ERROR]: System Error", m);
}