
/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `api`.
///
/// This structure is created through [`ApiPre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`Api`] as well.
pub struct ApiPre<T> {
  instance_pre: wasmtime::component::InstancePre<T>,
  indices: ApiIndices,
}

impl<T> Clone for ApiPre<T> {
  fn clone(&self) -> Self {
    Self {
      instance_pre: self.instance_pre.clone(),
      indices: self.indices.clone(),
    }
  }
}

impl<_T> ApiPre<_T> {
  /// Creates a new copy of `ApiPre` bindings which can then
  /// be used to instantiate into a particular store.
  ///
  /// This method may fail if the component behind `instance_pre`
  /// does not have the required exports.
  pub fn new(instance_pre: wasmtime::component::InstancePre<_T>) -> wasmtime::Result<Self> {
    let indices = ApiIndices::new(&instance_pre)?;
    Ok(Self { instance_pre, indices })
  }
  
  pub fn engine(&self) -> &wasmtime::Engine {
    self.instance_pre.engine()
  }
  
  pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
    &self.instance_pre
  }
  
  /// Instantiates a new instance of [`Api`] within the
  /// `store` provided.
  ///
  /// This function will use `self` as the pre-instantiated
  /// instance to perform instantiation. Afterwards the preloaded
  /// indices in `self` are used to lookup all exports on the
  /// resulting instance.
  pub async fn instantiate_async(
  &self,
  mut store: impl wasmtime::AsContextMut<Data = _T>,
  ) -> wasmtime::Result<Api>
  where _T: Send
  {
    let mut store = store.as_context_mut();
    let instance = self.instance_pre.instantiate_async(&mut store).await?;
    self.indices.load(&mut store, &instance)
  }
}


/// Auto-generated bindings for index of the exports of
/// `api`.
///
/// This is an implementation detail of [`ApiPre`] and can
/// be constructed if needed as well.
///
/// For more information see [`Api`] as well.
#[derive(Clone)]
pub struct ApiIndices {
  interface0: exports::repl::api::transport::GuestIndices,
  interface1: exports::repl::api::repl::GuestIndices,
  interface2: exports::repl::api::http_client::GuestIndices,
  interface3: exports::repl::api::plugin::GuestIndices,
  interface4: exports::repl::api::plugin_runner::GuestIndices,
}

/// Auto-generated bindings for an instance a component which
/// implements the world `api`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`Api::instantiate_async`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`ApiPre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`ApiPre::instantiate_async`] to
///   create a [`Api`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`Api::new`].
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct Api {
  interface0: exports::repl::api::transport::Guest,
  interface1: exports::repl::api::repl::Guest,
  interface2: exports::repl::api::http_client::Guest,
  interface3: exports::repl::api::plugin::Guest,
  interface4: exports::repl::api::plugin_runner::Guest,
}
const _: () = {
  
  #[allow(unused_imports)]
  use wasmtime::component::__internal::anyhow;
  
  impl ApiIndices {
    /// Creates a new copy of `ApiIndices` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component does not have the
    /// required exports.
    pub fn new<_T>(_instance_pre: &wasmtime::component::InstancePre<_T>) -> wasmtime::Result<Self> {
      let _component = _instance_pre.component();
      let _instance_type = _instance_pre.instance_type();
      
      let interface0 = exports::repl::api::transport::GuestIndices::new(_instance_pre)?;
      let interface1 = exports::repl::api::repl::GuestIndices::new(_instance_pre)?;
      let interface2 = exports::repl::api::http_client::GuestIndices::new(_instance_pre)?;
      let interface3 = exports::repl::api::plugin::GuestIndices::new(_instance_pre)?;
      let interface4 = exports::repl::api::plugin_runner::GuestIndices::new(_instance_pre)?;
      Ok(ApiIndices {
        interface0,
        interface1,
        interface2,
        interface3,
        interface4,
      })
    }
    
    /// Uses the indices stored in `self` to load an instance
    /// of [`Api`] from the instance provided.
    ///
    /// Note that at this time this method will additionally
    /// perform type-checks of all exports.
    pub fn load(
    &self,
    mut store: impl wasmtime::AsContextMut,
    instance: &wasmtime::component::Instance,
    ) -> wasmtime::Result<Api> {
      let _ = &mut store;
      let _instance = instance;
      
      let interface0 = self.interface0.load(&mut store, &_instance)?;
      let interface1 = self.interface1.load(&mut store, &_instance)?;
      let interface2 = self.interface2.load(&mut store, &_instance)?;
      let interface3 = self.interface3.load(&mut store, &_instance)?;
      let interface4 = self.interface4.load(&mut store, &_instance)?;
      Ok(Api {
        interface0,
        interface1,
        interface2,
        interface3,
        interface4,
      })
    }
  }
  impl Api{
    /// Convenience wrapper around [`ApiPre::new`] and
    /// [`ApiPre::instantiate_async`].
    pub async fn instantiate_async<_T>(
    store: impl wasmtime::AsContextMut<Data = _T>,
    component: &wasmtime::component::Component,
    linker: &wasmtime::component::Linker<_T>,
    ) -> wasmtime::Result<Api>
    where _T: Send
    {
      let pre = linker.instantiate_pre(component)?;
      ApiPre::new(pre)?.instantiate_async(store).await
    }
    
    /// Convenience wrapper around [`ApiIndices::new`] and
    /// [`ApiIndices::load`].
    pub fn new(
    mut store: impl wasmtime::AsContextMut,
    instance: &wasmtime::component::Instance,
    ) -> wasmtime::Result<Api> {
      let indices = ApiIndices::new(&instance.instance_pre(&store))?;
      indices.load(&mut store, instance)
    }
    
    
    pub fn repl_api_transport(&self) -> &exports::repl::api::transport::Guest {
      &self.interface0
    }
    
    pub fn repl_api_repl(&self) -> &exports::repl::api::repl::Guest {
      &self.interface1
    }
    
    pub fn repl_api_http_client(&self) -> &exports::repl::api::http_client::Guest {
      &self.interface2
    }
    
    pub fn repl_api_plugin(&self) -> &exports::repl::api::plugin::Guest {
      &self.interface3
    }
    
    pub fn repl_api_plugin_runner(&self) -> &exports::repl::api::plugin_runner::Guest {
      &self.interface4
    }
  }
};
pub mod exports {
  pub mod repl {
    pub mod api {
      
      #[allow(clippy::all)]
      pub mod transport {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::{anyhow, Box};
        
        #[derive(wasmtime::component::ComponentType)]
        #[derive(wasmtime::component::Lift)]
        #[derive(wasmtime::component::Lower)]
        #[component(enum)]
        #[derive(Clone, Copy, Eq, PartialEq)]
        #[repr(u8)]
        pub enum ReplStatus {
          #[component(name = "success")]Success,
          #[component(name = "error")]Error,
          #[component(name = "warning")]Warning,
        }
        impl core::fmt::Debug for ReplStatus {
          fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
              ReplStatus::Success => {
                f.debug_tuple("ReplStatus::Success").finish()
              }
              ReplStatus::Error => {
                f.debug_tuple("ReplStatus::Error").finish()
              }
              ReplStatus::Warning => {
                f.debug_tuple("ReplStatus::Warning").finish()
              }
            }
          }
        }
        const _: () = {
          assert!(1 == <ReplStatus as wasmtime::component::ComponentType>::SIZE32);
          assert!(1 == <ReplStatus as wasmtime::component::ComponentType>::ALIGN32);
        };
        #[derive(wasmtime::component::ComponentType)]
        #[derive(wasmtime::component::Lift)]
        #[derive(wasmtime::component::Lower)]
        #[component(record)]
        #[derive(Clone)]
        pub struct ReplResult {
          #[component(name = "color")]
          pub color: Option<wasmtime::component::__internal::String>,
          #[component(name = "status")]
          pub status: ReplStatus,
          #[component(name = "output")]
          pub output: Option<wasmtime::component::__internal::String>,
        }
        impl core::fmt::Debug for ReplResult {
          fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ReplResult").field("color", &self.color).field("status", &self.status).field("output", &self.output).finish()
          }
        }
        const _: () = {
          assert!(28 == <ReplResult as wasmtime::component::ComponentType>::SIZE32);
          assert!(4 == <ReplResult as wasmtime::component::ComponentType>::ALIGN32);
        };
        pub struct Guest {
        }
        #[derive(Clone)]
        pub struct GuestIndices {
        }
        impl GuestIndices {
          
          /// Constructor for [`GuestIndices`] which takes a
          /// [`Component`](wasmtime::component::Component) as input and can be executed
          /// before instantiation.
          ///
          /// This constructor can be used to front-load string lookups to find exports
          /// within a component.
          pub fn new<_T>(
          _instance_pre: &wasmtime::component::InstancePre<_T>,
          ) -> wasmtime::Result<GuestIndices> {
            let instance = _instance_pre.component().get_export_index(None, "repl:api/transport")
            .ok_or_else(|| anyhow::anyhow!("no exported instance named `repl:api/transport`"))?;
            let mut lookup = move |name| {
              _instance_pre.component().get_export_index(Some(&instance), name).ok_or_else(|| {
                anyhow::anyhow!(
                "instance export `repl:api/transport` does \
                not have export `{name}`"
                )
              })
            };
            let _ = &mut lookup;
            Ok(GuestIndices {
            })
          }
          
          pub fn load(
          &self,
          mut store: impl wasmtime::AsContextMut,
          instance: &wasmtime::component::Instance,
          ) -> wasmtime::Result<Guest> {
            let _instance = instance;
            let _instance_pre = _instance.instance_pre(&store);
            let _instance_type = _instance_pre.instance_type();
            let mut store = store.as_context_mut();
            let _ = &mut store;
            Ok(Guest {
            })
          }
        }
        impl Guest {
        }
        
      }
      
      
      #[allow(clippy::all)]
      pub mod repl {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::{anyhow, Box};
        
        pub type ReplResult = super::super::super::super::exports::repl::api::transport::ReplResult;
        const _: () = {
          assert!(28 == <ReplResult as wasmtime::component::ComponentType>::SIZE32);
          assert!(4 == <ReplResult as wasmtime::component::ComponentType>::ALIGN32);
        };
        #[derive(wasmtime::component::ComponentType)]
        #[derive(wasmtime::component::Lift)]
        #[derive(wasmtime::component::Lower)]
        #[component(record)]
        #[derive(Clone)]
        pub struct PluginConfig {
          #[component(name = "command")]
          pub command: wasmtime::component::__internal::String,
          #[component(name = "arg-count")]
          pub arg_count: Option<i8>,
          #[component(name = "man")]
          pub man: wasmtime::component::__internal::String,
        }
        impl core::fmt::Debug for PluginConfig {
          fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("PluginConfig").field("command", &self.command).field("arg-count", &self.arg_count).field("man", &self.man).finish()
          }
        }
        const _: () = {
          assert!(20 == <PluginConfig as wasmtime::component::ComponentType>::SIZE32);
          assert!(4 == <PluginConfig as wasmtime::component::ComponentType>::ALIGN32);
        };
        #[derive(wasmtime::component::ComponentType)]
        #[derive(wasmtime::component::Lift)]
        #[derive(wasmtime::component::Lower)]
        #[component(record)]
        #[derive(Clone)]
        pub struct ReplEnvVar {
          #[component(name = "key")]
          pub key: wasmtime::component::__internal::String,
          #[component(name = "value")]
          pub value: wasmtime::component::__internal::String,
        }
        impl core::fmt::Debug for ReplEnvVar {
          fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ReplEnvVar").field("key", &self.key).field("value", &self.value).finish()
          }
        }
        const _: () = {
          assert!(16 == <ReplEnvVar as wasmtime::component::ComponentType>::SIZE32);
          assert!(4 == <ReplEnvVar as wasmtime::component::ComponentType>::ALIGN32);
        };
        pub struct Guest {
          set_plugins: wasmtime::component::Func,
          set_env: wasmtime::component::Func,
          list_env: wasmtime::component::Func,
          readline: wasmtime::component::Func,
        }
        #[derive(Clone)]
        pub struct GuestIndices {
          set_plugins: wasmtime::component::ComponentExportIndex,
          set_env: wasmtime::component::ComponentExportIndex,
          list_env: wasmtime::component::ComponentExportIndex,
          readline: wasmtime::component::ComponentExportIndex,
        }
        impl GuestIndices {
          
          /// Constructor for [`GuestIndices`] which takes a
          /// [`Component`](wasmtime::component::Component) as input and can be executed
          /// before instantiation.
          ///
          /// This constructor can be used to front-load string lookups to find exports
          /// within a component.
          pub fn new<_T>(
          _instance_pre: &wasmtime::component::InstancePre<_T>,
          ) -> wasmtime::Result<GuestIndices> {
            let instance = _instance_pre.component().get_export_index(None, "repl:api/repl")
            .ok_or_else(|| anyhow::anyhow!("no exported instance named `repl:api/repl`"))?;
            let mut lookup = move |name| {
              _instance_pre.component().get_export_index(Some(&instance), name).ok_or_else(|| {
                anyhow::anyhow!(
                "instance export `repl:api/repl` does \
                not have export `{name}`"
                )
              })
            };
            let _ = &mut lookup;
            let set_plugins = lookup("set-plugins")?;
            let set_env = lookup("set-env")?;
            let list_env = lookup("list-env")?;
            let readline = lookup("readline")?;
            Ok(GuestIndices {
              set_plugins,
              set_env,
              list_env,
              readline,
            })
          }
          
          pub fn load(
          &self,
          mut store: impl wasmtime::AsContextMut,
          instance: &wasmtime::component::Instance,
          ) -> wasmtime::Result<Guest> {
            let _instance = instance;
            let _instance_pre = _instance.instance_pre(&store);
            let _instance_type = _instance_pre.instance_type();
            let mut store = store.as_context_mut();
            let _ = &mut store;
            let set_plugins = *_instance.get_typed_func::<(&[PluginConfig], ), ()>(&mut store, &self.set_plugins)?.func();
            let set_env = *_instance.get_typed_func::<(&ReplEnvVar, ), ()>(&mut store, &self.set_env)?.func();
            let list_env = *_instance.get_typed_func::<(), (wasmtime::component::__internal::Vec<ReplEnvVar>, )>(&mut store, &self.list_env)?.func();
            let readline = *_instance.get_typed_func::<(&str, ), (ReplResult, )>(&mut store, &self.readline)?.func();
            Ok(Guest {
              set_plugins,
              set_env,
              list_env,
              readline,
            })
          }
        }
        impl Guest {
          /// Register the list of available plugins from the host
          pub async fn call_set_plugins<S: wasmtime::AsContextMut>(&self, mut store: S, arg0: &[PluginConfig],) -> wasmtime::Result<()> where <S as wasmtime::AsContext>::Data: Send{
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(&[PluginConfig], ), ()>::new_unchecked(self.set_plugins)
            };
            let () = callee.call_async(store.as_context_mut(), (arg0, )).await?;
            callee.post_return_async(store.as_context_mut()).await?;
            Ok(())
          }
          pub async fn call_set_env<S: wasmtime::AsContextMut>(&self, mut store: S, arg0: &ReplEnvVar,) -> wasmtime::Result<()> where <S as wasmtime::AsContext>::Data: Send{
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(&ReplEnvVar, ), ()>::new_unchecked(self.set_env)
            };
            let () = callee.call_async(store.as_context_mut(), (arg0, )).await?;
            callee.post_return_async(store.as_context_mut()).await?;
            Ok(())
          }
          pub async fn call_list_env<S: wasmtime::AsContextMut>(&self, mut store: S, ) -> wasmtime::Result<wasmtime::component::__internal::Vec<ReplEnvVar>> where <S as wasmtime::AsContext>::Data: Send{
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(), (wasmtime::component::__internal::Vec<ReplEnvVar>, )>::new_unchecked(self.list_env)
            };
            let (ret0,) = callee.call_async(store.as_context_mut(), ()).await?;
            callee.post_return_async(store.as_context_mut()).await?;
            Ok(ret0)
          }
          /// Called by the host when a user submits a command line
          /// The repl is responsible for
          /// - expanding environment variables in the line.
          /// - running the appropriate plugin with the expanded payload.
          /// Behind this readline function, there is a dispatcher that will call the `run` function of the appropriate plugin.
          pub async fn call_readline<S: wasmtime::AsContextMut>(&self, mut store: S, arg0: &str,) -> wasmtime::Result<ReplResult> where <S as wasmtime::AsContext>::Data: Send{
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(&str, ), (ReplResult, )>::new_unchecked(self.readline)
            };
            let (ret0,) = callee.call_async(store.as_context_mut(), (arg0, )).await?;
            callee.post_return_async(store.as_context_mut()).await?;
            Ok(ret0)
          }
        }
        
      }
      
      
      #[allow(clippy::all)]
      pub mod http_client {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::{anyhow, Box};
        
        #[derive(wasmtime::component::ComponentType)]
        #[derive(wasmtime::component::Lift)]
        #[derive(wasmtime::component::Lower)]
        #[component(record)]
        #[derive(Clone)]
        pub struct HttpHeader {
          #[component(name = "name")]
          pub name: wasmtime::component::__internal::String,
          #[component(name = "value")]
          pub value: wasmtime::component::__internal::String,
        }
        impl core::fmt::Debug for HttpHeader {
          fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("HttpHeader").field("name", &self.name).field("value", &self.value).finish()
          }
        }
        const _: () = {
          assert!(16 == <HttpHeader as wasmtime::component::ComponentType>::SIZE32);
          assert!(4 == <HttpHeader as wasmtime::component::ComponentType>::ALIGN32);
        };
        #[derive(wasmtime::component::ComponentType)]
        #[derive(wasmtime::component::Lift)]
        #[derive(wasmtime::component::Lower)]
        #[component(record)]
        #[derive(Clone)]
        pub struct HttpResponse {
          #[component(name = "status")]
          pub status: u16,
          #[component(name = "headers")]
          pub headers: wasmtime::component::__internal::Vec<HttpHeader>,
          #[component(name = "body")]
          pub body: wasmtime::component::__internal::String,
        }
        impl core::fmt::Debug for HttpResponse {
          fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("HttpResponse").field("status", &self.status).field("headers", &self.headers).field("body", &self.body).finish()
          }
        }
        const _: () = {
          assert!(20 == <HttpResponse as wasmtime::component::ComponentType>::SIZE32);
          assert!(4 == <HttpResponse as wasmtime::component::ComponentType>::ALIGN32);
        };
        pub struct Guest {
          get: wasmtime::component::Func,
          post: wasmtime::component::Func,
        }
        #[derive(Clone)]
        pub struct GuestIndices {
          get: wasmtime::component::ComponentExportIndex,
          post: wasmtime::component::ComponentExportIndex,
        }
        impl GuestIndices {
          
          /// Constructor for [`GuestIndices`] which takes a
          /// [`Component`](wasmtime::component::Component) as input and can be executed
          /// before instantiation.
          ///
          /// This constructor can be used to front-load string lookups to find exports
          /// within a component.
          pub fn new<_T>(
          _instance_pre: &wasmtime::component::InstancePre<_T>,
          ) -> wasmtime::Result<GuestIndices> {
            let instance = _instance_pre.component().get_export_index(None, "repl:api/http-client")
            .ok_or_else(|| anyhow::anyhow!("no exported instance named `repl:api/http-client`"))?;
            let mut lookup = move |name| {
              _instance_pre.component().get_export_index(Some(&instance), name).ok_or_else(|| {
                anyhow::anyhow!(
                "instance export `repl:api/http-client` does \
                not have export `{name}`"
                )
              })
            };
            let _ = &mut lookup;
            let get = lookup("get")?;
            let post = lookup("post")?;
            Ok(GuestIndices {
              get,
              post,
            })
          }
          
          pub fn load(
          &self,
          mut store: impl wasmtime::AsContextMut,
          instance: &wasmtime::component::Instance,
          ) -> wasmtime::Result<Guest> {
            let _instance = instance;
            let _instance_pre = _instance.instance_pre(&store);
            let _instance_type = _instance_pre.instance_type();
            let mut store = store.as_context_mut();
            let _ = &mut store;
            let get = *_instance.get_typed_func::<(&str, &[HttpHeader], ), (HttpResponse, )>(&mut store, &self.get)?.func();
            let post = *_instance.get_typed_func::<(&str, &[HttpHeader], &str, ), (HttpResponse, )>(&mut store, &self.post)?.func();
            Ok(Guest {
              get,
              post,
            })
          }
        }
        impl Guest {
          pub async fn call_get<S: wasmtime::AsContextMut>(&self, mut store: S, arg0: &str,arg1: &[HttpHeader],) -> wasmtime::Result<HttpResponse> where <S as wasmtime::AsContext>::Data: Send{
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(&str, &[HttpHeader], ), (HttpResponse, )>::new_unchecked(self.get)
            };
            let (ret0,) = callee.call_async(store.as_context_mut(), (arg0, arg1, )).await?;
            callee.post_return_async(store.as_context_mut()).await?;
            Ok(ret0)
          }
          pub async fn call_post<S: wasmtime::AsContextMut>(&self, mut store: S, arg0: &str,arg1: &[HttpHeader],arg2: &str,) -> wasmtime::Result<HttpResponse> where <S as wasmtime::AsContext>::Data: Send{
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(&str, &[HttpHeader], &str, ), (HttpResponse, )>::new_unchecked(self.post)
            };
            let (ret0,) = callee.call_async(store.as_context_mut(), (arg0, arg1, arg2, )).await?;
            callee.post_return_async(store.as_context_mut()).await?;
            Ok(ret0)
          }
        }
        
      }
      
      
      #[allow(clippy::all)]
      pub mod plugin {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::{anyhow, Box};
        
        pub type ReplResult = super::super::super::super::exports::repl::api::transport::ReplResult;
        const _: () = {
          assert!(28 == <ReplResult as wasmtime::component::ComponentType>::SIZE32);
          assert!(4 == <ReplResult as wasmtime::component::ComponentType>::ALIGN32);
        };
        pub struct Guest {
          name: wasmtime::component::Func,
          man: wasmtime::component::Func,
          arg_count: wasmtime::component::Func,
          run: wasmtime::component::Func,
        }
        #[derive(Clone)]
        pub struct GuestIndices {
          name: wasmtime::component::ComponentExportIndex,
          man: wasmtime::component::ComponentExportIndex,
          arg_count: wasmtime::component::ComponentExportIndex,
          run: wasmtime::component::ComponentExportIndex,
        }
        impl GuestIndices {
          
          /// Constructor for [`GuestIndices`] which takes a
          /// [`Component`](wasmtime::component::Component) as input and can be executed
          /// before instantiation.
          ///
          /// This constructor can be used to front-load string lookups to find exports
          /// within a component.
          pub fn new<_T>(
          _instance_pre: &wasmtime::component::InstancePre<_T>,
          ) -> wasmtime::Result<GuestIndices> {
            let instance = _instance_pre.component().get_export_index(None, "repl:api/plugin")
            .ok_or_else(|| anyhow::anyhow!("no exported instance named `repl:api/plugin`"))?;
            let mut lookup = move |name| {
              _instance_pre.component().get_export_index(Some(&instance), name).ok_or_else(|| {
                anyhow::anyhow!(
                "instance export `repl:api/plugin` does \
                not have export `{name}`"
                )
              })
            };
            let _ = &mut lookup;
            let name = lookup("name")?;
            let man = lookup("man")?;
            let arg_count = lookup("arg-count")?;
            let run = lookup("run")?;
            Ok(GuestIndices {
              name,
              man,
              arg_count,
              run,
            })
          }
          
          pub fn load(
          &self,
          mut store: impl wasmtime::AsContextMut,
          instance: &wasmtime::component::Instance,
          ) -> wasmtime::Result<Guest> {
            let _instance = instance;
            let _instance_pre = _instance.instance_pre(&store);
            let _instance_type = _instance_pre.instance_type();
            let mut store = store.as_context_mut();
            let _ = &mut store;
            let name = *_instance.get_typed_func::<(), (wasmtime::component::__internal::String, )>(&mut store, &self.name)?.func();
            let man = *_instance.get_typed_func::<(), (wasmtime::component::__internal::String, )>(&mut store, &self.man)?.func();
            let arg_count = *_instance.get_typed_func::<(), (Option<i8>, )>(&mut store, &self.arg_count)?.func();
            let run = *_instance.get_typed_func::<(&str, ), (ReplResult, )>(&mut store, &self.run)?.func();
            Ok(Guest {
              name,
              man,
              arg_count,
              run,
            })
          }
        }
        impl Guest {
          pub async fn call_name<S: wasmtime::AsContextMut>(&self, mut store: S, ) -> wasmtime::Result<wasmtime::component::__internal::String> where <S as wasmtime::AsContext>::Data: Send{
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(), (wasmtime::component::__internal::String, )>::new_unchecked(self.name)
            };
            let (ret0,) = callee.call_async(store.as_context_mut(), ()).await?;
            callee.post_return_async(store.as_context_mut()).await?;
            Ok(ret0)
          }
          pub async fn call_man<S: wasmtime::AsContextMut>(&self, mut store: S, ) -> wasmtime::Result<wasmtime::component::__internal::String> where <S as wasmtime::AsContext>::Data: Send{
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(), (wasmtime::component::__internal::String, )>::new_unchecked(self.man)
            };
            let (ret0,) = callee.call_async(store.as_context_mut(), ()).await?;
            callee.post_return_async(store.as_context_mut()).await?;
            Ok(ret0)
          }
          pub async fn call_arg_count<S: wasmtime::AsContextMut>(&self, mut store: S, ) -> wasmtime::Result<Option<i8>> where <S as wasmtime::AsContext>::Data: Send{
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(), (Option<i8>, )>::new_unchecked(self.arg_count)
            };
            let (ret0,) = callee.call_async(store.as_context_mut(), ()).await?;
            callee.post_return_async(store.as_context_mut()).await?;
            Ok(ret0)
          }
          pub async fn call_run<S: wasmtime::AsContextMut>(&self, mut store: S, arg0: &str,) -> wasmtime::Result<ReplResult> where <S as wasmtime::AsContext>::Data: Send{
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(&str, ), (ReplResult, )>::new_unchecked(self.run)
            };
            let (ret0,) = callee.call_async(store.as_context_mut(), (arg0, )).await?;
            callee.post_return_async(store.as_context_mut()).await?;
            Ok(ret0)
          }
        }
        
      }
      
      
      #[allow(clippy::all)]
      pub mod plugin_runner {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::{anyhow, Box};
        
        pub type ReplResult = super::super::super::super::exports::repl::api::transport::ReplResult;
        const _: () = {
          assert!(28 == <ReplResult as wasmtime::component::ComponentType>::SIZE32);
          assert!(4 == <ReplResult as wasmtime::component::ComponentType>::ALIGN32);
        };
        pub struct Guest {
          exec: wasmtime::component::Func,
        }
        #[derive(Clone)]
        pub struct GuestIndices {
          exec: wasmtime::component::ComponentExportIndex,
        }
        impl GuestIndices {
          
          /// Constructor for [`GuestIndices`] which takes a
          /// [`Component`](wasmtime::component::Component) as input and can be executed
          /// before instantiation.
          ///
          /// This constructor can be used to front-load string lookups to find exports
          /// within a component.
          pub fn new<_T>(
          _instance_pre: &wasmtime::component::InstancePre<_T>,
          ) -> wasmtime::Result<GuestIndices> {
            let instance = _instance_pre.component().get_export_index(None, "repl:api/plugin-runner")
            .ok_or_else(|| anyhow::anyhow!("no exported instance named `repl:api/plugin-runner`"))?;
            let mut lookup = move |name| {
              _instance_pre.component().get_export_index(Some(&instance), name).ok_or_else(|| {
                anyhow::anyhow!(
                "instance export `repl:api/plugin-runner` does \
                not have export `{name}`"
                )
              })
            };
            let _ = &mut lookup;
            let exec = lookup("exec")?;
            Ok(GuestIndices {
              exec,
            })
          }
          
          pub fn load(
          &self,
          mut store: impl wasmtime::AsContextMut,
          instance: &wasmtime::component::Instance,
          ) -> wasmtime::Result<Guest> {
            let _instance = instance;
            let _instance_pre = _instance.instance_pre(&store);
            let _instance_type = _instance_pre.instance_type();
            let mut store = store.as_context_mut();
            let _ = &mut store;
            let exec = *_instance.get_typed_func::<(&str, &str, ), (ReplResult, )>(&mut store, &self.exec)?.func();
            Ok(Guest {
              exec,
            })
          }
        }
        impl Guest {
          /// The REPL calls this to dispatch a plugin execution.
          pub async fn call_exec<S: wasmtime::AsContextMut>(&self, mut store: S, arg0: &str,arg1: &str,) -> wasmtime::Result<ReplResult> where <S as wasmtime::AsContext>::Data: Send{
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(&str, &str, ), (ReplResult, )>::new_unchecked(self.exec)
            };
            let (ret0,) = callee.call_async(store.as_context_mut(), (arg0, arg1, )).await?;
            callee.post_return_async(store.as_context_mut()).await?;
            Ok(ret0)
          }
        }
        
      }
      
    }
  }
}
