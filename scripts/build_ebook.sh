# Comment out header and footer
#   <link rel="stylesheet" href="./css/main.css">

# Then
find _site/ -exec sed -i 's|"/img|"../img|g' {} \;

for filename in _site/*; do
	html2pdf $filename files/pdf/$(basename "$filename" .html).pdf;
done

rm files/pdf/feed.xml.pdf
rm files/pdf/LICENCE.md.pdf
mv files/pdf/tags.pdf files/pdf/1.pdf
mv files/pdf/index.pdf files/pdf/2.pdf

pdftk files/pdf/*.pdf cat output files/argmin_gravitas_temp.pdf

montage img/*a* -tile 10x10 montage.png

pdftk montage-0.pdf files/argmin_gravitas_temp.pdf cat output files/argmin_gravitas.pdf