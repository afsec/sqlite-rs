#[macro_export]
macro_rules! impl_name {
  ($struct_name:ty) => {
    impl $crate::traits::Name for $struct_name {
      const NAME: &'static str = stringify!($struct_name);
    }
  };
}

#[macro_export]
macro_rules! field_parsing_error {
  ($entity_name:expr) => {
    $crate::result::SqliteError::ParsingField(
      $crate::result::FieldParsingError {
        error: "Invalid payload".into(),
        ty: $entity_name,
      },
    )
  };
}

#[macro_export]
macro_rules! resolve_error {
  ($res:expr) => {
    match $res {
      Ok(data) => data,
      Err(err) => {
        error!("Error: {err}");
        return Err(err);
      }
    }
  };
}
