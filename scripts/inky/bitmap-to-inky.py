from PIL import Image
from inky.auto import auto

import argparse

parser = argparse.ArgumentParser(description="Display a bitmap image on an Inky display.")
parser.add_argument("bitmap_path", help="Path to the bitmap image")
args = parser.parse_args()

# Auto-detect Inky display
inky_display = auto()

image = Image.open(bitmap_path)

# Resize to this Inky pHAT dimensions
image = image.resize((inky_display.WIDTH, inky_display.HEIGHT))

# Convert to display palette
image = image.convert("P")

# Display the image
inky_display.set_image(image)
inky_display.show()