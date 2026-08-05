package com.myriad.wdl.model.sections;

import com.myriad.wdl.model.definitions.WdlTask.WdlTaskElement;
import com.myriad.wdl.model.expressions.WdlStringLiteral;
import lombok.Getter;
import lombok.Setter;

/**
 * The command template is evaluated after all of the inputs are staged and before the outputs are
 * evaluated. The command template is evaluated similarly to multi-line strings:
 *
 * <p>Remove all whitespace following the opening &lt;&lt;&lt;, up to and including a newline (if any).
 * Remove all whitespace preceeding the closing &gt;&gt;&gt;, up to and including a newline (if any). Use all
 * remaining non-blank lines to determine the common leading whitespace. Remove common leading
 * whitespace from each line. Evaluate placeholder expressions. Notice that there is one major
 * difference between the evaluation of multi-line strings vs the command template: line
 * continuations are removed in the former but left as-is in the latter. This also means that
 * continued lines are considered when determining common leading whitespace, and that common
 * leading whitespace is removed from continued lines as well.
 */
public final class WdlCommand implements WdlTaskElement {
  @Getter @Setter private WdlStringLiteral commandText;
  @Getter @Setter private boolean multiline;

  public WdlCommand() {}

  public WdlCommand(WdlStringLiteral commandText, boolean multiline) {
    setCommandText(commandText);
    this.multiline = multiline;
  }
}
