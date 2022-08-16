use std::marker::PhantomData;
use std::fmt::Display;

use serde::Serialize;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use crate::history::{BrowserHistory, History};
use crate::scope_ext::RouterScopeExt;
use crate::Routable;

/// Props for [`Link`]
#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps<R, Q = (), S = String>
where
    R: Routable,
    Q: Clone + PartialEq + Serialize,
    S: Display + Clone + PartialEq + Serialize,
{
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub classes: Classes,
    /// Route that will be pushed when the anchor is clicked.
    pub to: R,
    #[prop_or_default]
    pub fragment: Option<S>,
    /// Route query data
    #[prop_or_default]
    pub query: Option<Q>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

/// A wrapper around `<a>` tag to be used with [`Router`](crate::Router)
pub struct Link<R, Q = (), S = String>
where
    R: Routable + 'static,
    Q: Clone + PartialEq + Serialize + 'static,
    S: Display + Clone + PartialEq + Serialize + 'static,
{
    _route: PhantomData<R>,
    _query: PhantomData<Q>,
    _fragment: PhantomData<S>,
}

pub enum Msg {
    OnClick,
}

impl<R, Q, S> Component for Link<R, Q, S>
where
    R: Routable + 'static,
    Q: Clone + PartialEq + Serialize + 'static,
    S: Display + Clone + PartialEq + Serialize + 'static,
{
    type Message = Msg;
    type Properties = LinkProps<R, Q, S>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _route: PhantomData,
            _query: PhantomData,
            _fragment: PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnClick => {
                let LinkProps { to, fragment, query, .. } = ctx.props();
                let history = ctx.link().history().expect_throw("failed to read history");
                match query {
                    None => {
                        match fragment {
                            None => {
                                history.push(to.clone());
                            }
                            Some(frag) => {
                                history
                                    .push_with_fragment(to.clone(), &frag)
                                    .expect("failed push history with fragment");
                            }
                        } 
                    }
                    Some(data) => {
                        match fragment {
                            None => {
                                history
                                    .push_with_query(to.clone(), &data)
                                    .expect_throw("failed push history with query");
                            }
                            Some(frag) => {
                                history
                                    .push_with_query_and_fragment(to.clone(), &data, &frag)
                                    .expect_throw("failed push history with query and fragment");
                            }
                        } 
                    }
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let LinkProps {
            classes,
            to,
            fragment,
            children,
            disabled,
            ..
        } = ctx.props().clone();
        let onclick = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::OnClick
        });

        match fragment {
            None => {
                let href: AttrValue = BrowserHistory::route_to_url(to).into();
                html! {
                    <a class={classes}
                        {href}
                        {onclick}
                        {disabled}
                    >
                        { children }
                    </a>
                }
            }
            Some(frag) => {
                let h: AttrValue = BrowserHistory::route_to_url(to).into();
                let href : String = format!("{}#{}", h, frag);
                html! {
                    <a class={classes}
                        {href}
                        {onclick}
                        {disabled}
                    >
                        { children }
                    </a>
                }
            }
        } 
        
    }
}
