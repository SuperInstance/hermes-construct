# Analytics Integration

`analytics.js` is injected just before `</body>` in both `index.html` and `tags.html`.

```javascript
document.addEventListener('DOMContentLoaded', () => {
  // Page view event
  if (window.plausible) {
    window.plausible('gallery_view', {props: {path: window.location.pathname}});
  }

  // Feedback form submission
  const form = document.querySelector('form[action*="formspree"]');
  if (form) {
    form.addEventListener('submit', () => {
      if (window.plausible) {
        window.plausible('feedback_submitted');
      }
    });
  }

  // Tag index page view
  if (window.location.pathname === '/tags.html') {
    if (window.plausible) {
      window.plausible('tag_index_view');
    }
  }
});
```

- Fires Plausible events: `gallery_view`, `feedback_submitted`, `tag_index_view`.  
- No impact on page load; events are sent asynchronously.  
- Ensure Plausible script is present in the `<head>` (see `index.html` for the snippet).