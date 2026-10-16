use fluent_bundle::concurrent::FluentBundle;
use fluent_bundle::{FluentArgs, FluentResource};
use fluent_langneg::{NegotiationStrategy, negotiate_languages};
use std::collections::HashSet;
use std::sync::Mutex;
use unic_langid::LanguageIdentifier;

/// Un fichier `.ftl` embarqué (`include_str!`) pour une locale.
pub struct Resource {
    pub locale: LanguageIdentifier,
    pub source: &'static str,
}

#[derive(Debug, thiserror::Error)]
pub enum I18nError {
    #[error("erreurs Fluent pour la locale {locale} : {errors:?}")]
    Parse { locale: String, errors: Vec<String> },
    #[error("aucune ressource pour la langue de repli")]
    NoFallback,
}

pub struct Localizer {
    bundles: Vec<(LanguageIdentifier, FluentBundle<FluentResource>)>,
    fallback: LanguageIdentifier,
    current: LanguageIdentifier,
    warned: Mutex<HashSet<String>>,
}

impl Localizer {
    pub fn new(resources: Vec<Resource>, fallback: LanguageIdentifier) -> Result<Self, I18nError> {
        let mut bundles: Vec<(LanguageIdentifier, FluentBundle<FluentResource>)> = Vec::new();
        for r in resources {
            let parse_err = |errors: Vec<String>| I18nError::Parse {
                locale: r.locale.to_string(),
                errors,
            };
            let res = FluentResource::try_new(r.source.to_owned())
                .map_err(|(_, e)| parse_err(e.iter().map(|e| format!("{e:?}")).collect()))?;
            let idx = match bundles.iter().position(|(l, _)| *l == r.locale) {
                Some(i) => i,
                None => {
                    let mut b = FluentBundle::new_concurrent(vec![r.locale.clone()]);
                    b.set_use_isolating(false);
                    bundles.push((r.locale.clone(), b));
                    bundles.len() - 1
                }
            };
            bundles[idx]
                .1
                .add_resource(res)
                .map_err(|e| parse_err(e.iter().map(|e| format!("{e:?}")).collect()))?;
        }
        if !bundles.iter().any(|(l, _)| *l == fallback) {
            return Err(I18nError::NoFallback);
        }
        Ok(Self {
            bundles,
            current: fallback.clone(),
            fallback,
            warned: Mutex::new(HashSet::new()),
        })
    }

    /// Choisit la meilleure locale disponible pour `requested` (repli sinon) et la renvoie.
    pub fn negotiate(&mut self, requested: &[LanguageIdentifier]) -> LanguageIdentifier {
        let available = self.available();
        let chosen = negotiate_languages(
            requested,
            &available,
            Some(&self.fallback),
            NegotiationStrategy::Filtering,
        )
        .first()
        .map_or_else(|| self.fallback.clone(), |l| (*l).clone());
        self.current = chosen.clone();
        chosen
    }

    pub fn current(&self) -> &LanguageIdentifier {
        &self.current
    }

    pub fn available(&self) -> Vec<LanguageIdentifier> {
        self.bundles.iter().map(|(l, _)| l.clone()).collect()
    }

    /// Traduit `id` dans la locale courante, puis la langue de repli ; sinon renvoie `id`.
    pub fn tr(&self, id: &str, args: Option<&FluentArgs>) -> String {
        for locale in [&self.current, &self.fallback] {
            if let Some(s) = self.format(locale, id, args) {
                return s;
            }
        }
        if let Ok(mut warned) = self.warned.lock()
            && warned.insert(id.to_owned())
        {
            tracing::warn!(id, "message Fluent introuvable");
        }
        id.to_owned()
    }

    fn format(
        &self,
        locale: &LanguageIdentifier,
        id: &str,
        args: Option<&FluentArgs>,
    ) -> Option<String> {
        let (_, bundle) = self.bundles.iter().find(|(l, _)| l == locale)?;
        let pattern = bundle.get_message(id)?.value()?;
        let mut errors = Vec::new();
        let s = bundle.format_pattern(pattern, args, &mut errors);
        if !errors.is_empty() {
            tracing::warn!(id, ?errors, "erreurs de formatage Fluent");
        }
        Some(s.into_owned())
    }
}
