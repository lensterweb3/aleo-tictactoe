// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{ScopeError, TypeAssertionError};
use leo_static_check::TypeError;
use leo_typed::{Error as FormattedError, Span};

use std::path::PathBuf;

/// Errors encountered when tracking variable names in a program.
#[derive(Debug, Error)]
pub enum FrameError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    ScopeError(#[from] ScopeError),

    #[error("{}", _0)]
    TypeAssertionError(#[from] TypeAssertionError),

    #[error("{}", _0)]
    TypeError(#[from] TypeError),
}

impl FrameError {
    ///
    /// Set the filepath for the error stacktrace
    ///
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            FrameError::Error(error) => error.set_path(path),
            FrameError::ScopeError(error) => error.set_path(path),
            FrameError::TypeAssertionError(error) => error.set_path(path),
            FrameError::TypeError(error) => error.set_path(path),
        }
    }

    // ///
    // /// Return a new formatted error with a given message and span information
    // ///
    // fn new_from_span(message: String, span: Span) -> Self {
    //     FrameError::Error(FormattedError::new_from_span(message, span))
    // }
}
