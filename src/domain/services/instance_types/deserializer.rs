use std::collections::HashMap;

use log::debug;

use crate::domain::model::InstanceType;
use crate::utils::memory::parse_memory_in_bytes;
use serde::{
  de::{MapAccess, Visitor},
  Deserialize, Deserializer,
};

const INSTANCE_TYPE_ATTR: &str = "instanceType";
const INSTANCE_FAMILY_ATTR: &str = "instanceFamily";
const MEMORY_ATTR: &str = "memory";
const VCPU_ATTR: &str = "vcpu";
const GPU_ATTR: &str = "gpu";
const OPERATING_SYSTEM_ATTR: &str = "operatingSystem";
const OPERATING_SYSTEM_LINUX: &str = "Linux";

#[derive(Deserialize, Clone, Debug, PartialEq)]
struct RawProduct {
  pub attributes: HashMap<String, String>,
}

pub fn deserialize_instance_types<'de, D>(deserializer: D) -> Result<Vec<InstanceType>, D::Error>
where
  D: Deserializer<'de>,
{
  struct ProductsVisitor;

  impl<'de> Visitor<'de> for ProductsVisitor {
    type Value = Vec<InstanceType>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
      formatter.write_str("a map of instance_types")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
      A: MapAccess<'de>,
    {
      let mut instance_types = HashMap::<String, InstanceType>::new();

      while let Some((_, product)) = map.next_entry::<String, RawProduct>()? {
        let operating_system = product.attributes.get(OPERATING_SYSTEM_ATTR).map(String::as_str);

        if let Some(OPERATING_SYSTEM_LINUX) = operating_system {
          let maybe_instance_type_name = product.attributes.get(INSTANCE_TYPE_ATTR).cloned();

          let instance_family = product
            .attributes
            .get(INSTANCE_FAMILY_ATTR)
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string());

          let maybe_memory = product
            .attributes
            .get(MEMORY_ATTR)
            .and_then(|value| parse_memory_in_bytes(value.as_str()).ok());

          let maybe_vcpu = product.attributes.get(VCPU_ATTR).and_then(|value| value.parse::<usize>().ok());

          let gpu = product
            .attributes
            .get(GPU_ATTR)
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(0);

          let required_attributes = (maybe_instance_type_name, maybe_memory, maybe_vcpu);
          if let (Some(instance_type_name), Some(memory), Some(vcpu)) = required_attributes {
            let num_instance_types = instance_types.len();
            instance_types.entry(instance_type_name.clone()).or_insert_with(|| {
              let instance_type = InstanceType {
                name: instance_type_name,
                family: instance_family,
                memory,
                vcpu,
                gpu,
              };
              debug!("{:?} ({} in total)", instance_type, num_instance_types + 1);
              instance_type
            });
          }
        }
      }

      let instance_types = instance_types.into_iter().map(|(_, value)| value).collect();

      Ok(instance_types)
    }
  }

  deserializer.deserialize_map(ProductsVisitor)
}
