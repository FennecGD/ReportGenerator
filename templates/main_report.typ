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
            text("www.pentestcompany.com")
        )
    ]
)

#block(height: 100pt)

#set align(left)
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

#pagebreak()
#set align(center)
Generated using "Report Generator". Cool
