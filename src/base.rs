#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FunctionalState {
	#[doc = "Field disabled"]
	DISABLE = 0,
	#[doc = "Field enabled"]
	ENABLE = 1
}

impl From<FunctionalState> for bool {
	#[inline(always)]
    fn from(state: FunctionalState) -> Self {
        match state {
			FunctionalState::DISABLE => false,
			FunctionalState::ENABLE => true
		}
    }
}

impl From<bool> for FunctionalState {
	#[inline(always)]
    fn from(state: bool) -> Self {
        match state {
			false => FunctionalState::DISABLE,
			true => FunctionalState::ENABLE
		}
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ErrorStatus {
	#[doc = "Error status"]
	ERROR = 0,
	#[doc = "Succes Status"]
	SUCCESS = 1
}

impl From<ErrorStatus> for bool {
	#[inline(always)]
    fn from(state: ErrorStatus) -> Self {
        match state {
			ErrorStatus::ERROR => false,
			ErrorStatus::SUCCESS => true
		}
    }
}

impl From<bool> for ErrorStatus {
	#[inline(always)]
    fn from(state: bool) -> Self {
        match state {
			false => ErrorStatus::ERROR,
			true => ErrorStatus::SUCCESS
		}
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlagStatus {
	RESET = 0,
	SET   = 1
}

impl From<FlagStatus> for bool {
	#[inline(always)]
    fn from(state: FlagStatus) -> Self {
        match state {
			FlagStatus::RESET => false,
			FlagStatus::SET => true
		}
    }
}

impl From<bool> for FlagStatus {
	#[inline(always)]
    fn from(state: bool) -> Self {
        match state {
			false => FlagStatus::RESET,
			true => FlagStatus::SET
		}
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITStatus {
	RESET = 0,
	SET   = 1
}

impl From<ITStatus> for bool {
	#[inline(always)]
    fn from(state: ITStatus) -> Self {
        match state {
			ITStatus::RESET => false,
			ITStatus::SET => true
		}
    }
}

impl From<bool> for ITStatus {
	#[inline(always)]
    fn from(state: bool) -> Self {
        match state {
			false => ITStatus::RESET,
			true => ITStatus::SET
		}
    }
}
