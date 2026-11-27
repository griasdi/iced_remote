"""The README hero: the window of an annotated screenshot next to a
terminal with the ctl batch that wrote it and the replies, verbatim.

usage: hero-compose.py SOCKET BATCH REPLIES SHOT.png OUT.png
"""

import sys

from PIL import Image, ImageDraw, ImageFilter, ImageFont

sock, batch_path, replies_path, shot_path, out_path = sys.argv[1:]
batch = open(batch_path).read().splitlines()
replies = open(replies_path).read().splitlines()
shot = Image.open(shot_path).convert("RGBA")

MONO = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"
SANS = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"
font = ImageFont.truetype(MONO, 25)
title_font = ImageFont.truetype(SANS, 22)

BG = (22, 22, 30, 255)
BAR = (40, 41, 54, 255)
FG = (205, 208, 220, 255)
MUTED = (125, 130, 155, 255)
PROMPT = (122, 162, 247, 255)
OK = (158, 206, 106, 255)
NODE = (125, 207, 255, 255)

# Terminal lines: (text, colour) runs per line.
lines = [[("$ ", PROMPT), (f"hero --headless --control {sock} --scale 2 &", FG)]]
lines.append([("$ ", PROMPT), (f"iced-remote ctl {sock} - <<'EOF'", FG)])
lines += [[("  ", FG), (c, FG)] for c in batch]
lines.append([("  EOF", FG)])
max_nodes = 6
shown = 0
for reply in replies:
    if reply.startswith("ok") or reply.startswith("err"):
        word, _, rest = reply.partition(" ")
        lines.append([(word, OK if word == "ok" else (247, 118, 142, 255)), (" " + rest, MUTED)])
    elif shown < max_nodes:
        index, _, rest = reply.partition(" ")
        lines.append([(index.rjust(2), NODE), (" " + rest, FG)])
        shown += 1
    elif shown == max_nodes:
        lines.append([(" …", MUTED)])
        shown += 1

LINE_H = 36
PAD = 28
BAR_H = 52
char_w = font.getlength("M")
cols = max(sum(len(t) for t, _ in line) for line in lines)
term_w = int(PAD * 2 + char_w * cols)
term_h = BAR_H + PAD * 2 + LINE_H * len(lines)


def window(content_w, content_h, title, fill):
    img = Image.new("RGBA", (content_w, content_h + BAR_H), fill)
    d = ImageDraw.Draw(img)
    d.rectangle((0, 0, content_w, BAR_H), fill=BAR)
    for i, colour in enumerate([(247, 118, 142), (224, 175, 104), (158, 206, 106)]):
        cx = 26 + i * 28
        d.ellipse((cx - 8, BAR_H / 2 - 8, cx + 8, BAR_H / 2 + 8), fill=colour)
    tw = d.textlength(title, font=title_font)
    d.text(((content_w - tw) / 2, BAR_H / 2), title, font=title_font, fill=MUTED, anchor="lm")
    mask = Image.new("L", img.size, 0)
    ImageDraw.Draw(mask).rounded_rectangle((0, 0, *img.size), radius=14, fill=255)
    img.putalpha(mask)
    return img


term = window(term_w, term_h - BAR_H, "agent", BG)
d = ImageDraw.Draw(term)
y = BAR_H + PAD
for line in lines:
    x = PAD
    for text, colour in line:
        d.text((x, y), text, font=font, fill=colour)
        x += char_w * len(text)
    y += LINE_H

app = window(shot.width, shot.height, "Tasks  (headless, scale 2)", BG)
app.paste(shot, (0, BAR_H), shot)
mask = Image.new("L", app.size, 0)
ImageDraw.Draw(mask).rounded_rectangle((0, 0, *app.size), radius=14, fill=255)
app.putalpha(mask)

# App window left, terminal right and lower, no overlap: both stay readable.
M = 60
app_x, app_y = M, M
term_x = app_x + app.width + 48
term_y = app_y + 120
W = term_x + term_w + M
H = max(app_y + app.height, term_y + term_h) + M
canvas = Image.new("RGBA", (W, H), (0, 0, 0, 0))


def shadowed(img, x, y):
    shadow = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    alpha = img.getchannel("A").point(lambda a: a * 0.55)
    shadow.paste((0, 0, 0, 255), (x, y + 16), alpha)
    canvas.alpha_composite(shadow.filter(ImageFilter.GaussianBlur(28)))
    canvas.alpha_composite(img, (x, y))


shadowed(app, app_x, app_y)
shadowed(term, term_x, term_y)
canvas.save(out_path, optimize=True)
