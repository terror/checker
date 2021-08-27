use crate::common::*;

pub fn check(name: &str) -> Result<Package> {
  let client = Client::new()?;

  let res = client.get(&format!("/api/v1/crates/{}", name))?;

  let status = Status::from(res.status());

  let json: Value = from_str(&res.text().unwrap())?;

  let data: Option<Data> = from_value(json["crate"].to_owned())?;

  let owners = find_owners(client, &data)?;

  Ok(Package::new(name.to_string(), data, owners, status))
}

fn find_owners(client: Client, data: &Option<Data>) -> Result<Option<Vec<Owner>>> {
  if let Some(data) = data {
    let json: Value = from_str(&client.get(&data.links.owner_user)?.text().unwrap())?;

    let owners: Vec<Owner> = from_value(json["users"].to_owned())?;

    return Ok(Some(owners));
  }

  Ok(None)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn free_crate() {
    let result = check("freecratenameyeet").unwrap();
    assert_eq!(result.is_taken(), false);
    assert!(result.name == *"freecratenameyeet");
    assert!(result.data.is_none());
    assert!(result.owners.is_none());
  }

  #[test]
  fn taken_crate() {
    let result = check("syn").unwrap();
    assert_eq!(result.is_taken(), true);
    assert!(result.name == *"syn");
    assert!(result.data.is_some());
    assert!(result.owners.is_some());
  }

  #[test]
  fn multiple_free_crates() {
    let crate_names = vec![
      "sdlkfjsdkeekx",
      "noonewillclaimthisxd",
      "dkjddkk333",
      "dodooodeiie",
      "sdfkljk3i22",
      "x0rowowowpp",
    ];

    for name in crate_names {
      let result = check(name).unwrap();
      assert_eq!(result.is_taken(), false);
      assert!(result.name == *name);
      assert!(result.data.is_none());
      assert!(result.owners.is_none());
    }
  }

  #[test]
  fn multiple_taken_crates() {
    let crate_names = vec![
      "syn",
      "just",
      "ff",
      "xplr",
      "run",
      "rust",
      "hack",
      "dev",
      "root",
      "file",
      "own",
      "serde",
      "serde_json",
    ];

    for name in crate_names {
      let result = check(name).unwrap();
      assert_eq!(result.is_taken(), true);
      assert!(result.name == *name);
      assert!(result.data.is_some());
      assert!(result.owners.is_some());
    }
  }

  #[test]
  fn is_inactive() {
    let result = check("t").unwrap();

    assert_eq!(result.is_inactive().unwrap(), true);
    assert_eq!(result.is_taken(), true);

    assert!(result.days_since_last_updated().unwrap() >= 1825);
    assert!(result.name == *"t");
    assert!(result.data.is_some());
    assert!(result.owners.is_some());
  }

  #[test]
  fn is_not_inactive() {
    let result = check("syn").unwrap();

    assert_eq!(result.is_inactive().unwrap(), false);
    assert_eq!(result.is_taken(), true);

    assert!(result.days_since_last_updated().unwrap() < 1825);
    assert!(result.name == *"syn");
    assert!(result.data.is_some());
    assert!(result.owners.is_some());
  }
}
