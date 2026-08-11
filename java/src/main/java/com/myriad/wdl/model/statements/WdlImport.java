package com.myriad.wdl.model.statements;

import com.myriad.wdl.model.WdlDocument.WdlDocumentElement;
import com.myriad.wdl.model.base.WdlNode;
import com.myriad.wdl.model.base.WdlSourceRange;
import com.myriad.wdl.model.expressions.WdlStringLiteral;
import java.util.ArrayDeque;
import lombok.Getter;
import lombok.Setter;

/**
 * Import of another WDL document by URI-like source string.
 *
 * <p>Three forms are recognized.
 *
 * <p>1. `import <i>source</i> [as <i>alias</i>] (alias <i>Old</i> as <i>New</i>)*`
 * — the existing import form.
 * `<i>source</i>` is either a quoted URI or a symbolic module path. `as <i>alias</i>` renames the
 * pseudo-namespace through which the imported tasks and workflows are accessed; `alias <i>Old</i> as
 * <i>New</i>` renames an imported struct or enum.
 *
 * 2. `import * from <i>source</i>`
 * — every task, workflow,
 * and user-defined type from `<i>source</i>` is brought into the importing document's scope.
 *
 * 3. `import { <i>member</i> [as <i>Name</i>], ... } from <i>source</i>`
 * — only the listed items are brought into scope. A
 * per-member `as <i>Name</i>` renames the selected item locally.
 *
 * <p>Forms 2 and 3 do not accept a trailing `as <i>alias</i>` or `alias` clause.
 */
public abstract class WdlImport implements WdlDocumentElement {

  @Getter @Setter protected WdlStringLiteral source;
  @Getter @Setter protected String sourceText;
  @Getter @Setter protected WdlSourceRange sourceRange;

  /** Resolver-normalized import identifier used as the lookup key in imported document maps. */
  @Getter @Setter protected String importIdentifier;

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  public static final class WdlImportStandard extends WdlImport {
    @Getter @Setter private String alias;
    private ArrayDeque<WdlImportMember> members = new ArrayDeque<>();

    public void setSource(WdlStringLiteral source) {
      this.source = source;
    }

    public ArrayDeque<WdlImportMember> members() {
      return members;
    }
  }

  public static final class WdlImportStar extends WdlImport {}

  public static final class WdlImportMembers extends WdlImport {
    private ArrayDeque<WdlImportMember> members = new ArrayDeque<>();

    public ArrayDeque<WdlImportMember> members() {
      return members;
    }

    public void setSource(WdlStringLiteral source) {
      this.source = source;
    }
  }

  public static final class WdlImportMember implements WdlNode {
    @Getter @Setter private String member;
    @Getter @Setter private String alias;

    public WdlImportMember() {}

    public WdlImportMember(String member, String alias) {
      setMember(member);
      setAlias(alias);
    }

    @Override
    public String toString() {
      return getClass().getSimpleName();
    }
  }
}
