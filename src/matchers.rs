use crate::element::ResolvedElement;
pub use test_that::matchers::{containers::*, *};
use test_that::{
    description::Description,
    matcher::{Describable, MatcherResult},
    prelude::Matcher,
};

/// Returns a [Matcher] which matches an element whose inner HTML is matched by the [Matcher]
/// `inner`.
pub fn inner_html(inner: impl Matcher<String>) -> impl for<'vdom> Matcher<ResolvedElement<'vdom>> {
    struct InnerHtmlMatcher<InnerMatcher>(InnerMatcher);

    impl<'vdom, InnerMatcher: Matcher<String>> Matcher<ResolvedElement<'vdom>>
        for InnerHtmlMatcher<InnerMatcher>
    {
        fn matches(&self, actual: &ResolvedElement<'vdom>) -> MatcherResult {
            let inner_html = actual.inner_html();
            self.0.matches(&inner_html)
        }
    }

    impl<InnerMatcher: Describable> Describable for InnerHtmlMatcher<InnerMatcher> {
        fn describe(&self, matcher_result: MatcherResult) -> test_that::description::Description {
            Description::new()
                .text("Has inner HTML matching")
                .nested(self.0.describe(matcher_result))
        }
    }

    InnerHtmlMatcher(inner)
}

/// Returns a [Matcher] which matches an element whose element `name` is matched by the [Matcher]
/// `inner`.
///
/// Because the attribute might not exist on the element, the inner matcher must match an
/// `Option<String>`. When asserting that the attribute exists, use `some()`:
///
/// ```
/// # use dioxus::prelude::*;
/// # use dioxus_test::{by_testid, matchers::{attribute, eq, some}, render};
/// #[component]
/// fn TestComponent() -> Element {
///     rsx! {
///         div {
///             "data-testid": "item",
///             "my-attribute": "A value",
///         }
///     }
/// }
/// let tester = render(TestComponent).build();
/// tester.query(by_testid("item"))
///     .expect(attribute("my-attribute", some(eq("A value"))))
///     .immediately()
/// # .unwrap();
/// ```
///
/// When assertings its absence, use `none()`:
///
/// ```
/// # use dioxus::prelude::*;
/// # use dioxus_test::{by_testid, matchers::{attribute, none}, render};
/// #[component]
/// fn TestComponent() -> Element {
///     rsx! {
///         div {
///             "data-testid": "item",
///         }
///     }
/// }
/// let tester = render(TestComponent).build();
/// tester.query(by_testid("item"))
///     .expect(attribute("my-attribute", none()))
///     .immediately()
/// # .unwrap();
/// ```
pub fn attribute(
    name: impl AsRef<str>,
    inner: impl Matcher<Option<String>>,
) -> impl for<'vdom> Matcher<ResolvedElement<'vdom>> {
    struct AttributeMatcher<InnerMatcher>(String, InnerMatcher);

    impl<'vdom, InnerMatcher: Matcher<Option<String>>> Matcher<ResolvedElement<'vdom>>
        for AttributeMatcher<InnerMatcher>
    {
        fn matches(&self, actual: &ResolvedElement<'vdom>) -> MatcherResult {
            let attribute_content = actual.attribute(&self.0);
            self.1.matches(&attribute_content)
        }
    }

    impl<InnerMatcher: Describable> Describable for AttributeMatcher<InnerMatcher> {
        fn describe(&self, matcher_result: MatcherResult) -> test_that::description::Description {
            Description::new()
                .text("Has inner HTML matching")
                .nested(self.1.describe(matcher_result))
        }
    }

    AttributeMatcher(name.as_ref().to_string(), inner)
}
