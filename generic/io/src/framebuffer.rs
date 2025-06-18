/*
 * This file is part of the hadron distribution (https://github.com/fxttr/hadron).
 * Copyright (c) 2025 Florian Marrero Liestmann.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use self::vga::WRITER;
use core::{fmt::{Arguments, Write}, u64};
use generic_exception::hcf;

mod vga;
mod font;

pub trait Framebuffer {
    fn new() -> Self;
}

pub fn init<F>() -> &'static F
where F: Framebuffer  {
    let framebuffer: F;

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        if framebuffer_response.framebuffer_count < 1 {
            hcf();
        }

        // Get the first framebuffer's information.
        framebuffer = &framebuffer_response.framebuffers()[0];
    } else {
        hcf();
    }

    framebuffer
}

pub fn _kprint(args: Arguments<'_>) {
    let _ = WRITER.lock().write_fmt(args);
}
