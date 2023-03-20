MathJax = {
  tex: {
    inlineMath: [['$', '$'], ["\\(", "\\)"]],
    processEscapes: true,
  }
}

function reRenderMath() {
    MathJax.typeset()
}
