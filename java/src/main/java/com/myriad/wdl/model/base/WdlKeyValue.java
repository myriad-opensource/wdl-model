package com.myriad.wdl.model.base;

import com.myriad.wdl.model.expressions.WdlExpression;
import lombok.Getter;
import lombok.Setter;

/**
 * Generic key-value node used throughout the WDL model.
 *
 * <p>Examples include metadata entries, runtime and requirements entries, map literal entries, and
 * call input bindings. The WDL specification uses key-value structures in sections such as
 * metadata, requirements, hints, and JSON-like literal forms.
 */
public abstract class WdlKeyValue<K, V> implements WdlNode {
  @Getter @Setter private K key;
  @Getter @Setter private V value;

  protected WdlKeyValue() {}

  protected WdlKeyValue(K key) {
    this();
    setKey(key);
  }

  protected WdlKeyValue(K key, V value) {
    this(key);
    setValue(value);
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  public abstract static class WdlStringKeyValue extends WdlKeyValue<String, WdlExpression> {
    protected WdlStringKeyValue() {
      super();
    }

    protected WdlStringKeyValue(String key, WdlExpression value) {
      super(key, value);
    }

    protected WdlStringKeyValue(String key) {
      super(key);
    }
  }

  /** Key-value node whose key and value are both expressions. */
  public abstract static class WdlExpresionKeyValue
      extends WdlKeyValue<WdlExpression, WdlExpression> {
    protected WdlExpresionKeyValue() {
      super();
    }

    protected WdlExpresionKeyValue(WdlExpression key, WdlExpression value) {
      super(key, value);
    }

    protected WdlExpresionKeyValue(WdlExpression key) {
      super(key);
    }
  }
}
