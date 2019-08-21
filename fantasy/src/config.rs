use std::path::Path;

/// fantasy config
#[derive(Debug, Clone, Getters, Default)]
pub struct Config<P: AsRef<Path>> {
  /// rtdlib project root path
  path_rtd: P,
  /// telegram client project root path
  path_telegram_client: P,
  /// tl schema file path
  path_tl: P,
}




