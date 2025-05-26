use crate::errors::ParseResult;
use crate::peek::{PeekResult, Peekable, Peeking};
use crate::scanner::Scanner;

/// A [Peeker] is a type that is used to find the best group to forecast
pub struct Peeker<'a, 'b, T, S, E> {
    scanner: &'b mut Scanner<'a, T>,
    /// Pool of [Peekable]
    peekables: Vec<Box<dyn Peekable<'a, T, S, E>>>,
}

impl<'a, 'b, T, S, E> Peeker<'a, 'b, T, S, E> {
    pub fn new(scanner: &'b mut Scanner<'a, T>) -> Self {
        Self {
            scanner,
            peekables: vec![],
        }
    }
}

impl<'a, 'b, T, S, E> Peeker<'a, 'b, T, S, E> {
    /// Add new [Peekable] element to the peeking pool
    pub fn add_peekable<F: Peekable<'a, T, S, E> + 'static>(mut self, forecastable: F) -> Self {
        self.peekables.push(Box::new(forecastable));
        self
    }

    /// Run the [Forecast] pool, find the minimal group
    pub fn forecast(self) -> ParseResult<Option<Peeking<'b, T, S, E>>> {
        let mut result = None;
        // on boucle sur les possibilités de prédictions
        for peekable in self.peekables.into_iter() {
            let peek_result = peekable.peek(self.scanner)?;
            // on tente de prédire l'élément
            match peek_result {
                // si l'on a trouvé quelque chose
                PeekResult::Found {
                    start,
                    end,
                    end_slice,
                } => {
                    // on récupère le groupe prédit
                    let remaining = self.scanner.remaining();
                    let data = &remaining[..end_slice];
                    let new_forecast = Peeking {
                        start,
                        end,
                        data,
                        end_slice,
                    };
                    match &result {
                        // si l'on n'a encore rien prédit du tout
                        None => {
                            // le groupe trouvé devient le résultat
                            result = Some(new_forecast);
                        }
                        // s'il y a déjà une prédiction
                        Some(min_forecast) => {
                            // on compare la taille du groupe trouvé par rapport
                            // à celui déjà trouvé
                            if new_forecast.data.len() < min_forecast.data.len() {
                                // il devient alors le nouveau groupe prédit
                                result = Some(new_forecast);
                            }
                        }
                    }
                }
                // si la prédiction échoue, on ne fait rien
                PeekResult::NotFound => {}
            }
        }
        Ok(result)
    }
}
