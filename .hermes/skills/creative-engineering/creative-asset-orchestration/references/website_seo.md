# SEO Basics for Static Art Galleries

## Meta Tags
```html
<meta name="description" content="Hermes Image Gallery showcases AI‑generated art from the Hermes fleet. Explore high‑resolution visuals, download PNGs, and learn about the creative process.">
<meta name="keywords" content="AI art, generative art, Hermes fleet, digital gallery, image generation, superinstance">
```

## Sitemap (sitemap.xml)
The `sitemap.xml` placed at the site root automatically informs crawlers of all page URLs:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url><loc>https://example.com/</loc></url>
  <url><loc>https://example.com/index.html</loc></url>
  <url><loc>https://example.com/image_00001.html</loc></url>
  <url><loc>https://example.com/image_00002.html</loc></url>
  <url><loc>https://example.com/image_00003.html</loc></url>
  <url><loc>https://example.com/image_00004.html</loc></url>
  <url><loc>https://example.com/about.html</loc></url>
  <url><loc>https://example.com/contact.html</loc></url>
</urlset>
```

## robots.txt
A minimal `robots.txt` helps crawlers index the public gallery while protecting private assets:
```
User-agent: *
Allow: /
Allow: /images/
Allow: /css/
Allow: /js/
Sitemap: https://example.com/sitemap.xml
```

## Canonical URLs
Add a `<link rel="canonical" href="https://example.com/">` tag on every page to avoid duplicate‑content issues.

## Open Graph & Social Sharing
For richer link previews on social platforms:
```html
<meta property="og:title" content="Hermes Image Gallery – AI‑Generated Art">
<meta property="og:description" content="Explore high‑resolution AI‑generated images from the Hermes fleet.">
<meta property="og:image" content="https://example.com/images/Hermes_Sub_00001.png">
<meta property="og:url" content="https://example.com/">
<meta property="og:type" content="website">
```

## Performance Tips
- Serve images in WebP format for smaller file sizes; convert existing PNGs with `cwebp`.
- Enable gzip or Brotli compression on CSS/JS files.
- Set long‑term caching headers (`Cache‑Control: max‑age=31536000`) for static assets.
- Use `loading="lazy"` on `<img>` tags to defer off‑screen loading.

## Accessibility Checklist
- Every `<img>` includes descriptive `alt` text.
- Color contrast meets WCAG AA standards (use tools like axe.dev to audit).
- All interactive elements are keyboard‑focusable.
- Provide a skip‑to‑content link for screen readers.

### Optional: Google Search Console
Submit the site’s URL to Google Search Console to monitor indexing, receive crawl errors, and view performance metrics (impressions, clicks, average position).

---  
*This guide assumes you are publishing the static site from `C:\Users\casey\website` or the Docker container built from it. Adjust URLs and paths accordingly for your hosting environment.*