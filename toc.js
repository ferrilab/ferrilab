// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="dedication.html">Dedication</a></li><li class="chapter-item expanded "><a href="introduction.html"><strong aria-hidden="true">1.</strong> Introduction to Ferrilab</a></li><li class="chapter-item expanded "><a href="radium.html"><strong aria-hidden="true">2.</strong> Radium</a></li><li class="chapter-item expanded "><a href="funty.html"><strong aria-hidden="true">3.</strong> funty</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="funty/permission-changes.html"><strong aria-hidden="true">3.1.</strong> Permission Changes</a></li></ol></li><li class="chapter-item expanded "><a href="bitvec.html"><strong aria-hidden="true">4.</strong> bitvec</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="bitvec/data-structures.html"><strong aria-hidden="true">4.1.</strong> Data Structures</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="bitvec/data-structures/bitslice.html"><strong aria-hidden="true">4.1.1.</strong> BitSlice</a></li><li class="chapter-item expanded "><a href="bitvec/data-structures/bitarray.html"><strong aria-hidden="true">4.1.2.</strong> BitArray</a></li><li class="chapter-item expanded "><a href="bitvec/data-structures/bitvec.html"><strong aria-hidden="true">4.1.3.</strong> BitVec and BitBox</a></li></ol></li><li class="chapter-item expanded "><a href="bitvec/type-parameters.html"><strong aria-hidden="true">4.2.</strong> Type Parameters</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="bitvec/type-parameters/bitorder.html"><strong aria-hidden="true">4.2.1.</strong> BitOrder</a></li><li class="chapter-item expanded "><a href="bitvec/type-parameters/bitstore.html"><strong aria-hidden="true">4.2.2.</strong> BitStore</a></li></ol></li><li class="chapter-item expanded "><a href="bitvec/practical-use.html"><strong aria-hidden="true">4.3.</strong> Practical Use</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="bitvec/practical-use/collections.html"><strong aria-hidden="true">4.3.1.</strong> bool Collections</a></li><li class="chapter-item expanded "><a href="bitvec/practical-use/bitfields.html"><strong aria-hidden="true">4.3.2.</strong> C-Style Bitfields</a></li></ol></li><li class="chapter-item expanded "><a href="bitvec/memory-representation.html"><strong aria-hidden="true">4.4.</strong> Memory Representation</a></li><li class="chapter-item expanded "><a href="bitvec/memory-model.html"><strong aria-hidden="true">4.5.</strong> Memory Model</a></li><li class="chapter-item expanded "><a href="bitvec/performance.html"><strong aria-hidden="true">4.6.</strong> Performance</a></li><li class="chapter-item expanded "><a href="bitvec/pointer-encoding.html"><strong aria-hidden="true">4.7.</strong> Pointer Encoding</a></li><li class="chapter-item expanded "><a href="bitvec/miscellaneous.html"><strong aria-hidden="true">4.8.</strong> Other Features</a></li></ol></li><li class="chapter-item expanded "><a href="afterword.html">Afterword</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
