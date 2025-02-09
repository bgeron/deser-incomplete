#[cfg(doc)]
use serde::de::SeqAccess;

use crate::attempt::HaltingPoint;
use crate::error::InternalError;
use crate::options_impl::ExtraOptions;
use crate::reporter::Reporter;
use crate::Options;

pub(crate) struct GlobalState<Extra: crate::options_impl::ExtraOptions> {
    /// Set to 1 during first attempt
    pub(super) n_backtracks: usize,

    // technically we don't have to keep the Extra value field of Options
    pub(super) config: Options<Extra>,
    pub(super) reporter: Extra::Reporter,
    pub(super) fallbacks: Extra::FallbackProvider,
}

pub(crate) struct AttemptState<Extra: crate::options_impl::ExtraOptions> {
    pub(super) reporter: Extra::Reporter,

    /// If the previous attempt failed, then there may be a point where we can tell
    /// the visitor there's no more data (e.g. in the sequence or map) and safely
    /// finish deserialization.
    intend_to_stop_deserializing_at: Option<HaltingPoint>,

    /// Whether we have intervened in deserialization, and what the cause originally was.
    intervention_active: Option<Intervention>,

    next_halting_point: HaltingPoint,

    /// Stack of points where we may halt deserialization on the next attempt.
    ///
    /// For instance, if deserializing a field failed, then on the next attempt it
    /// can make sense to halt just before that field (pretend the field is absent).
    /// But if that doesn't work then the next best thing is to halt one level up, etc.
    ///
    /// On returning an error from an attempt, this field will remain intact as of the
    /// point of the original error.
    pub(super) halting_point_stack: Vec<HaltingPoint>,
}

#[derive(Clone, Debug)]
pub(crate) struct Intervention {
    reason: InterventionReason,
    candidate_halting_point_for_next_attempt: Option<HaltingPoint>,
}

#[derive(Clone, Debug)]
pub(crate) enum InterventionReason {
    /// The deserializer has returned an error before calling the visitor.
    /// (We do not distinguish between its errors.)
    DeserializerStart,
    /// The deserializer returned an error after the visitor succeeded. In this
    /// case, we can salvage the value returned by the visitor.
    DeserializerFinishSaved,
    /// We planned to halt deserialization at a certain point, and we have reached that point.
    PlannedHalting {
        #[allow(dead_code)]
        at: HaltingPoint,
    },
    /// A visitor (data type) returned an error. We may or may not have applied a
    /// fallback.
    ///
    /// This may well be recoverable, e.g. if the error happens inside [`SeqAccess`] or
    /// a map key.
    VisitError,
}

impl<Extra: ExtraOptions> Options<Extra> {
    pub(crate) fn build(mut self) -> GlobalState<Extra> {
        let reporter = self.extra.make_reporter();
        let fallbacks = self.extra.make_fallback_provider(&self.behavior);
        GlobalState {
            n_backtracks: 0,
            config: self,
            reporter,
            fallbacks,
        }
    }
}

impl<Extra: ExtraOptions> AttemptState<Extra> {
    pub(crate) fn initial(global: &GlobalState<Extra>) -> Self {
        Self {
            reporter: global.reporter.clone(),
            intend_to_stop_deserializing_at: None,
            intervention_active: None,
            next_halting_point: HaltingPoint::default(),
            halting_point_stack: Vec::new(),
        }
    }

    /// When an attempt failed, then compute the state to start next attempt with -- if
    /// we know a way to potentially do better next attempt.
    ///
    /// Logs to tracing accordingly.
    pub(crate) fn next_attempt_state_after_failure(
        mut self,
    ) -> Result<Option<Self>, InternalError> {
        match self.intervention_active.take() {
            Some(Intervention {
                reason,
                candidate_halting_point_for_next_attempt: Some(next_halting_point),
            }) => {
                trace!(
                    ?next_halting_point,
                    ?reason,
                    "creating state for next attempt"
                );

                self.halting_point_stack.clear();
                Ok(Some(Self {
                    reporter: self.reporter,
                    intend_to_stop_deserializing_at: Some(next_halting_point),
                    intervention_active: None,
                    next_halting_point: HaltingPoint::default(),
                    halting_point_stack: self.halting_point_stack,
                }))
            }
            None => {
                debug!(
                    "no reason recorded for failure of deserialization attempt; please report a bug against {} if there is no bug report open",
                    env!("CARGO_PKG_NAME")
                );
                Ok(None)
            }
            Some(Intervention {
                reason,
                candidate_halting_point_for_next_attempt: None,
            }) => {
                trace!(failed_because=?reason, "no halting point active after attempt, giving up");
                Ok(None)
            }
        }
    }

    fn get_next_halting_point(&mut self) -> HaltingPoint {
        let next = self.next_halting_point.clone();
        self.next_halting_point.increment();
        next
    }

    /// A new halting point will now happen. Return the value of the current halting point
    /// if we're supposed to continue past this.
    ///
    /// Activates intervention when applicable.
    pub(crate) fn new_halting_point_and_check_continue(&mut self) -> Option<HaltingPoint> {
        let this_halting_point = self.get_next_halting_point();
        self.reporter.report_new_halting_point(&this_halting_point);

        match self.intend_to_stop_deserializing_at.clone() {
            Some(stop) if *stop <= *this_halting_point => {
                if *stop < *this_halting_point {
                    error!(
                        intend_to_stop_deserializing_at=?stop,
                        ?this_halting_point,
                        "we wanted to stop at a halting point, but continued past it"
                    );
                }
                self.activate_intervention(InterventionReason::PlannedHalting { at: stop });
                None
            }
            _ => Some(this_halting_point),
        }
    }

    pub(crate) fn intervention_is_empty(&self) -> bool {
        self.intervention_active.is_none()
    }

    /// If no intervention is active yet, then set a reason for intervention,
    /// and remember a potential better halting point for next attempt.
    pub(crate) fn activate_intervention(&mut self, reason: InterventionReason) {
        if self.intervention_active.is_none() {
            let candidate_halting_point_for_next_attempt = self.halting_point_stack.last().cloned();

            self.reporter.report_start_intervention(
                &reason,
                candidate_halting_point_for_next_attempt.as_ref(),
                &self.halting_point_stack,
            );

            self.intervention_active = Some(Intervention {
                reason,
                candidate_halting_point_for_next_attempt,
            });
        }
    }
}
