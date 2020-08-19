// @generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: proto/iomod.capnp



pub mod iomod {
  #![allow(unused_variables)]
  pub type GetInfoParams<> = ::capnp::capability::Params<crate::iomod_capnp::iomod::get_info_params::Owned>;
  pub type GetInfoResults<> = ::capnp::capability::Results<crate::iomod_capnp::iomod::get_info_results::Owned>;

  pub struct Client {
    pub client: ::capnp::capability::Client,
  }
  impl  ::capnp::capability::FromClientHook for Client {
    fn new(hook: Box<dyn (::capnp::private::capability::ClientHook)>) -> Client {
      Client { client: ::capnp::capability::Client::new(hook),  }
    }
  }
  #[derive(Copy, Clone)]
  pub struct Owned;
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Client; type Builder = Client; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Client; }
  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Client<>  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, _default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Client<>> {
      ::core::result::Result::Ok(::capnp::capability::FromClientHook::new(reader.get_capability()?))
    }
  }
  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Client<>  {
    fn init_pointer(_builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Client<> {
      unimplemented!()
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Client<>> {
      ::core::result::Result::Ok(::capnp::capability::FromClientHook::new(builder.get_capability()?))
    }
  }

  impl <> ::capnp::traits::SetPointerBuilder<Client<>> for Client<>  {
    fn set_pointer_builder(pointer: ::capnp::private::layout::PointerBuilder, from: Client<>, _canonicalize: bool) -> ::capnp::Result<()> {
      pointer.set_capability(from.client.hook);
      ::core::result::Result::Ok(())
    }
  }
  impl  ::capnp::traits::HasTypeId for Client {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl  Clone for Client {
    fn clone(&self) -> Client {
      Client { client: ::capnp::capability::Client::new(self.client.hook.add_ref()),  }
    }
  }
  impl  Client {
    pub fn get_info_request(&self) -> ::capnp::capability::Request<crate::iomod_capnp::iomod::get_info_params::Owned,crate::iomod_capnp::iomod::get_info_results::Owned> {
      self.client.new_call(_private::TYPE_ID, 0, None)
    }
  }
  pub trait Server<>   {
    fn get_info(&mut self, _: GetInfoParams<>, _: GetInfoResults<>) -> ::capnp::capability::Promise<(), ::capnp::Error> { ::capnp::capability::Promise::err(::capnp::Error::unimplemented("method not implemented".to_string())) }
  }
  pub struct ServerDispatch<_T,> {
    pub server: _T,
  }
  impl <_S: Server + 'static, > ::capnp::capability::FromServer<_S> for Client   {
    type Dispatch = ServerDispatch<_S, >;
    fn from_server(s: _S) -> ServerDispatch<_S, > {
      ServerDispatch { server: s,  }
    }
  }
  impl <_T: Server> ::core::ops::Deref for ServerDispatch<_T> {
    type Target = _T;
    fn deref(&self) -> &_T { &self.server}
  }
  impl <_T: Server> ::core::ops::DerefMut for ServerDispatch<_T> {
    fn deref_mut(&mut self) -> &mut _T { &mut self.server}
  }
  impl <_T: Server> ::capnp::capability::Server for ServerDispatch<_T> {
    fn dispatch_call(&mut self, interface_id: u64, method_id: u16, params: ::capnp::capability::Params<::capnp::any_pointer::Owned>, results: ::capnp::capability::Results<::capnp::any_pointer::Owned>) -> ::capnp::capability::Promise<(), ::capnp::Error> {
      match interface_id {
        _private::TYPE_ID => ServerDispatch::<_T, >::dispatch_call_internal(&mut self.server, method_id, params, results),
        _ => { ::capnp::capability::Promise::err(::capnp::Error::unimplemented("Method not implemented.".to_string())) }
      }
    }
  }
  impl <_T :Server> ServerDispatch<_T> {
    pub fn dispatch_call_internal(server: &mut _T, method_id: u16, params: ::capnp::capability::Params<::capnp::any_pointer::Owned>, results: ::capnp::capability::Results<::capnp::any_pointer::Owned>) -> ::capnp::capability::Promise<(), ::capnp::Error> {
      match method_id {
        0 => server.get_info(::capnp::private::capability::internal_get_typed_params(params), ::capnp::private::capability::internal_get_typed_results(results)),
        _ => { ::capnp::capability::Promise::err(::capnp::Error::unimplemented("Method not implemented.".to_string())) }
      }
    }
  }
  pub mod _private {
    pub const TYPE_ID: u64 = 0xed59_8875_3935_db88;
  }

  pub mod agent_info {
    #[derive(Copy, Clone)]
    pub struct Owned;
    impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

    #[derive(Clone, Copy)]
    pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

    impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
      fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
        Reader { reader,  }
      }
    }

    impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
      fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
        ::core::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
      }
    }

    impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
      fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
        self.reader
      }
    }

    impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
      fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
        self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
      }
    }

    impl <'a,> Reader<'a,>  {
      pub fn reborrow(&self) -> Reader<> {
        Reader { .. *self }
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.reader.total_size()
      }
      #[inline]
      pub fn get_organization(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
        ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::core::option::Option::None)
      }
      pub fn has_organization(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_namespace(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
        ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(1), ::core::option::Option::None)
      }
      pub fn has_namespace(&self) -> bool {
        !self.reader.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn get_name(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
        ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(2), ::core::option::Option::None)
      }
      pub fn has_name(&self) -> bool {
        !self.reader.get_pointer_field(2).is_null()
      }
    }

    pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
    impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
      #[inline]
      fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
    }
    impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
      fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
        Builder { builder,  }
      }
    }

    impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
      fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
        self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
      }
    }

    impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
      fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
        ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
      }
      fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
        ::core::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
      }
    }

    impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
      fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
    }

    impl <'a,> Builder<'a,>  {
      pub fn into_reader(self) -> Reader<'a,> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }
      pub fn reborrow(&mut self) -> Builder<> {
        Builder { .. *self }
      }
      pub fn reborrow_as_reader(&self) -> Reader<> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.builder.into_reader().total_size()
      }
      #[inline]
      pub fn get_organization(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
        ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::core::option::Option::None)
      }
      #[inline]
      pub fn set_organization(&mut self, value: ::capnp::text::Reader)  {
        self.builder.get_pointer_field(0).set_text(value);
      }
      #[inline]
      pub fn init_organization(self, size: u32) -> ::capnp::text::Builder<'a> {
        self.builder.get_pointer_field(0).init_text(size)
      }
      pub fn has_organization(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_namespace(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
        ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(1), ::core::option::Option::None)
      }
      #[inline]
      pub fn set_namespace(&mut self, value: ::capnp::text::Reader)  {
        self.builder.get_pointer_field(1).set_text(value);
      }
      #[inline]
      pub fn init_namespace(self, size: u32) -> ::capnp::text::Builder<'a> {
        self.builder.get_pointer_field(1).init_text(size)
      }
      pub fn has_namespace(&self) -> bool {
        !self.builder.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn get_name(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
        ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(2), ::core::option::Option::None)
      }
      #[inline]
      pub fn set_name(&mut self, value: ::capnp::text::Reader)  {
        self.builder.get_pointer_field(2).set_text(value);
      }
      #[inline]
      pub fn init_name(self, size: u32) -> ::capnp::text::Builder<'a> {
        self.builder.get_pointer_field(2).init_text(size)
      }
      pub fn has_name(&self) -> bool {
        !self.builder.get_pointer_field(2).is_null()
      }
    }

    pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
    impl ::capnp::capability::FromTypelessPipeline for Pipeline {
      fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
        Pipeline { _typeless: typeless,  }
      }
    }
    impl Pipeline  {
    }
    mod _private {
      use capnp::private::layout;
      pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 0, pointers: 3 };
      pub const TYPE_ID: u64 = 0x8052_b978_21c0_78ab;
    }
  }

  pub mod get_info_params {
    #[derive(Copy, Clone)]
    pub struct Owned;
    impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

    #[derive(Clone, Copy)]
    pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

    impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
      fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
        Reader { reader,  }
      }
    }

    impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
      fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
        ::core::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
      }
    }

    impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
      fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
        self.reader
      }
    }

    impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
      fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
        self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
      }
    }

    impl <'a,> Reader<'a,>  {
      pub fn reborrow(&self) -> Reader<> {
        Reader { .. *self }
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.reader.total_size()
      }
    }

    pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
    impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
      #[inline]
      fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
    }
    impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
      fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
        Builder { builder,  }
      }
    }

    impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
      fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
        self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
      }
    }

    impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
      fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
        ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
      }
      fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
        ::core::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
      }
    }

    impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
      fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
    }

    impl <'a,> Builder<'a,>  {
      pub fn into_reader(self) -> Reader<'a,> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }
      pub fn reborrow(&mut self) -> Builder<> {
        Builder { .. *self }
      }
      pub fn reborrow_as_reader(&self) -> Reader<> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.builder.into_reader().total_size()
      }
    }

    pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
    impl ::capnp::capability::FromTypelessPipeline for Pipeline {
      fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
        Pipeline { _typeless: typeless,  }
      }
    }
    impl Pipeline  {
    }
    mod _private {
      use capnp::private::layout;
      pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 0, pointers: 0 };
      pub const TYPE_ID: u64 = 0xce55_ea99_6032_31fd;
    }
  }

  pub mod get_info_results {
    #[derive(Copy, Clone)]
    pub struct Owned;
    impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

    #[derive(Clone, Copy)]
    pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

    impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
      fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
        Reader { reader,  }
      }
    }

    impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
      fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
        ::core::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
      }
    }

    impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
      fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
        self.reader
      }
    }

    impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
      fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
        self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
      }
    }

    impl <'a,> Reader<'a,>  {
      pub fn reborrow(&self) -> Reader<> {
        Reader { .. *self }
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.reader.total_size()
      }
      #[inline]
      pub fn get_info(self) -> ::capnp::Result<crate::iomod_capnp::iomod::agent_info::Reader<'a>> {
        ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::core::option::Option::None)
      }
      pub fn has_info(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
    }

    pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
    impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
      #[inline]
      fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
    }
    impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
      fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
        Builder { builder,  }
      }
    }

    impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
      fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
        self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
      }
    }

    impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
      fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
        ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
      }
      fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
        ::core::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
      }
    }

    impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
      fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
    }

    impl <'a,> Builder<'a,>  {
      pub fn into_reader(self) -> Reader<'a,> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }
      pub fn reborrow(&mut self) -> Builder<> {
        Builder { .. *self }
      }
      pub fn reborrow_as_reader(&self) -> Reader<> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.builder.into_reader().total_size()
      }
      #[inline]
      pub fn get_info(self) -> ::capnp::Result<crate::iomod_capnp::iomod::agent_info::Builder<'a>> {
        ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::core::option::Option::None)
      }
      #[inline]
      pub fn set_info<'b>(&mut self, value: crate::iomod_capnp::iomod::agent_info::Reader<'b>) -> ::capnp::Result<()> {
        ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(0), value, false)
      }
      #[inline]
      pub fn init_info(self, ) -> crate::iomod_capnp::iomod::agent_info::Builder<'a> {
        ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), 0)
      }
      pub fn has_info(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
    }

    pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
    impl ::capnp::capability::FromTypelessPipeline for Pipeline {
      fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
        Pipeline { _typeless: typeless,  }
      }
    }
    impl Pipeline  {
      pub fn get_info(&self) -> crate::iomod_capnp::iomod::agent_info::Pipeline {
        ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
      }
    }
    mod _private {
      use capnp::private::layout;
      pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 0, pointers: 1 };
      pub const TYPE_ID: u64 = 0xb462_18f1_9b2c_db02;
    }
  }
}
