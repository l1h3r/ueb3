/// Add `#[cfg(feature = ...)]` individually to a sequence of items.
#[macro_export]
macro_rules! feature {
  (#[feature = $feature:literal] $($item:item)*) => {
    $(
      #[cfg(feature = $feature)]
      $item
    )*
  };
}
