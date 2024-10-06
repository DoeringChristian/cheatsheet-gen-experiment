
#set page(flipped: true, margin: 0.1cm)
#set heading(numbering: "1.1")
#show math.equation: set text(font: "New Computer Modern Math")

#show: rest => columns(4, rest, gutter: 1%)
  
{% for block in blocks %}
  #block(
    [{{block.content}}],
    stroke: 1pt + black,
  )
{% endfor %}
  
