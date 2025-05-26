use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct as _;

use crate::crypto::keccak256;
use crate::eth::Address;
use crate::eth::H256;
use crate::rpc::FilterBlock;
use crate::rpc::FilterSet;

// -----------------------------------------------------------------------------
// Ethereum Filter Object
// -----------------------------------------------------------------------------

/// A transaction log filter object.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Filter {
  /// The filter block identifier.
  pub block: FilterBlock,
  /// The contract address or list of addresses to match logs against.
  pub address: FilterSet<Address>,
  /// An array of order-dependent log topics.
  pub topics: [FilterSet<H256>; 4],
}

impl Filter {
  /// Creates a new, empty `Filter`.
  #[inline]
  pub const fn new() -> Self {
    Self {
      block: FilterBlock::Empty,
      address: FilterSet::new(),
      topics: [const { FilterSet::new() }; 4],
    }
  }

  /// Returns `true` if `self` has any associated topics.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::rpc::Filter;
  ///
  /// let filter: Filter = Filter::new().with_event("Foo");
  ///
  /// assert_eq!(filter.has_topics(), true);
  /// ```
  #[inline]
  pub fn has_topics(&self) -> bool {
    self.topics.iter().any(|topic| !topic.is_empty())
  }

  /// Sets the filter block identifier.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::Filter;
  /// use ueb3_types::rpc::FilterBlock;
  ///
  /// let mut filter: Filter = Filter::new();
  ///
  /// assert_eq!(filter.block, FilterBlock::Empty);
  ///
  /// filter.block(H256::NULL);
  ///
  /// assert_eq!(filter.block, FilterBlock::Exact(H256::NULL));
  /// ```
  #[inline]
  pub fn block<T>(&mut self, value: T)
  where
    T: Into<FilterBlock>,
  {
    self.block = value.into();
  }

  /// Sets the address value of the filter.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::Address;
  /// use ueb3_types::rpc::Filter;
  /// use ueb3_types::rpc::FilterSet;
  ///
  /// let mut filter: Filter = Filter::new();
  ///
  /// assert_eq!(filter.address.len(), 0);
  ///
  /// filter.address(Address::NULL);
  ///
  /// assert_eq!(filter.address.len(), 1);
  /// assert_eq!(filter.address, FilterSet::from(Address::NULL));
  /// ```
  #[inline]
  pub fn address<T>(&mut self, value: T)
  where
    T: Into<FilterSet<Address>>,
  {
    self.address = value.into();
  }

  /// Sets the event name to filter non-anonymous events.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::Filter;
  /// use ueb3_types::rpc::FilterSet;
  ///
  /// let string: &str = "0xb608c74283f334e1f047dbbf1daa2407d41d4689aca67c422796f936acce16b7";
  /// let hashed: H256 = string.parse().unwrap();
  /// let mut filter: Filter = Filter::new();
  ///
  /// assert_eq!(filter.topics[0].len(), 0);
  ///
  /// filter.event("Foo");
  ///
  /// assert_eq!(filter.topics[0].len(), 1);
  /// assert_eq!(filter.topics[0], FilterSet::from(hashed));
  /// ```
  #[inline]
  pub fn event(&mut self, value: &str) {
    self.topic0(keccak256(value));
  }

  /// Sets the value of the topic at index `0`.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::Filter;
  /// use ueb3_types::rpc::FilterSet;
  ///
  /// let string: &str = "0xb608c74283f334e1f047dbbf1daa2407d41d4689aca67c422796f936acce16b7";
  /// let hashed: H256 = string.parse().unwrap();
  /// let mut filter: Filter = Filter::new();
  ///
  /// assert_eq!(filter.topics[0].len(), 0);
  ///
  /// filter.topic0(hashed);
  ///
  /// assert_eq!(filter.topics[0].len(), 1);
  /// assert_eq!(filter.topics[0], FilterSet::from(hashed));
  #[inline]
  pub fn topic0<T>(&mut self, value: T)
  where
    T: Into<FilterSet<H256>>,
  {
    self.topics[0] = value.into();
  }

  /// Sets the value of the topic at index `1`.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::Filter;
  /// use ueb3_types::rpc::FilterSet;
  ///
  /// let string: &str = "0xb608c74283f334e1f047dbbf1daa2407d41d4689aca67c422796f936acce16b7";
  /// let hashed: H256 = string.parse().unwrap();
  /// let mut filter: Filter = Filter::new();
  ///
  /// assert_eq!(filter.topics[1].len(), 0);
  ///
  /// filter.topic1(hashed);
  ///
  /// assert_eq!(filter.topics[1].len(), 1);
  /// assert_eq!(filter.topics[1], FilterSet::from(hashed));
  #[inline]
  pub fn topic1<T>(&mut self, value: T)
  where
    T: Into<FilterSet<H256>>,
  {
    self.topics[1] = value.into();
  }

  /// Sets the value of the topic at index `2`.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::Filter;
  /// use ueb3_types::rpc::FilterSet;
  ///
  /// let string: &str = "0xb608c74283f334e1f047dbbf1daa2407d41d4689aca67c422796f936acce16b7";
  /// let hashed: H256 = string.parse().unwrap();
  /// let mut filter: Filter = Filter::new();
  ///
  /// assert_eq!(filter.topics[2].len(), 0);
  ///
  /// filter.topic2(hashed);
  ///
  /// assert_eq!(filter.topics[2].len(), 1);
  /// assert_eq!(filter.topics[2], FilterSet::from(hashed));
  #[inline]
  pub fn topic2<T>(&mut self, value: T)
  where
    T: Into<FilterSet<H256>>,
  {
    self.topics[2] = value.into();
  }

  /// Sets the value of the topic at index `3`.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::Filter;
  /// use ueb3_types::rpc::FilterSet;
  ///
  /// let string: &str = "0xb608c74283f334e1f047dbbf1daa2407d41d4689aca67c422796f936acce16b7";
  /// let hashed: H256 = string.parse().unwrap();
  /// let mut filter: Filter = Filter::new();
  ///
  /// assert_eq!(filter.topics[3].len(), 0);
  ///
  /// filter.topic3(hashed);
  ///
  /// assert_eq!(filter.topics[3].len(), 1);
  /// assert_eq!(filter.topics[3], FilterSet::from(hashed));
  #[inline]
  pub fn topic3<T>(&mut self, value: T)
  where
    T: Into<FilterSet<H256>>,
  {
    self.topics[3] = value.into();
  }

  // ---------------------------------------------------------------------------
  // Builder-Style API
  // ---------------------------------------------------------------------------

  /// Sets the filter block identifier.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::Filter;
  /// use ueb3_types::rpc::FilterBlock;
  ///
  /// let filter: Filter = Filter::new().with_block(H256::NULL);
  ///
  /// assert_eq!(filter.block, FilterBlock::Exact(H256::NULL));
  /// ```
  #[inline]
  #[must_use]
  pub fn with_block<T>(mut self, value: T) -> Self
  where
    T: Into<FilterBlock>,
  {
    self.block(value);
    self
  }

  /// Sets the address value of the filter.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::Address;
  /// use ueb3_types::rpc::Filter;
  /// use ueb3_types::rpc::FilterSet;
  ///
  /// let filter: Filter = Filter::new().with_address(Address::NULL);
  ///
  /// assert_eq!(filter.address, FilterSet::from(Address::NULL));
  /// ```
  #[inline]
  #[must_use]
  pub fn with_address<T>(mut self, value: T) -> Self
  where
    T: Into<FilterSet<Address>>,
  {
    self.address(value);
    self
  }

  /// Sets the event name to filter non-anonymous events.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::Filter;
  /// use ueb3_types::rpc::FilterSet;
  ///
  /// let string: &str = "0xb608c74283f334e1f047dbbf1daa2407d41d4689aca67c422796f936acce16b7";
  /// let hashed: H256 = string.parse().unwrap();
  /// let filter: Filter = Filter::new().with_event("Foo");
  ///
  /// assert_eq!(filter.topics[0], FilterSet::from(hashed));
  /// ```
  #[inline]
  #[must_use]
  pub fn with_event(mut self, value: &str) -> Self {
    self.event(value);
    self
  }

  /// Sets the value of the topic at index `0`.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::Filter;
  /// use ueb3_types::rpc::FilterSet;
  ///
  /// let string: &str = "0xb608c74283f334e1f047dbbf1daa2407d41d4689aca67c422796f936acce16b7";
  /// let hashed: H256 = string.parse().unwrap();
  /// let filter: Filter = Filter::new().with_topic0(hashed);
  ///
  /// assert_eq!(filter.topics[0], FilterSet::from(hashed));
  #[inline]
  #[must_use]
  pub fn with_topic0<T>(mut self, value: T) -> Self
  where
    T: Into<FilterSet<H256>>,
  {
    self.topic0(value);
    self
  }

  /// Sets the value of the topic at index `1`.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::Filter;
  /// use ueb3_types::rpc::FilterSet;
  ///
  /// let string: &str = "0xb608c74283f334e1f047dbbf1daa2407d41d4689aca67c422796f936acce16b7";
  /// let hashed: H256 = string.parse().unwrap();
  /// let filter: Filter = Filter::new().with_topic1(hashed);
  ///
  /// assert_eq!(filter.topics[1], FilterSet::from(hashed));
  #[inline]
  #[must_use]
  pub fn with_topic1<T>(mut self, value: T) -> Self
  where
    T: Into<FilterSet<H256>>,
  {
    self.topic1(value);
    self
  }

  /// Sets the value of the topic at index `2`.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::Filter;
  /// use ueb3_types::rpc::FilterSet;
  ///
  /// let string: &str = "0xb608c74283f334e1f047dbbf1daa2407d41d4689aca67c422796f936acce16b7";
  /// let hashed: H256 = string.parse().unwrap();
  /// let filter: Filter = Filter::new().with_topic2(hashed);
  ///
  /// assert_eq!(filter.topics[2], FilterSet::from(hashed));
  #[inline]
  #[must_use]
  pub fn with_topic2<T>(mut self, value: T) -> Self
  where
    T: Into<FilterSet<H256>>,
  {
    self.topic2(value);
    self
  }

  /// Sets the value of the topic at index `3`.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eth::H256;
  /// use ueb3_types::rpc::Filter;
  /// use ueb3_types::rpc::FilterSet;
  ///
  /// let string: &str = "0xb608c74283f334e1f047dbbf1daa2407d41d4689aca67c422796f936acce16b7";
  /// let hashed: H256 = string.parse().unwrap();
  /// let filter: Filter = Filter::new().with_topic3(hashed);
  ///
  /// assert_eq!(filter.topics[3], FilterSet::from(hashed));
  #[inline]
  #[must_use]
  pub fn with_topic3<T>(mut self, value: T) -> Self
  where
    T: Into<FilterSet<H256>>,
  {
    self.topic3(value);
    self
  }
}

impl Serialize for Filter {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut fields: usize = 0;

    fields += usize::from(!self.address.is_empty());
    fields += usize::from(self.has_topics());

    match self.block {
      FilterBlock::Exact(_) => fields += 1,
      FilterBlock::Range(_) => fields += 2,
      FilterBlock::Empty => {}
    }

    let mut state: S::SerializeStruct = serializer.serialize_struct("Filter", fields)?;

    match self.block {
      FilterBlock::Exact(ref inner) => {
        state.serialize_field("blockHash", inner)?;
        state.skip_field("fromBlock")?;
        state.skip_field("toBlock")?;
      }
      FilterBlock::Range(inner) => {
        state.skip_field("blockHash")?;
        state.serialize_field("fromBlock", &inner.src)?;
        state.serialize_field("toBlock", &inner.dst)?;
      }
      FilterBlock::Empty => {
        state.skip_field("blockHash")?;
        state.skip_field("fromBlock")?;
        state.skip_field("toBlock")?;
      }
    }

    if self.address.is_empty() {
      state.skip_field("address")?;
    } else {
      state.serialize_field("address", &self.address)?;
    }

    let mut items: [Option<&FilterSet<H256>>; 4] = [None; 4];
    let mut write: usize = 0;

    for (index, topic) in self.topics.iter().enumerate() {
      if !topic.is_empty() {
        items[index] = Some(topic);
        write = index + 1;
      }
    }

    if write == 0 {
      state.skip_field("topics")?;
    } else {
      state.serialize_field("topics", &items[..write])?;
    }

    state.end()
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_ser_tokens;

  use crate::eth::Address;
  use crate::eth::H256;
  use crate::rpc::Filter;

  #[test]
  fn test_serde() {
    let a: Filter = Filter::new();
    let b: Filter = Filter::new().with_block(H256::NULL);
    let c: Filter = Filter::new().with_address(Address::NULL);

    let d: Filter = Filter::new()
      .with_address(Address::NULL)
      .with_event("Foo")
      .with_topic2(H256::NULL);

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "Filter",
        len: 0,
      },
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "Filter",
        len: 1,
      },
      Token::Str("blockHash"),
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::StructEnd,
    ];

    let tokens_c: &[Token] = &[
      Token::Struct {
        name: "Filter",
        len: 1,
      },
      Token::Str("address"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::StructEnd,
    ];

    let tokens_d: &[Token] = &[
      Token::Struct {
        name: "Filter",
        len: 2,
      },
      Token::Str("address"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("topics"),
      Token::Seq { len: Some(3) },
      Token::Some,
      Token::Str("0xb608c74283f334e1f047dbbf1daa2407d41d4689aca67c422796f936acce16b7"),
      Token::None,
      Token::Some,
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::SeqEnd,
      Token::StructEnd,
    ];

    assert_ser_tokens(&a.readable(), tokens_a);
    assert_ser_tokens(&b.readable(), tokens_b);
    assert_ser_tokens(&c.readable(), tokens_c);
    assert_ser_tokens(&d.readable(), tokens_d);
  }
}
