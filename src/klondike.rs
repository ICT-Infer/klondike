/*
 * Copyright (c) 2018 Erik Nordstrøm <erik@nordstroem.no>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

use arrayvec::ArrayVec;

use std::ops::Deref;

use cards::Card;

macro_rules! impl_cardstack
{
    ($t:ident, $a:ty) =>
    {
        impl_cardstack_ops!($t, $a);

        impl $t
        {
            pub fn new () -> $t
            {
                $t::from(ArrayVec::<$a>::new())
            }
        }
    }
}

impl_cardstack!(TableauSlot, [Card; 19]);

pub struct Table
{
    pub tableau: [TableauSlot; 7],
    //pub foundation: [FoundationSlot; 4],
}
