use crate::*;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Type {
    NonNull(Box<Type>),
    List(Box<Type>),
    Named(String),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Int(i64),
    Float(f64),
    Enum(String),
    String(String),
    Object(BTreeMap<String, Value>),
    List(Vec<Value>),
}

#[derive(Debug, PartialEq)]
pub struct Document {
    pub definition_list: Vec<Positioned<DocumentDefinition>>,
}

#[derive(Debug, PartialEq)]
pub enum DocumentDefinition {
    Operation(Positioned<OperationDefinition>),
    Fragment(Positioned<FragmentDefinition>),
    Schema(Positioned<SchemaDefinition>),
    Type(Positioned<TypeDefinition>),
    Directive(Positioned<DirectiveDefinition>),
}

#[derive(Debug, PartialEq)]
pub struct TypeSystemDocument {
    pub definition_list: Vec<Positioned<TypeSystemDefinition>>,
}

#[derive(Debug, PartialEq)]
pub enum TypeSystemDefinition {
    Schema(Positioned<SchemaDefinition>),
    Type(Positioned<TypeDefinition>),
    Directive(Positioned<DirectiveDefinition>),
}

#[derive(Debug, PartialEq)]
pub struct ExecutableDocument {
    pub definition_list: Vec<Positioned<ExecutableDefinition>>,
}

#[derive(Debug, PartialEq)]
pub enum ExecutableDefinition {
    Operation(Positioned<OperationDefinition>),
    Fragment(Positioned<FragmentDefinition>),
}

#[derive(Debug, PartialEq)]
pub struct OperationDefinition {
    pub ty: OperationType,
    pub name: Option<Positioned<String>>,
    pub variable_list: Vec<Positioned<VariableDefinition>>,
    pub selection_list: Vec<Positioned<Selection>>,
    pub directive_list: Vec<Positioned<Directive>>,
}

#[derive(Debug, PartialEq)]
pub struct FragmentDefinition {
    pub name: Positioned<String>,
    pub on: Positioned<String>,
    pub selection_list: Vec<Positioned<Selection>>,
    pub directive_list: Vec<Positioned<Directive>>,
}

#[derive(Debug, PartialEq)]
pub struct SchemaDefinition {
    pub is_extend: bool,
    pub description: Option<Positioned<String>>,
    pub directive_list: Vec<Positioned<Directive>>,
    pub field_list: Vec<OperationField>,
}

#[derive(Debug, PartialEq)]
pub struct VariableDefinition {
    pub name: Positioned<String>,
    pub ty: Positioned<Type>,
    pub default_value: Option<Positioned<Value>>,
}

#[derive(Debug, PartialEq)]
pub struct OperationField {
    pub ty: OperationType,
    pub name: Positioned<String>,
}

#[derive(Debug, PartialEq)]
pub enum OperationType {
    Query,
    Mutation,
    Subscription,
}

#[derive(Debug, PartialEq)]
pub enum TypeDefinition {
    Scalar(Positioned<ScalarType>),
    Object(Positioned<ObjectType>),
    Interface(Positioned<InterfaceType>),
    Union(Positioned<UnionType>),
    Enum(Positioned<EnumType>),
    InputObject(Positioned<InputObjectType>),
}

#[derive(Debug, PartialEq)]
pub struct ScalarType {
    pub is_extend: bool,
    pub description: Option<Positioned<String>>,
    pub name: Positioned<String>,
    pub directive_list: Vec<Positioned<Directive>>,
}

#[derive(Debug, PartialEq)]
pub struct ObjectType {
    pub is_extend: bool,
    pub description: Option<Positioned<String>>,
    pub interface_list: Vec<Positioned<String>>,
    pub name: Positioned<String>,
    pub field_list: Vec<Positioned<FieldDefinition>>,
    pub directive_list: Vec<Positioned<Directive>>,
}

#[derive(Debug, PartialEq)]
pub struct InterfaceType {
    pub is_extend: bool,
    pub description: Option<Positioned<String>>,
    pub name: Positioned<String>,
    pub field_list: Vec<Positioned<FieldDefinition>>,
    pub directive_list: Vec<Positioned<Directive>>,
}

#[derive(Debug, PartialEq)]
pub struct UnionType {
    pub is_extend: bool,
    pub description: Option<Positioned<String>>,
    pub name: Positioned<String>,
    pub member_list: Vec<Positioned<String>>,
    pub directive_list: Vec<Positioned<Directive>>,
}

#[derive(Debug, PartialEq)]
pub struct EnumType {
    pub is_extend: bool,
    pub description: Option<Positioned<String>>,
    pub name: Positioned<String>,
    pub member_list: Vec<Positioned<EnumMember>>,
    pub directive_list: Vec<Positioned<Directive>>,
}

#[derive(Debug, PartialEq)]
pub struct InputObjectType {
    pub is_extend: bool,
    pub description: Option<Positioned<String>>,
    pub name: Positioned<String>,
    pub field_list: Vec<Positioned<FieldDefinition>>,
    pub directive_list: Vec<Positioned<Directive>>,
}

#[derive(Debug, PartialEq)]
pub struct DirectiveDefinition {
    pub description: Option<Positioned<String>>,
    pub name: Positioned<String>,
    pub argument_list: Vec<Positioned<FieldArgument>>,
    pub location_list: Vec<Positioned<DirectiveLocation>>,
}

#[derive(Debug, PartialEq)]
pub enum Selection {
    Field(Field),
    FragmentSpread(FragmentSpread),
    InlineFragment(InlineFragment),
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: Positioned<String>,
    pub alias: Option<Positioned<String>>,
    pub argument_list: Vec<Positioned<FieldArgument>>,
    pub selection_list: Vec<Positioned<Selection>>,
    pub directive_list: Vec<Positioned<Directive>>,
}

#[derive(Debug, PartialEq)]
pub struct FragmentSpread {
    pub name: Positioned<String>,
    pub directive_list: Vec<Positioned<Directive>>,
}

#[derive(Debug, PartialEq)]
pub struct InlineFragment {
    pub on: Option<Positioned<String>>,
    pub selection_list: Vec<Positioned<Selection>>,
    pub directive_list: Vec<Positioned<Directive>>,
}

#[derive(Debug, PartialEq)]
pub struct FieldDefinition {
    pub description: Option<Positioned<String>>,
    pub name: Positioned<String>,
    pub argument_list: Vec<Positioned<FieldArgument>>,
    pub ty: Positioned<Type>,
    pub directive_list: Vec<Positioned<Directive>>,
}

#[derive(Debug, PartialEq)]
pub struct FieldArgument {
    pub description: Option<Positioned<String>>,
    pub name: Positioned<String>,
    pub ty: Positioned<Type>,
    pub default_value: Option<Positioned<Value>>,
    pub directive_list: Vec<Positioned<Directive>>,
}

#[derive(Debug, PartialEq)]
pub struct EnumMember {
    pub description: Option<Positioned<String>>,
    pub name: Positioned<String>,
    pub directive_list: Vec<Positioned<Directive>>,
}

#[derive(Debug, PartialEq)]
pub enum DirectiveLocation {
    Executable(ExecutableDirectiveLocation),
    TypeSystem(TypeSystemDirectiveLocation),
}

#[derive(Debug, PartialEq)]
pub enum ExecutableDirectiveLocation {
    Query,
    Mutation,
    Subscription,
    Field,
    FragmentDefinition,
    FragmentSpread,
    InlineFragment,
}

#[derive(Debug, PartialEq)]
pub enum TypeSystemDirectiveLocation {
    Schema,
    Scalar,
    Object,
    FieldDefinition,
    ArgumentDefinition,
    Interface,
    Union,
    Enum,
    EnumValue,
    InputObject,
    InputFieldDefinition,
}

#[derive(Debug, PartialEq)]
pub struct Directive {
    pub name: Positioned<String>,
    pub argument_list: Vec<Positioned<FieldArgument>>,
}
