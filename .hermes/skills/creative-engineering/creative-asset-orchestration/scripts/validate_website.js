// validate_website.js – simple sanity checks for the static gallery
document.addEventListener('DOMContentLoaded', function () {
  // Check that all images have an 'alt' attribute
  const images = document.querySelectorAll('img');
  images.forEach(img => {
    if (!img.hasAttribute('alt') || img.getAttribute('alt') === '') {
      console.warn('Missing or empty alt attribute for:', img.src);
    }
  });

  // Verify all internal links stay within the site
  const links = document.querySelectorAll('a[href]');
  const base = location.origin;
  links.forEach(link => {
    const href = link.getAttribute('href');
    const full = new URL(href, base);
    if (!full.origin === base) {
      console.warn('External link detected:', href);
    }
  });

  // Ensure no broken image src
  images.forEach(img => {
    if (img.getAttribute('src').endsWith('.png') && !img.complete) {
      img.onload_error = () => console.error('Failed to load image:', img.src);
    }
  });

  // Log any 404 fetch errors (images that failed to load)
  const observer = new MutationObserver((mutations) => {
    mutations.forEach(mutation => {
      mutation.addedNodes.forEach(node => {
        if (node.nodeType === Node.ELEMENT_NODE && node.tagName === 'IMG') {
          if (node.complete === false) {
            console.warn('Image still loading or failed:', node.src);
          }
        }
      });
    });
  });
  observer.observe(document.body, { childList: true, subtree: true });
});