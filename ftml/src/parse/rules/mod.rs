/*
 * parse/rules/mod.rs
 *
 * wikidot-html - Convert Wikidot code to HTML
 * Copyright (C) 2019 Ammon Smith for Project Foundation
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

mod object;

use self::object::Rule::{self, *};

// Copied from Wikidot Text_Wiki source
// For maximum backwards-compatibility, leave as-is
pub const RULES: [Rule; 68] = [
    Include,
    Prefilter,
    Delimeter,
    Code,
    Form,
    Raw,
    RawOld, // ?
    ModulePre, // ?
    Module,
    Module654, // ?
    IfTags,
    Comment,
    IFrame,
    Date,
    Math,
    ConcatLines,
    FreeLink,
    EquationReference,
    Footnote,
    FootnoteItem,
    FootnoteBlock,
    BibItem,
    Bibliography,
    BibCite,
    Html,
    DivPrefilter,
    Anchor,
    User,
    Blockquote,
    Heading,
    Toc,
    Horiz,
    Separator,
    ClearFloat,
    Break,
    Span,
    Size,
    Div,
    DivAlign,
    Collapsible,
    TabView,
    Note,
    Gallery,
    List,
    DefList,
    Table,
    TableAdv,
    Image,
    Embed,
    Social,
    File,
    Center,
    Newline,
    Paragraph,
    Url,
    Email,
    MathInline,
    Interwiki,
    Colortext,
    Strong,
    Emphasis,
    Underline,
    Strikethrough,
    Teletype,
    Superscript,
    Subscript,
    Typography,
    Tighten,
];

#[test]
fn test_variants() {
    use crate::SyntaxTree;

    let mut tree = SyntaxTree;

    for rule in &RULES[..] {
        rule.apply(&mut tree);
    }
}
