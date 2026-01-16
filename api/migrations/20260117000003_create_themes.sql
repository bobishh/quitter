CREATE TABLE themes (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    css TEXT NOT NULL,
    icon_limit INTEGER
);

ALTER TABLE habits ADD COLUMN theme_id UUID REFERENCES themes(id);

-- Seed default theme (Winamp Classic)
INSERT INTO themes (id, name, css, icon_limit) VALUES (
    '00000000-0000-0000-0000-000000000000',
    'Winamp Classic',
    'body {
    font-family: "Courier New", Courier, monospace;
    background-color: #1a1a1a;
    color: #0f0;
    margin: 0;
    padding: 20px;
}

/* Scrollbar */
::-webkit-scrollbar {
    width: 10px;
    background: #000;
}
::-webkit-scrollbar-thumb {
    background: #0f0;
    border: 1px solid #000;
}

.app-container {
    width: 95%;
    margin: 0 auto;
    border: 2px solid #555;
    background-color: #000;
    box-shadow: 5px 5px 0px #333;
    padding: 10px;
}

header {
    border-bottom: 2px dashed #333;
    padding-bottom: 15px;
    margin-bottom: 20px;
    text-align: center;
}

h1 {
    font-size: 1.2rem;
    margin: 0 0 15px 0;
    text-transform: uppercase;
    letter-spacing: 2px;
}

.main-nav {
    display: flex;
    justify-content: center;
    gap: 10px;
}

/* Winamp Buttons */
.winamp-btn, .winamp-btn-small {
    background-color: #000;
    color: #0f0;
    border: 1px solid #0f0;
    font-family: inherit;
    font-size: 0.8rem;
    padding: 5px 10px;
    cursor: pointer;
    text-transform: uppercase;
    box-shadow: 2px 2px 0px #0f0;
    transition: all 0.1s;
}

.winamp-btn.active {
    background-color: #0f0;
    color: #000;
    box-shadow: inset 2px 2px 0px #000;
}

.winamp-btn:active, .winamp-btn-small:active {
    transform: translate(2px, 2px);
    box-shadow: 0px 0px 0px;
}

.winamp-btn-small {
    padding: 2px 5px;
    font-size: 0.7rem;
    border-color: #f00;
    color: #f00;
    box-shadow: 2px 2px 0px #f00;
}

/* Habit Card (Tracker) */
.habit-card {
    border: 2px solid #0f0; /* Default, overridden by inline style */
    margin-bottom: 15px;
    padding: 10px;
    background: #111;
}

.habit-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid #333;
    padding-bottom: 5px;
    margin-bottom: 10px;
}

.habit-icon {
    font-size: 1.5rem;
    margin-right: 10px;
}

.habit-name {
    font-weight: bold;
    flex-grow: 1;
}

.habit-stats {
    font-size: 1.2rem;
    margin-bottom: 10px;
    text-align: center;
}

/* Habit Library Item */
.habit-item {
    border: 1px solid #333;
    padding: 10px;
    margin-bottom: 5px;
    background: #111;
    display: flex;
    align-items: center;
    transition: background-color 0.2s;
}
.habit-item:hover {
    background: #222;
}
.tracker-item {
    cursor: pointer;
}
.habit-item .icon { font-size: 1.5rem; margin-right: 10px; }
.habit-item .name { font-weight: bold; flex-grow: 1; }
.habit-item .details { font-size: 0.8rem; color: #777; }

.library-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 15px;
}

.empty-state {
    text-align: center;
    padding: 20px;
    color: #555;
    font-style: italic;
    border: 1px dashed #333;
}

.hint {
    color: #555;
    font-size: 0.8rem;
    margin-top: 5px;
}

/* Visualizer */
.visualizer-container {
    margin-top: 10px;
    border-top: 1px dotted #333;
    padding-top: 10px;
    background: #050505;
}

.visualizer-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 2px;
}

.viz-item {
    font-size: 1.2rem;
    line-height: 1;
}

.viz-overflow {
    display: block;
    width: 100%;
    text-align: center;
    font-style: italic;
    color: #555;
}

/* Forms */
.creator-form {
    border: 1px solid #555;
    padding: 15px;
    background: #0d0d0d;
}

.form-group {
    margin-bottom: 10px;
}

.form-group label {
    display: block;
    font-size: 0.8rem;
    color: #777;
    margin-bottom: 3px;
}

.form-group input, .form-group select {
    width: 100%;
    background: #000;
    border: 1px solid #333;
    color: #fff;
    padding: 5px;
    font-family: inherit;
    font-size: 1rem;
}

.actions {
    margin-top: 20px;
    display: flex;
    justify-content: flex-end;
    gap: 10px;
}

.info {
    font-size: 0.8rem;
    color: #aaa;
    font-style: italic;
    padding: 5px;
    border: 1px dotted #333;
}

.habit-item.error {
    border-color: red;
    color: red;
}

.habit-item.loading {
    /* Loading style */
}

/* Theme Manager */
.theme-form-section {
    border: 1px dashed #333;
    padding: 10px;
    margin-bottom: 20px;
}

.theme-list {
    display: flex;
    flex-direction: column;
    gap: 15px;
}

.theme-item {
    border: 1px solid #333;
    padding: 10px;
    background: #111;
}

.theme-item-header {
    font-weight: bold;
    border-bottom: 1px solid #222;
    margin-bottom: 5px;
    padding-bottom: 5px;
    color: #0f0;
}

.theme-item-css {
    margin: 0;
    font-size: 0.8rem;
    background: #000;
    padding: 5px;
    overflow-x: auto;
    color: #777;
}

/* Habit Detail View */
.habit-info-container {
    padding: 10px;
}

.habit-detail-actions {
    margin-top: 20px;
    border-top: 1px dashed #333;
    padding-top: 10px;
}

.tracker-card-actions {
    margin-top: 20px;
    border-top: 1px dashed #333;
    padding-top: 10px;
    display: flex;
    justify-content: space-between;
}

/* Theme Manager Textarea */
.theme-css-textarea {
    width: 100%;
    height: 100px;
    background: #000;
    color: #fff;
    border: 1px solid #333;
    font-family: monospace;
}',
    NULL
);

INSERT INTO themes (id, name, css, icon_limit) VALUES (
    '11111111-1111-1111-1111-111111111111',
    'Hyper Roam',
    'body { background: #000; color: #fff; overflow: hidden; margin: 0; }
.app-container { width: 100vw; height: 100vh; border: none; background: transparent; display: flex; flex-direction: column; align-items: center; justify-content: center; }
header, footer { display: none; }
.habit-card.full-view { border: none; background: transparent; box-shadow: none; width: 90vw; max-width: 800px; text-align: center; }
.habit-header { border-bottom: 1px solid #333; font-size: 1.5rem; justify-content: center; color: #fff; }
.habit-stats { font-size: 2rem; color: #0f0; margin: 20px 0; text-shadow: 0 0 10px #0f0; }
.visualizer-container { position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; z-index: -1; pointer-events: none; }
.visualizer-grid { display: block; width: 100%; height: 100%; }
.viz-item { position: absolute; font-size: 3rem; animation: roam 15s infinite alternate ease-in-out; filter: drop-shadow(0 0 5px rgba(255,255,255,0.5)); }
@keyframes roam {
  0% { transform: translate(0, 0) rotate(0deg); }
  33% { transform: translate(var(--tx), var(--ty)) rotate(var(--tr)); }
  66% { transform: translate(calc(var(--tx) * -1.2), calc(var(--ty) * 0.8)) rotate(calc(var(--tr) * -0.5)); }
  100% { transform: translate(calc(var(--tx) * 0.5), calc(var(--ty) * -1.5)) rotate(calc(var(--tr) * 2)); }
}
.viz-item:nth-child(2n) { animation-duration: 20s; }
.viz-item:nth-child(3n) { animation-duration: 25s; }
.viz-item:nth-child(5n) { animation-duration: 30s; }',
    500
);
