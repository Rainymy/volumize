export type EnumKeyFromValue<
  E extends Record<string, string | number>,
  V extends E[keyof E]
> = {
  [K in keyof E]: E[K] extends V ? K : never;
}[keyof E];