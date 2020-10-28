// This file is @generated by syn-serde-internal-codegen.
// It is not intended for manual editing.

use crate::*;
/// An adapter for [`enum@syn::AttrStyle`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttrStyle {
    Outer,
    Inner,
}
/// An adapter for [`enum@syn::BinOp`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BinOp {
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "-")]
    Sub,
    #[serde(rename = "*")]
    Mul,
    #[serde(rename = "/")]
    Div,
    #[serde(rename = "%")]
    Rem,
    #[serde(rename = "&&")]
    And,
    #[serde(rename = "||")]
    Or,
    #[serde(rename = "^")]
    BitXor,
    #[serde(rename = "&")]
    BitAnd,
    #[serde(rename = "|")]
    BitOr,
    #[serde(rename = "<<")]
    Shl,
    #[serde(rename = ">>")]
    Shr,
    #[serde(rename = "==")]
    Eq,
    #[serde(rename = "<")]
    Lt,
    #[serde(rename = "<=")]
    Le,
    #[serde(rename = "!=")]
    Ne,
    #[serde(rename = ">=")]
    Ge,
    #[serde(rename = ">")]
    Gt,
    #[serde(rename = "+=")]
    AddEq,
    #[serde(rename = "-=")]
    SubEq,
    #[serde(rename = "*=")]
    MulEq,
    #[serde(rename = "/=")]
    DivEq,
    #[serde(rename = "%=")]
    RemEq,
    #[serde(rename = "^=")]
    BitXorEq,
    #[serde(rename = "&=")]
    BitAndEq,
    #[serde(rename = "|=")]
    BitOrEq,
    #[serde(rename = "<<=")]
    ShlEq,
    #[serde(rename = ">>=")]
    ShrEq,
}
/// An adapter for [`enum@syn::Expr`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Expr {
    Array(ExprArray),
    Assign(ExprAssign),
    AssignOp(ExprAssignOp),
    Async(ExprAsync),
    Await(ExprAwait),
    Binary(ExprBinary),
    Block(ExprBlock),
    Box(ExprBox),
    Break(ExprBreak),
    Call(ExprCall),
    Cast(ExprCast),
    Closure(ExprClosure),
    Continue(ExprContinue),
    Field(ExprField),
    ForLoop(ExprForLoop),
    Group(ExprGroup),
    If(ExprIf),
    Index(ExprIndex),
    Let(ExprLet),
    Lit(ExprLit),
    Loop(ExprLoop),
    Macro(ExprMacro),
    Match(ExprMatch),
    MethodCall(ExprMethodCall),
    Paren(ExprParen),
    Path(ExprPath),
    Range(ExprRange),
    Reference(ExprReference),
    Repeat(ExprRepeat),
    Return(ExprReturn),
    Struct(ExprStruct),
    Try(ExprTry),
    TryBlock(ExprTryBlock),
    Tuple(ExprTuple),
    Type(ExprType),
    Unary(ExprUnary),
    Unsafe(ExprUnsafe),
    Verbatim(TokenStream),
    While(ExprWhile),
    Yield(ExprYield),
    #[doc(hidden)]
    __Nonexhaustive,
}
/// An adapter for [`enum@syn::Fields`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Fields {
    Named(FieldsNamed),
    Unnamed(FieldsUnnamed),
    Unit,
}
/// An adapter for [`enum@syn::FnArg`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FnArg {
    Receiver(Receiver),
    Typed(PatType),
}
/// An adapter for [`enum@syn::ForeignItem`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForeignItem {
    Fn(ForeignItemFn),
    Static(ForeignItemStatic),
    Type(ForeignItemType),
    Macro(ForeignItemMacro),
    Verbatim(TokenStream),
    #[doc(hidden)]
    __Nonexhaustive,
}
/// An adapter for [`enum@syn::GenericArgument`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenericArgument {
    Lifetime(Lifetime),
    Type(Type),
    Binding(Binding),
    Constraint(Constraint),
    Const(Expr),
}
/// An adapter for [`enum@syn::GenericMethodArgument`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenericMethodArgument {
    Type(Type),
    Const(Expr),
}
/// An adapter for [`enum@syn::GenericParam`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenericParam {
    Type(TypeParam),
    Lifetime(LifetimeDef),
    Const(ConstParam),
}
/// An adapter for [`enum@syn::ImplItem`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImplItem {
    Const(ImplItemConst),
    Method(ImplItemMethod),
    Type(ImplItemType),
    Macro(ImplItemMacro),
    Verbatim(TokenStream),
    #[doc(hidden)]
    __Nonexhaustive,
}
/// An adapter for [`enum@syn::Item`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Item {
    Const(ItemConst),
    Enum(ItemEnum),
    ExternCrate(ItemExternCrate),
    Fn(ItemFn),
    ForeignMod(ItemForeignMod),
    Impl(ItemImpl),
    Macro(ItemMacro),
    Macro2(ItemMacro2),
    Mod(ItemMod),
    Static(ItemStatic),
    Struct(ItemStruct),
    Trait(ItemTrait),
    TraitAlias(ItemTraitAlias),
    Type(ItemType),
    Union(ItemUnion),
    Use(ItemUse),
    Verbatim(TokenStream),
    #[doc(hidden)]
    __Nonexhaustive,
}
/// An adapter for [`enum@syn::Lit`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Lit {
    Str(LitStr),
    ByteStr(LitByteStr),
    Byte(LitByte),
    Char(LitChar),
    Int(LitInt),
    Float(LitFloat),
    Bool(LitBool),
    Verbatim(Literal),
}
/// An adapter for [`enum@syn::MacroDelimiter`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroDelimiter {
    Paren,
    Brace,
    Bracket,
}
/// An adapter for [`enum@syn::Member`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Member {
    #[serde(rename = "ident")]
    Named(Ident),
    #[serde(rename = "index")]
    Unnamed(Index),
}
/// An adapter for [`enum@syn::Meta`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Meta {
    Path(Path),
    List(MetaList),
    NameValue(MetaNameValue),
}
/// An adapter for [`enum@syn::NestedMeta`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NestedMeta {
    Meta(Meta),
    Lit(Lit),
}
/// An adapter for [`enum@syn::Pat`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Pat {
    Box(PatBox),
    Ident(PatIdent),
    Lit(PatLit),
    Macro(PatMacro),
    Or(PatOr),
    Path(PatPath),
    Range(PatRange),
    Reference(PatReference),
    Rest(PatRest),
    Slice(PatSlice),
    Struct(PatStruct),
    Tuple(PatTuple),
    TupleStruct(PatTupleStruct),
    Type(PatType),
    Verbatim(TokenStream),
    #[serde(rename = "_")]
    Wild(PatWild),
    #[doc(hidden)]
    __Nonexhaustive,
}
/// An adapter for [`enum@syn::PathArguments`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathArguments {
    None,
    AngleBracketed(AngleBracketedGenericArguments),
    Parenthesized(ParenthesizedGenericArguments),
}
/// An adapter for [`enum@syn::RangeLimits`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RangeLimits {
    #[serde(rename = "..")]
    HalfOpen,
    #[serde(rename = "..=")]
    Closed,
}
/// An adapter for [`enum@syn::Stmt`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stmt {
    #[serde(rename = "let")]
    Local(Local),
    Item(Item),
    Expr(Expr),
    Semi(Expr),
}
/// An adapter for [`enum@syn::TraitBoundModifier`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraitBoundModifier {
    None,
    Maybe,
}
/// An adapter for [`enum@syn::TraitItem`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraitItem {
    Const(TraitItemConst),
    Method(TraitItemMethod),
    Type(TraitItemType),
    Macro(TraitItemMacro),
    Verbatim(TokenStream),
    #[doc(hidden)]
    __Nonexhaustive,
}
/// An adapter for [`enum@syn::Type`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Array(TypeArray),
    BareFn(TypeBareFn),
    Group(TypeGroup),
    ImplTrait(TypeImplTrait),
    #[serde(rename = "_")]
    Infer,
    Macro(TypeMacro),
    #[serde(rename = "!")]
    Never,
    Paren(TypeParen),
    Path(TypePath),
    Ptr(TypePtr),
    Reference(TypeReference),
    Slice(TypeSlice),
    TraitObject(TypeTraitObject),
    Tuple(TypeTuple),
    Verbatim(TokenStream),
    #[doc(hidden)]
    __Nonexhaustive,
}
/// An adapter for [`enum@syn::TypeParamBound`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeParamBound {
    Trait(TraitBound),
    Lifetime(Lifetime),
}
/// An adapter for [`enum@syn::UnOp`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnOp {
    #[serde(rename = "*")]
    Deref,
    #[serde(rename = "!")]
    Not,
    #[serde(rename = "-")]
    Neg,
}
/// An adapter for [`enum@syn::UseTree`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UseTree {
    Path(UsePath),
    #[serde(rename = "ident")]
    Name(UseName),
    Rename(UseRename),
    #[serde(rename = "*")]
    Glob,
    Group(UseGroup),
}
/// An adapter for [`enum@syn::Visibility`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    #[serde(rename = "pub")]
    Public,
    Crate,
    Restricted(VisRestricted),
    Inherited,
}
/// An adapter for [`enum@syn::WherePredicate`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WherePredicate {
    Type(PredicateType),
    Lifetime(PredicateLifetime),
    Eq(PredicateEq),
}
