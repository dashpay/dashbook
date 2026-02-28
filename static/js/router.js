export class Router {
    constructor() {
        this.routes = [];
        this.currentCleanup = null;
        window.addEventListener('hashchange', () => this.resolve());
    }

    on(pattern, handler) {
        const paramNames = [];
        const regex = new RegExp(
            '^' + pattern.replace(/:(\w+)/g, (_, name) => {
                paramNames.push(name);
                return '([^/]+)';
            }) + '$'
        );
        this.routes.push({ regex, paramNames, handler });
        return this;
    }

    async resolve() {
        const hash = location.hash.slice(1) || '/';
        if (this.currentCleanup) {
            this.currentCleanup();
            this.currentCleanup = null;
        }

        // Update active nav link
        document.querySelectorAll('.nav-link').forEach(link => {
            const href = link.getAttribute('href').slice(1);
            link.classList.toggle('active',
                href === '/' ? hash === '/' : hash.startsWith(href)
            );
        });

        for (const route of this.routes) {
            const match = hash.match(route.regex);
            if (match) {
                const params = {};
                route.paramNames.forEach((name, i) => params[name] = decodeURIComponent(match[i + 1]));
                try {
                    const cleanup = await route.handler(params);
                    if (typeof cleanup === 'function') this.currentCleanup = cleanup;
                } catch (e) {
                    console.error('Route error:', e);
                    const p = document.createElement('p');
                    p.textContent = e.message;
                    const app = document.getElementById('app');
                    app.innerHTML = '<div class="error-message"><h2>Error</h2></div>';
                    app.querySelector('.error-message').appendChild(p);
                }
                return;
            }
        }

        document.getElementById('app').innerHTML = `
            <div class="error-message">
                <h2>Page Not Found</h2>
                <p>The page you're looking for doesn't exist.</p>
            </div>`;
    }

    start() { this.resolve(); }
}
