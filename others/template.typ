#set align(center)
#set text(18pt)

#text(size: 24pt, weight: 900)[{{ report_title }}] \
{{ date }} \
Prepared for: {{ prepared_for }}

#set align(right)
#set text(14pt)

#block(height: 100pt)

#text(size: 20pt)[Prepared by:] \
{{ prepared_by }}

#set align(left)

#pagebreak()
#outline()

{{ sections }}

#pagebreak()
= Findings

{{ findings }}
