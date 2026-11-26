---
trigger: always_on
---

# UI & Dark Mode Policy

- **Dark Mode Only**: This application uses dark mode exclusively. Do not add a light/dark theme toggle or light-mode color variants unless explicitly requested.
- **Tailwind CSS v4**: Define shared colors, typography, and spacing tokens using `@theme` in `frontend/src/routes/layout.css` rather than a `tailwind.config.js` file.
- **Contrast & Surfaces**: Use distinct dark surface layers (base background, elevated card/panel, subtle border) with accessible text contrast.
