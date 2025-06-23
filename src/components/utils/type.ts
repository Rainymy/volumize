export type EnumKeyFromValue<
  E extends Record<string, string | number>,
  V extends E[keyof E]
> = {
  [K in keyof E]: E[K] extends V ? K : never;
}[keyof E];

export type GetSessionType<
  K extends Record<string, string | number>,
  T extends K[keyof K]
> = EnumKeyFromValue<K, T>

// enum Hello { Foo = "Foo", Bar = "Bar" }
// export type World = GetSessionType<typeof Hello, Hello.Bar>