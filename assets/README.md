# SVG Logos

The Ferrilab project and crate logos are SVG images. They contain embedded CSS
that reacts to the browser’s light or dark theme.

They are embedded into the documentation in two ways. Ordinary Markdown
processors can use ordinary image links, but Rustdoc strips external image
embeds.

Rustdoc exposes two attributes for setting the favicon and crate logo. We supply
these with base64-encoded versions of the SVG text.

The title heading in each crate’s README document (which becomes the crate-level
documentation in `src/lib.rs`) receives embedded CSS in the README. It exploits
a difference between ordinary and Rustdoc rendering contexts: Rustdoc implicitly
increases each heading rank by 1, so `# ![][logo]` becomes `h2 img` in Rustdoc
but `h1 img` everywhere else.

Each README contains CSS which sets `h2 > img` to have a background image of the
base64-encoded crate logo. This selector only matches inside Rustdoc. The end
result is that previewing the README documents, rendering crate documentation
with Rustdoc, and rendering the guide/manual with mdBook, all use the logos for
the project and crate titles.

Crate assets are stored in `<crate>/assets`. Project assets are stored here. The
book has symlinks to the project and crate asset files.

The SVG files are minified and converted to base64 using
<https://www.svgviewer.dev/svg-to-data-uri>.
