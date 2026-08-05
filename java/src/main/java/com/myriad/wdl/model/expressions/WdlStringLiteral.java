package com.myriad.wdl.model.expressions;

import com.myriad.wdl.model.base.WdlNode;
import java.util.ArrayDeque;
import lombok.Getter;
import lombok.Setter;

public final class WdlStringLiteral implements WdlExpression {
  public static enum Delimiter {
    SINGLE_QUOTE,
    DOUBLE_QUOTE,
    MULTILINE,
  }

  @Getter @Setter private Delimiter delimiter;
  private final ArrayDeque<WdlStringComponent> components = new ArrayDeque<>();

  public WdlStringLiteral() {}

  public WdlStringLiteral(Delimiter delimiter) {
    this.delimiter = delimiter;
  }

  public ArrayDeque<WdlStringComponent> components() {
    return components;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.STR_LIT;
  }

  public abstract static class WdlStringComponent implements WdlNode {
    public static enum ComponentType {
      TEXT,
      ESC,
      PLACEHOLDER,
      SPECIAL
    }

    public abstract WdlStringComponent.ComponentType componentType();
  }

  public static final class WdlStringText extends WdlStringComponent {
    @Getter @Setter private String text;

    public WdlStringText() {}

    public WdlStringText(String text) {
      setText(text);
    }

    @Override
    public ComponentType componentType() {
      return ComponentType.TEXT;
    }
  }

  public static final class WdlStringEscape extends WdlStringComponent {
    @Getter @Setter private String escapeText;

    public WdlStringEscape() {}

    public WdlStringEscape(String escapeText) {
      setEscapeText(escapeText);
    }

    @Override
    public ComponentType componentType() {
      return ComponentType.ESC;
    }
  }

  public static final class WdlStringToken extends WdlStringComponent {
    @Getter @Setter private String tokenText;

    public WdlStringToken() {}

    public WdlStringToken(String tokenText) {
      setTokenText(tokenText);
    }

    @Override
    public ComponentType componentType() {
      return ComponentType.SPECIAL;
    }
  }

  public static final class WdlStringPlaceholder extends WdlStringComponent {
    public static enum PlaceHolderSymbol {
      TILDE("~"),
      DOLLAR("$");
      @Getter private final String wdlString;

      private PlaceHolderSymbol(String wdlString) {
        this.wdlString = wdlString;
      }
    }

    @Getter @Setter private WdlStringPlaceholderOption option;
    @Getter @Setter private WdlExpression expression;
    @Getter @Setter private PlaceHolderSymbol symbol;

    public WdlStringPlaceholder() {}

    public WdlStringPlaceholder(WdlStringPlaceholderOption option, WdlExpression expression) {
      this.option = option;
      this.expression = expression;
    }

    @Override
    public ComponentType componentType() {
      return ComponentType.PLACEHOLDER;
    }
  }

  public static final class WdlStringPlaceholderOption implements WdlNode {
    public static enum Type {
      SEP,
      DEFAULT,
      TRUE_FALSE,
      FALSE_TRUE
    }

    @Getter private final Type type;
    private WdlStringLiteral value;
    private WdlStringLiteral trueValue;
    private WdlStringLiteral falseValue;

    public WdlStringPlaceholderOption(Type type) {
      this.type = type;
    }

    public WdlStringPlaceholderOption(Type type, WdlStringLiteral value) {
      this(type);
      if (type != Type.SEP && type != Type.DEFAULT) {
        throw new IllegalAccessError("Illegal access for " + type);
      }
      this.value = value;
    }

    public WdlStringPlaceholderOption(
        Type type, WdlStringLiteral trueValue, WdlStringLiteral falseValue) {
      this(type);
      if (type != Type.TRUE_FALSE && type != Type.FALSE_TRUE) {
        throw new IllegalAccessError("Illegal access for " + type);
      }
      this.trueValue = trueValue;
      this.falseValue = falseValue;
    }

    public WdlStringLiteral getValue() {
      if (type != Type.SEP && type != Type.DEFAULT) {
        throw new IllegalAccessError("Illegal access for " + type);
      }
      return value;
    }

    public void setValue(WdlStringLiteral value) {
      if (type != Type.SEP && type != Type.DEFAULT) {
        throw new IllegalAccessError("Illegal access for " + type);
      }
      this.value = value;
    }

    public WdlStringLiteral getTrueValue() {
      if (type != Type.TRUE_FALSE && type != Type.FALSE_TRUE) {
        throw new IllegalAccessError("Illegal access for " + type);
      }
      return trueValue;
    }

    public void setTrueValue(WdlStringLiteral value) {
      if (type != Type.TRUE_FALSE && type != Type.FALSE_TRUE) {
        throw new IllegalAccessError("Illegal access for " + type);
      }
      this.trueValue = value;
    }

    public WdlStringLiteral getFalseValue() {
      if (type != Type.TRUE_FALSE && type != Type.FALSE_TRUE) {
        throw new IllegalAccessError("Illegal access for " + type);
      }
      return falseValue;
    }

    public void setFalseValue(WdlStringLiteral value) {
      if (type != Type.TRUE_FALSE && type != Type.FALSE_TRUE) {
        throw new IllegalAccessError("Illegal access for " + type);
      }
      this.falseValue = value;
    }

    @Override
    public String toString() {
      return getClass().getSimpleName();
    }
  }
}
