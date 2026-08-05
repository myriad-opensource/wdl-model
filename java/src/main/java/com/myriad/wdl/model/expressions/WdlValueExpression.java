package com.myriad.wdl.model.expressions;

public interface WdlValueExpression<T> extends WdlExpression {
  T getValue();
}
