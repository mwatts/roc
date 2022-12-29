interface Html.Internal
    exposes [
        App,
        Html,
        Attribute,
        CyclicStructureAccessor,
        Handler,
        initServerApp,
        initClientApp,
    ]
    imports [
        Action.{ Action },
        Effect.{ Effect },
        Encode,
        Json,
    ]

PlatformState state initData : {
    app : App state initData,
    state,
    view : RenderedHtml,
    handlerLookup : HandlerLookup state,
    isOddArena : Bool,
}

HandlerLookup state : {
    handlers : List (Result (Handler state) [NoHandler]),
    freeList : List Nat,
}

App state initData : {
    init : DecodingResult initData -> state,
    render : state -> Html state,
    wasmUrl : Str,
}

DecodingResult a : Result a [Leftover (List U8), TooShort]

Html state : [
    None,
    Text Str,
    Element Str Size (List (Attribute state)) (List (Html state)),
]

RenderedHtml : [
    RenderedNone,
    RenderedText JsIndex Str,
    RenderedElement JsIndex Str Size (List RenderedAttribute) (List RenderedHtml),
]

JsIndex : Nat
Size : Nat

Attribute state : [
    EventListener Str (List CyclicStructureAccessor) (Handler state),
    HtmlAttr Str Str,
    DomProp Str (List U8),
    Style Str Str,
]

RenderedAttribute : [
    RenderedEventListener Str (List CyclicStructureAccessor) HandlerId,
    RenderedHtmlAttr Str Str,
    RenderedDomProp Str (List U8),
    RenderedStyle Str Str,
]

HandlerId : Nat

CyclicStructureAccessor : [
    ObjectField Str CyclicStructureAccessor,
    ArrayIndex Nat CyclicStructureAccessor,
    SerializableValue,
]

Handler state := [
    Normal (state, List (List U8) -> Action state),
    Custom (state, List (List U8) -> { action : Action state, stopPropagation : Bool, preventDefault : Bool }),
]


translateStatic : Html state -> Html *

initServerApp : App state initData, initData, Str -> Html []
initServerApp = \app, initData, hostJavaScript ->
    translateStatic None

initClientApp : List U8, App state initData -> { list : List RenderedHtml, index : Nat }
initClientApp = \json, app ->
    indexNodes { list: [], index: 0 } None

indexNodes : { list : List RenderedHtml, index : Nat }, Html state -> { list : List RenderedHtml, index : Nat }
indexNodes = \{ list, index }, unrendered ->
    List.walk [] { list, index } indexNodes
