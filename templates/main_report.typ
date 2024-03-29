#set text(font: "Noto Sans")
#set page(
    header: [
        #set align(right)
        #set text(8pt)
        Penetration Test Report Logo
    ],
    footer: [
        #set text(8pt)

        #place(
            left,
            text("Page No. " + counter(page).display("1 of 1", both: true))
        )

        #place(
            center,
            text("Client Confidential")
        )

        #place(
            right,
            text("{{ company_website }}")
        )
    ]
)

#block(height: 100pt)

#set align(right)
#set text(16pt)

#text(size: 24pt, weight: 900)[{{ report_title }}] \
{{ current_date }} \
#text(fill: blue)[Prepared for: ]{{ prepared_for }}

#set align(left)
#set text(14pt)

#block(height: 100pt)

#text(fill: blue, weight: 600, size: 20pt)[Prepared by:] \
{{ prepared_by }} \
{{ company_website }} \
#text(fill: blue)[E-mail: ]{{ company_email }} \
#text(fill: blue)[Phone: ]{{ company_phone }} \

#set align(left)

#pagebreak()
#outline(title: text(fill: blue)[Table of Contents])

{{ sections }}

#pagebreak()
= Findings

{{ findings }}

#pagebreak()
#set align(center)
= TODO: LAST PAGE CHANGE ME
