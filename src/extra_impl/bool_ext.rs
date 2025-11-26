// Copyright © 2025 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait BoolExt {
    fn toggle(&mut self);
}

impl BoolExt for bool {
    fn toggle(&mut self) {
        *self = !*self;
    }
}
