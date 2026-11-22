"""Generate koshr's brand assets (SVG sources and PNG renders) by the startr.trade family guide.

Run from this folder: python3 brand.py  (needs cairosvg and the Noto Sans fonts).
The family guide: one thread through the lockup, saffron once in the mark, the name in lower case
in Noto Sans Bold, the shared palette. Edit this script, not the SVGs it writes.
"""
import cairosvg

PRODUCT = "koshr"
GLOSS_WORD = "कोश"     # the source word, in its own script
GLOSS_REST = "kośa · a treasury, kept"
TAGLINE = ("A git server that stays git: verified identity,", "row-level access, every mutation signed.")          # two lines on the social card
SITE = "koshr.startr.trade"
FONT = "'Noto Sans','Segoe UI',ui-sans-serif,sans-serif"
GLOSS_FONT = "'Noto Sans Devanagari','Noto Sans',sans-serif"

PAL = {
    "light": dict(ink="#26317E", ground="#FFFFFF", saffron="#E8871E", grey="#8A8FA8", text="#26317E"),
    "dark": dict(ink="#AEB6E8", ground="#0F1117", saffron="#F49D1A", grey="#7C82A0", text="#EDEFF9"),
}

MARK_NOTE = ("A casket (कोश kośa, a treasury, a store): the code kept safe. The thread comes out of "
             "the keyhole to a saffron dot: the signed commit the store lets through.")
THREAD_Y = 80    # the keyhole ring, in the mark's own coordinates

def mark(c, thread=True):
    ink, ground, saffron = c["ink"], c["ground"], c["saffron"]
    body = f'''    <path d="M22 58 V48 C22 24 138 24 138 48 V58 Z" fill="{ground}" stroke="{ink}" stroke-width="5" stroke-linejoin="round"/>
    <rect x="22" y="58" width="116" height="56" rx="8" fill="{ground}" stroke="{ink}" stroke-width="5"/>
    <path d="M44 34 V114 M116 34 V114" fill="none" stroke="{ink}" stroke-width="4" stroke-linecap="round"/>'''
    if thread:
        body += f'''
    <path d="M80 88 V122 C80 132 86 138 96 138 H110" fill="none" stroke="{ink}" stroke-width="7" stroke-linecap="round"/>
    <circle cx="124" cy="138" r="13" fill="{saffron}"/>'''
    body += f'''
    <circle cx="80" cy="{THREAD_Y}" r="8" fill="{ground}" stroke="{ink}" stroke-width="4"/>'''
    return body

def favicon():
    return '''  <path class="ground line" d="M10 52 V42 C10 16 118 16 118 42 V52 Z" stroke-width="8" stroke-linejoin="round"/>
  <rect class="ink" x="6" y="52" width="116" height="50" rx="9"/>
  <path class="line" d="M64 74 V102 C64 110 70 114 78 114 H88" fill="none" stroke-width="12" stroke-linecap="round"/>
  <circle class="ground" cx="64" cy="72" r="9"/>
  <circle cx="104" cy="112" r="14" fill="#E8871E"/>'''


def mark_svg(c):
    return f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 160 160" role="img" aria-label="{PRODUCT} mark">
  <!-- {MARK_NOTE} -->
{mark(c, thread=True)}
</svg>
'''

def lockup_svg(c, variant):
    return f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 720 200" role="img" aria-label="{PRODUCT} logo{' (dark)' if variant == 'dark' else ''}">
  <!-- The mark, then the thread out of it runs down, along and up to become the headline bar the
       wordmark hangs from (the same thread that carries every startr.trade lockup), ending in a
       saffron arrow. -->
  <g transform="translate(14,20)">
{mark(c, thread=False)}
  </g>
  <path d="M94 {THREAD_Y + 20} V142 C94 152 100 158 110 158 H140 C170 158 164 52 196 52 H686" fill="none" stroke="{c['ink']}" stroke-width="5" stroke-linecap="round"/>
  <circle cx="94" cy="{THREAD_Y + 20}" r="7" fill="{c['ground']}" stroke="{c['ink']}" stroke-width="4"/>
  <path d="M686 44 L704 52 L686 60 Z" fill="{c['saffron']}"/>
  <text x="212" y="124" font-family="{FONT}"
        font-size="74" font-weight="700" letter-spacing="1" fill="{c['text']}">{PRODUCT}</text>
  <text x="216" y="166" font-family="{FONT}"
        font-size="26" fill="{c['grey']}"><tspan font-family="{GLOSS_FONT}">{GLOSS_WORD}</tspan> {GLOSS_REST}</text>
</svg>
'''

def favicon_svg():
    return f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128" role="img" aria-label="{PRODUCT}">
  <!-- Favicon cut of the mark: heavier strokes, the thread and the saffron dot, so it still reads
       at 16 px. Theme-aware via prefers-color-scheme. -->
  <style>
    .ink    {{ fill: #26317E; }}
    .line   {{ stroke: #26317E; }}
    .ground {{ fill: #FFFFFF; }}
    @media (prefers-color-scheme: dark) {{
      .ink    {{ fill: #AEB6E8; }}
      .line   {{ stroke: #AEB6E8; }}
      .ground {{ fill: #0F1117; }}
    }}
  </style>
{favicon()}
</svg>
'''

def app_icon_svg(c):
    return f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 180 180" role="img" aria-label="{PRODUCT}">
  <!-- Home-screen / app icon: the mark on a light tile (iOS adds its own rounding). -->
  <rect width="180" height="180" fill="#FFFFFF"/>
  <g transform="translate(10,6)">
{mark(c, thread=True)}
  </g>
</svg>
'''

def social_svg(c):
    return f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1280 640" role="img" aria-label="{PRODUCT}: {TAGLINE[0]} {TAGLINE[1]}">
  <!-- GitHub social-preview card (rendered to PNG at 1280x640 by brand.py). -->
  <rect width="1280" height="640" fill="#0F1117"/>
  <g transform="translate(90,130) scale(2.4)">
{mark(c, thread=True)}
  </g>
  <path d="M566 190 H1120" fill="none" stroke="{c['ink']}" stroke-width="6" stroke-linecap="round"/>
  <path d="M1120 180 L1144 190 L1120 200 Z" fill="{c['saffron']}"/>
  <text x="566" y="330" font-family="{FONT}"
        font-size="128" font-weight="700" letter-spacing="2" fill="{c['text']}">{PRODUCT}</text>
  <text x="572" y="392" font-family="{FONT}"
        font-size="34" fill="{c['grey']}"><tspan font-family="{GLOSS_FONT}">{GLOSS_WORD}</tspan> {GLOSS_REST.split(' · ')[0]}</text>
  <text x="570" y="470" font-family="{FONT}"
        font-size="27" fill="#C9CDE4">{TAGLINE[0]}</text>
  <text x="570" y="514" font-family="{FONT}"
        font-size="27" fill="#C9CDE4">{TAGLINE[1]}</text>
  <text x="570" y="586" font-family="{FONT}"
        font-size="30" letter-spacing="1" fill="#E8871E">{SITE}</text>
</svg>
'''

if __name__ == "__main__":
    L, D = PAL["light"], PAL["dark"]
    files = {
        f"{PRODUCT}-mark.svg": mark_svg(L),
        f"{PRODUCT}-lockup-light.svg": lockup_svg(L, "light"),
        f"{PRODUCT}-lockup-dark.svg": lockup_svg(D, "dark"),
        f"{PRODUCT}-favicon.svg": favicon_svg(),
        f"{PRODUCT}-app-icon.svg": app_icon_svg(L),
        f"{PRODUCT}-social-preview.svg": social_svg(D),
    }
    for name, body in files.items():
        open(name, "w", encoding="utf-8").write(body)
        print(name)
    cairosvg.svg2png(url=f"{PRODUCT}-app-icon.svg", write_to="apple-touch-icon.png", output_width=180)
    cairosvg.svg2png(url=f"{PRODUCT}-social-preview.svg", write_to=f"{PRODUCT}-social-preview.png", output_width=1280)
    print("apple-touch-icon.png", f"{PRODUCT}-social-preview.png")
