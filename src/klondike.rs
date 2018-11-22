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

type TableauSlotArray = [Card; 19];
pub struct TableauSlot(ArrayVec<TableauSlotArray>);

NewtypeFrom! { () pub struct TableauSlot(ArrayVec<TableauSlotArray>); }

impl TableauSlot
{
    pub fn new () -> TableauSlot
    {
        TableauSlot::from(ArrayVec::<TableauSlotArray>::new())
    }

    pub fn push (&mut self, element: <TableauSlotArray as arrayvec::Array>::Item)
    {
        ArrayVec::<TableauSlotArray>::push(&mut self.0, element)
    }
}

impl Deref for TableauSlot
{
    type Target = [<TableauSlotArray as arrayvec::Array>::Item];

    fn deref (&self) -> &[<TableauSlotArray as arrayvec::Array>::Item]
    {
        ArrayVec::<TableauSlotArray>::deref(&self.0)
    }
}

pub struct Table
{
    pub tableau: [TableauSlot; 7],
    //pub foundation: [FoundationSlot; 4],
}
