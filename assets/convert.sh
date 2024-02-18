for file in *.svg; do
    filename=$(basename -- "$file")
    filename="${filename%.*}"
    inkscape --export-width=130 --export-type=png --export-filename="${filename}.png" "${filename}.svg"
done