# Comment gérer le numéro de version

Ne sachant pas comment gérer ce numéro de version dans mes projets, et plus particulièrement ceux en Rust, j'ai été fouiller dans des projets à fortes respectabilités pour voir comment eux s'y prennaient.

Les projets en question :
- [ripgrep](https://github.com/BurntSushi/ripgrep)
- [exa](https://github.com/ogham/exa)
- [alacritty](https://github.com/jwilm/alacritty)

Si l'on souhaite simplement récupérer le numéro de version stocké dans le fichier `Cargo.toml`, la méthode simple suffit amplement. Par contre, pour un numéro de version plus complexe avec une date et/ou le hash git, il faut passer par un « [Build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) »

Cargo exécute ce « script » rust avant la compilation en elle-même, et permet d'effectuer un ensemble d'actions, dont il pourra communiquer le résultat à la compilation principale via la création de variable d'environnement.

Dans mon cas, je vais récupérer la version depuis le fichier `Cargo.toml`, le hash git et la date de la compilation pour avoir un numéro de version du format suivant : `v1.2.3 (decba7e built on 2019-10-29)`.

À noter que l'utilisation de ce script ne se limite pas qu'à la gestion du numéro de version, il est également possible de s'en servir pour générer les « man page » et la documentation du projet.

## La manière simple

Dans le fichier `Cargo.toml` modifier le numéro de version

```toml
version = "1.2.3"
```

Et dans le source, un simple appel à la variable d'enviromment correspondante

  ```rust
  use std::env;
  fn main() {
    println!("{}", env::var("CARGO_PKG_VERSION").unwrap());
  }
  ```

## Via « Build script »

Dans un premier temps, dans le fichier `Cargo.toml` il faut renseigner le script à exécuter avant la compilation, ainsi que les dépendances nécessaires dans une partie spécifique du fichier.

Pour faciliter, la récupéreration du hash git, j'utilise le paquet [rustc_tools_util](https://github.com/rust-lang/rust-clippy/tree/master/rustc_tools_util).
  ```toml
  [package]
    ...
    build = "build.rs"
    ...
  [build-dependencies]
    datetime = "0.4.7"
    rustc_tools_util = "0.1"
  ```

On crée le fichier `build.rs` avec les traitements souhaités.

  ```rust
  ...
  println!(
    "cargo:rustc-env=GIT_HASH={}",
    rustc_tools_util::get_commit_hash().unwrap_or_default()
  );
  ...
  ```

Enfin pour créer le numéro de série dans le code source :

  ```rust
  use std::env;
  fn main() {
    ...
    println!(
      "Avec les informations de build.rs(MYPROJECT_VERSION) : {}",
      format!(
        "v{} ({} built on {})",
        env::var("CARGO_PKG_VERSION").unwrap(),
        env::var("GIT_HASH").unwrap(),
        env::var("BUILD_DATE").unwrap(),
      )
    );
    ...
  }
  ```
