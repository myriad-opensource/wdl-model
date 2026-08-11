import { describe, expect, it } from 'vitest';

import {
  inferEnumValueType,
  WdlEnum,
  WdlEnumChoice,
  WdlFloatLiteral,
  WdlFunction,
  WdlFunctionCallOperation,
  WdlIntLiteral,
  WdlPrimitiveType,
  WdlStruct,
  WdlStructMember,
} from '../src/index.js';

describe('TypeScript type inference helpers', () => {
  it('infers implicit enum type as String', () => {
    const en = new WdlEnum('Letters');
    en.elements().push(new WdlEnumChoice('A'));
    en.elements().push(new WdlEnumChoice('B'));

    const inferred = inferEnumValueType(en);

    expect(inferred).toBeInstanceOf(WdlPrimitiveType);
    expect((inferred as WdlPrimitiveType).primitiveType()).toBe(WdlPrimitiveType.Type.STRING);
  });

  it('widens Int and Float enum choices to Float', () => {
    const en = new WdlEnum('Numbers');
    en.elements().push(new WdlEnumChoice('ONE', new WdlIntLiteral(1)));
    en.elements().push(new WdlEnumChoice('PI', new WdlFloatLiteral(3.14)));

    const inferred = inferEnumValueType(en);

    expect(inferred).toBeInstanceOf(WdlPrimitiveType);
    expect((inferred as WdlPrimitiveType).primitiveType()).toBe(WdlPrimitiveType.Type.FLOAT);
  });

  it('returns undefined for non-literal enum value expressions', () => {
    const dynamic = new WdlFunctionCallOperation();
    dynamic.setFunctionName('foo');
    dynamic.setFunction(WdlFunction.NONSTANDARD);

    const en = new WdlEnum('Bad');
    en.elements().push(new WdlEnumChoice('ONE', new WdlIntLiteral(1)));
    en.elements().push(new WdlEnumChoice('DYNAMIC', dynamic));

    expect(inferEnumValueType(en)).toBeUndefined();
  });

  it('supports local struct and enum introspection helpers', () => {
    const struct = new WdlStruct('Person');
    struct.elements().push(
      new WdlStructMember(new WdlPrimitiveType(WdlPrimitiveType.Type.STRING), 'name'),
    );
    struct.elements().push(
      new WdlStructMember(new WdlPrimitiveType(WdlPrimitiveType.Type.INT), 'age'),
    );

    const en = new WdlEnum('Status');
    en.elements().push(new WdlEnumChoice('NEW'));
    en.elements().push(new WdlEnumChoice('DONE'));

    expect(struct.hasMember('name')).toBe(true);
    expect(struct.hasMember('missing')).toBe(false);
    expect(struct.member('name')).toBeDefined();
    expect((struct.memberType('age') as WdlPrimitiveType).primitiveType()).toBe(
      WdlPrimitiveType.Type.INT,
    );

    expect(en.hasChoice('DONE')).toBe(true);
    expect(en.hasChoice('FAILED')).toBe(false);
    expect(en.choice('NEW')).toBeDefined();
  });
});
