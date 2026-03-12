/* ============================================================
   MCInfo Docs — Client-side search
   ============================================================ */

(function () {
  'use strict';

  var index = null;
  var input = document.querySelector('.search-input');
  var results = document.querySelector('.search-results');
  var selectedIdx = -1;

  if (!input || !results) return;

  // Resolve base path for assets (works on GH Pages and locally)
  var base = document.querySelector('link[rel="icon"]');
  var basePath = '';
  if (base) {
    var href = base.getAttribute('href');
    basePath = href.substring(0, href.lastIndexOf('/assets/'));
  }

  function loadIndex() {
    if (index) return Promise.resolve(index);
    return fetch(basePath + '/search-index.json')
      .then(function (r) { return r.json(); })
      .then(function (data) { index = data; return data; });
  }

  function search(query) {
    if (!index || !query) return [];
    var terms = query.toLowerCase().split(/\s+/).filter(Boolean);
    var scored = [];

    index.forEach(function (item) {
      var titleLower = item.title.toLowerCase();
      var contentLower = item.content.toLowerCase();
      var score = 0;

      terms.forEach(function (term) {
        if (titleLower.includes(term)) score += 10;
        if (contentLower.includes(term)) score += 1;
        // Exact title match bonus
        if (titleLower === query.toLowerCase()) score += 50;
      });

      if (score > 0) {
        // Find snippet
        var snippet = '';
        var pos = contentLower.indexOf(terms[0]);
        if (pos !== -1) {
          var start = Math.max(0, pos - 40);
          var end = Math.min(item.content.length, pos + 80);
          snippet = (start > 0 ? '...' : '') + item.content.substring(start, end) + (end < item.content.length ? '...' : '');
        }
        scored.push({ item: item, score: score, snippet: snippet });
      }
    });

    scored.sort(function (a, b) { return b.score - a.score; });
    return scored.slice(0, 10);
  }

  function highlightTerms(text, query) {
    if (!text || !query) return text;
    var terms = query.toLowerCase().split(/\s+/).filter(Boolean);
    var result = text;
    terms.forEach(function (term) {
      var re = new RegExp('(' + term.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') + ')', 'gi');
      result = result.replace(re, '<mark>$1</mark>');
    });
    return result;
  }

  function renderResults(matches, query) {
    selectedIdx = -1;
    if (matches.length === 0) {
      results.innerHTML = '<div class="search-no-results">No results for "' +
        query.replace(/</g, '&lt;') + '"</div>';
      results.classList.add('active');
      return;
    }
    var html = '';
    matches.forEach(function (m, i) {
      html += '<a class="search-result-item" href="' + basePath + '/' + m.item.url + '" data-idx="' + i + '">' +
        '<div class="search-result-section">' + m.item.section + '</div>' +
        '<div class="search-result-title">' + highlightTerms(m.item.title, query) + '</div>' +
        (m.snippet ? '<div class="search-result-snippet">' + highlightTerms(m.snippet, query) + '</div>' : '') +
        '</a>';
    });
    results.innerHTML = html;
    results.classList.add('active');
  }

  function close() {
    results.classList.remove('active');
    selectedIdx = -1;
  }

  var debounceTimer;
  input.addEventListener('input', function () {
    clearTimeout(debounceTimer);
    var q = input.value.trim();
    if (!q) { close(); return; }
    debounceTimer = setTimeout(function () {
      loadIndex().then(function () {
        renderResults(search(q), q);
      });
    }, 150);
  });

  input.addEventListener('focus', function () {
    if (input.value.trim()) {
      loadIndex().then(function () {
        var q = input.value.trim();
        renderResults(search(q), q);
      });
    }
  });

  // Keyboard navigation
  input.addEventListener('keydown', function (e) {
    var items = results.querySelectorAll('.search-result-item');
    if (!items.length) return;

    if (e.key === 'ArrowDown') {
      e.preventDefault();
      selectedIdx = Math.min(selectedIdx + 1, items.length - 1);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      selectedIdx = Math.max(selectedIdx - 1, 0);
    } else if (e.key === 'Enter' && selectedIdx >= 0) {
      e.preventDefault();
      items[selectedIdx].click();
      return;
    } else if (e.key === 'Escape') {
      close();
      input.blur();
      return;
    } else {
      return;
    }

    items.forEach(function (el, i) {
      el.classList.toggle('selected', i === selectedIdx);
    });
  });

  // Close on outside click
  document.addEventListener('click', function (e) {
    if (!e.target.closest('.search-wrapper')) close();
  });

  // Ctrl+K / "/" shortcut to focus search
  document.addEventListener('keydown', function (e) {
    if ((e.ctrlKey && e.key === 'k') || (e.key === '/' && !isInputFocused())) {
      e.preventDefault();
      input.focus();
      input.select();
    }
  });

  function isInputFocused() {
    var tag = document.activeElement && document.activeElement.tagName;
    return tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT';
  }
})();
