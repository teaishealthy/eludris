export enum ItemType {
  Struct = 'struct',
  Enum = 'enum',
  Route = 'route'
}

export interface FieldInfo {
  name: string;
  doc: string | null;
  field_type: string;
  flattened: boolean;
  nullable: boolean;
  ommitable: boolean;
}

export interface StructInfo {
  type: ItemType.Struct;
  fields: FieldInfo[];
}

export enum VariantType {
  Unit = 'unit',
  Tuple = 'tuple',
  Struct = 'struct'
}

export interface UnitEnumVariant {
  type: VariantType.Unit;
  name: string;
  doc: string | null;
}

export interface TupleEnumVariant {
  type: VariantType.Tuple;
  name: string;
  doc: string | null;
  field_type: string;
}

export interface StructEnumVariant {
  type: VariantType.Struct;
  name: string;
  doc: string | null;
  fields: FieldInfo[];
}

export type EnumVariant = UnitEnumVariant | TupleEnumVariant | StructEnumVariant;

export interface EnumInfo {
  type: ItemType.Enum;
  tag: string | null;
  untagged: boolean;
  content: string | null;
  rename_all: string | null;
  variants: EnumVariant[];
}

export interface PathParamInfo {
  name: string;
  param_type: string;
}

export interface QueryParamInfo {
  name: string;
  param_type: string;
}

export interface RouteInfo {
  type: ItemType.Route;
  method: string;
  route: string;
  path_params: PathParamInfo[];
  query_params: QueryParamInfo[];
  body_type: string | null;
  return_type: string | null;
  guards: string[];
}

export type Item = StructInfo | EnumInfo | RouteInfo;

export interface ItemInfo {
  name: string;
  doc: string;
  category: string;
  hidden: boolean;
  package: string;
  item: Item;
}
