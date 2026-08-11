package org.openwdl.wdl.model.v1;

import org.junit.platform.suite.api.SelectClasses;
import org.junit.platform.suite.api.Suite;

@Suite
@SelectClasses({
  WdlV11SpecExamplesTest.class,
  WdlV12SpecExamplesTest.class,
  WdlV13SpecExamplesTest.class
})
public class WdlSpecExamplesSuite {}
