/* ============================================================
   MCInfo Docs — Navigation (sidebar, mobile, TOC)
   ============================================================ */

(function () {
  'use strict';

  // --- Sidebar: highlight active link ---
  const path = window.location.pathname.replace(/\/$/, '').replace(/\/index\.html$/, '');
  document.querySelectorAll('.sidebar-links a').forEach(function (a) {
    const href = a.getAttribute('href').replace(/\/$/, '').replace(/\/index\.html$/, '');
    if (path.endsWith(href) || path.endsWith(href.replace('.html', ''))) {
      a.classList.add('active');
      // Expand parent section
      const section = a.closest('.sidebar-section');
      if (section) {
        const heading = section.querySelector('.sidebar-heading');
        const links = section.querySelector('.sidebar-links');
        if (heading) heading.classList.remove('collapsed');
        if (links) links.classList.remove('collapsed');
      }
    }
  });

  // --- Sidebar: collapsible sections ---
  document.querySelectorAll('.sidebar-heading').forEach(function (heading) {
    heading.addEventListener('click', function () {
      const links = this.nextElementSibling;
      if (!links) return;
      const isCollapsed = this.classList.toggle('collapsed');
      links.classList.toggle('collapsed', isCollapsed);
    });
  });

  // --- Mobile menu toggle ---
  var toggle = document.querySelector('.menu-toggle');
  var sidebar = document.querySelector('.sidebar');
  var overlay = document.querySelector('.sidebar-overlay');

  if (toggle && sidebar) {
    toggle.addEventListener('click', function () {
      sidebar.classList.toggle('open');
      if (overlay) overlay.classList.toggle('open');
    });
    if (overlay) {
      overlay.addEventListener('click', function () {
        sidebar.classList.remove('open');
        overlay.classList.remove('open');
      });
    }
  }

  // --- Table of Contents auto-generation ---
  var tocList = document.querySelector('.toc-list');
  var main = document.querySelector('.main .prose');
  if (tocList && main) {
    var headings = main.querySelectorAll('h2, h3');
    headings.forEach(function (h) {
      if (!h.id) {
        h.id = h.textContent.trim()
          .toLowerCase()
          .replace(/[^a-z0-9]+/g, '-')
          .replace(/(^-|-$)/g, '');
      }
      var li = document.createElement('li');
      var a = document.createElement('a');
      a.href = '#' + h.id;
      a.textContent = h.textContent;
      if (h.tagName === 'H3') a.classList.add('toc-h3');
      li.appendChild(a);
      tocList.appendChild(li);
    });

    // TOC scroll spy
    if (headings.length > 0) {
      var tocLinks = tocList.querySelectorAll('a');
      var onScroll = function () {
        var scrollY = window.scrollY + 100;
        var current = null;
        headings.forEach(function (h) {
          if (h.offsetTop <= scrollY) current = h;
        });
        tocLinks.forEach(function (a) {
          a.classList.toggle('active', current && a.getAttribute('href') === '#' + current.id);
        });
      };
      window.addEventListener('scroll', onScroll, { passive: true });
      onScroll();
    }
  }
})();
