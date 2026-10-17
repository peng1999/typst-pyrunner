#import "@preview/tidy:0.4.3"
#import "../pkg/pyrunner.typ" as py

#set document(title: "pyrunner Reference")

#let ink = rgb("202d3a")
#let muted = rgb("64748b")
#let accent = rgb("236b73")
#let rule = rgb("dce3e8")

#set page(
  paper: "a4",
  margin: (x: 20mm, top: 18mm, bottom: 18mm),
  footer: context {
    set text(size: 8pt, fill: muted)
    line(length: 100%, stroke: 0.5pt + rule)
    v(5pt)
    [pyrunner / API reference #h(1fr) #counter(page).display()]
  },
)
#set text(font: "DejaVu Sans", size: 9pt, fill: ink)
#set par(leading: 0.65em, spacing: 7pt)
#set heading(numbering: none)
#show heading.where(level: 2): set text(size: 14pt, fill: accent)
#show heading.where(level: 3): set text(size: 9pt, fill: muted)
// Compensate for raw's built-in 0.8em scaling so code matches surrounding text.
#show raw: set text(font: "DejaVu Sans Mono", size: 1.25em)
#show link: set text(fill: accent)

#let docs = tidy.parse-module(read("../pkg/pyrunner.typ"), scope: (py: py))
#let package = toml("../pkg/typst.toml").package

// Stack examples vertically to keep longer code and results readable.
#let example-layout(code, preview) = {
  block(
    width: 100%, fill: rgb("f2f6f7"), radius: 3pt,
    inset: 10pt, breakable: true, sticky: true,
    code,
  )
  block(width: 100%, inset: (x: 10pt, y: 8pt), stroke: (left: 2pt + accent))[
    #text(size: 8pt, weight: "bold", fill: muted)[RESULT]
    #v(5pt)
    #preview
  ]
}

#let show-example(..args) = {
  set text(size: 9pt)
  tidy.show-example.show-example(..args, layout: example-layout)
}

// Allow long API sections to flow naturally across pages.
#let show-function(fn, style-args) = block(
  width: 100%,
  breakable: true,
  above: 16pt,
  below: 0pt,
  {
    [#heading(level: 2, raw(fn.name + "()"))#label(style-args.label-prefix + fn.name + "()")]

    let args = fn.args.pairs().map(((name, info)) => {
      let default = if "default" in info { " = " + info.default } else { "" }
      name + default
    })
    let returns = fn.at("return-types", default: ())
    let signature = fn.name + "(" + args.join(", ") + ")"
    if returns != none and returns.len() > 0 {
      signature += " → " + returns.join(" | ")
    }
    block(
      width: 100%, fill: rgb("f2f6f7"), radius: 3pt,
      inset: (x: 10pt, y: 8pt), above: 7pt, below: 8pt,
      raw(signature),
    )
    tidy.utilities.eval-docstring(fn.description, style-args)

    let cells = ()
    for (name, info) in fn.args {
      let parameter = [
        #raw(name)
        #label(style-args.label-prefix + fn.name + "." + name.trim("."))
      ]
      let description = tidy.utilities.eval-docstring(
        info.at("description", default: ""), style-args,
      )
      if "default" in info {
        description += [ #text(fill: muted)[Default: #raw(info.default).]]
      }
      cells += (
        parameter,
        text(fill: muted, raw(info.at("types", default: ()).join(" | "))),
        description,
      )
    }
    table(
      columns: (19%, 22%, 59%),
      stroke: (top: none, left: none, right: none, bottom: 0.5pt + rule),
      inset: (x: 6pt, y: 7pt),
      align: left + top,
      table.header(
        text(size: 8pt, weight: "bold", fill: muted, "Parameter"),
        text(size: 8pt, weight: "bold", fill: muted, "Type"),
        text(size: 8pt, weight: "bold", fill: muted, "Description"),
      ),
      ..cells,
    )
  },
)

#text(size: 8pt, weight: "bold", fill: accent, tracking: 1pt)[TYPST PACKAGE / API REFERENCE]
#v(16pt)
#text(size: 30pt, weight: "bold")[pyrunner]
#h(1fr)
#text(size: 9pt, fill: muted)[v#package.version]
#v(14pt)
Run Python code inside Typst. Execute a block directly, or compile once and call
functions from the resulting bytecode.

Examples use #raw("#import \"@preview/pyrunner:" + package.version + "\" as py", lang: "typ").
#v(14pt)
#block(width: 100%, inset: (y: 12pt), below: 12pt, stroke: (top: 0.5pt + rule, bottom: 0.5pt + rule))[
  #text(size: 8pt, weight: "bold", fill: muted)[FUNCTIONS]
  #h(14pt)
  #docs.functions.map(fn => link(label(docs.label-prefix + fn.name + "()"), raw(fn.name + "()"))).join(h(16pt))
]

#tidy.show-module(
  docs,
  style: tidy.utilities.get-style-functions(tidy.styles.default) + (
    show-function: show-function,
    show-example: show-example,
  ),
  show-module-name: false,
  show-outline: false,
  first-heading-level: 1,
  sort-functions: none,
  omit-private-definitions: true,
)
