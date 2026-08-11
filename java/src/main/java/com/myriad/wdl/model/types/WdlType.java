package com.myriad.wdl.model.types;

import com.myriad.wdl.model.base.WdlNode;

/**
 * Base class for WDL types.
 *
 * <p>The WDL specification defines primitive types, arrays, maps, pairs, custom type references,
 * optionals, and related coercion rules. Concrete subclasses represent those categories while this
 * base class carries the shared optional marker used by the validators and processors.
 */
public abstract class WdlType implements WdlNode {
  /** High-level type family used for traversal and validation dispatch. */
  public static enum ComponentType {
    PRIMITIVE,
    TYPEREF,
    ARRAY,
    PAIR,
    MAP,
  }

  protected boolean optional;

  protected WdlType() {}

  protected WdlType(boolean optional) {
    this.optional = optional;
  }

  /** Returns the broad category of this type node. */
  public abstract ComponentType componentType();

  /** Returns whether this type is optional, as in {@code T?}. */
  public boolean isOptional() {
    return optional;
  }

  /** Sets whether this type is optional, as in {@code T?}. */
  public void setOptional(boolean optional) {
    this.optional = optional;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }
}
