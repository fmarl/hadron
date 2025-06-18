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

use self::default::WRITER;
use core::fmt::{Arguments, Write};
use generic_exception::hcf;
use limine::{Framebuffer, NonNullPtr};

mod default;
mod font;

static FRAMEBUFFER_REQUEST: limine::request::FramebufferRequest =
    limine::request::FramebufferRequest::new();

pub fn init() -> &'static NonNullPtr<Framebuffer> {
    let framebuffer: &NonNullPtr<Framebuffer>;

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
