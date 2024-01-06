//! Tests that don't make use of external websites.
use std::time::Duration;

use serial_test::serial;
use thirtyfour::{prelude::*, SameSite};

use crate::common::sample_page_url;

mod common;

async fn resolve_execute_async_value(c: WebDriver) -> Result<(), WebDriverError> {
    let url = sample_page_url();
    c.goto(&url).await?;

    let count: u64 = c
        .execute_async("setTimeout(() => arguments[1](arguments[0] + 1))", vec![1_u32.into()])
        .await?
        .convert()
        .expect("should be integer variant");

    assert_eq!(2, count);

    let count: u64 = c
        .execute_async("setTimeout(() => arguments[0](2))", vec![])
        .await?
        .convert()
        .expect("should be integer variant");

    assert_eq!(2, count);

    Ok(())
}

async fn status_firefox(c: WebDriver) -> Result<(), WebDriverError> {
    // Geckodriver only supports a single session, and since we're already in a
    // session, it should return `false` here.
    assert!(!c.status().await?.ready);
    Ok(())
}

async fn status_chrome(c: WebDriver) -> Result<(), WebDriverError> {
    // Chromedriver supports multiple sessions, so it should always return
    // `true` here.
    assert!(c.status().await?.ready);
    Ok(())
}

async fn timeouts(c: WebDriver) -> Result<(), WebDriverError> {
    let new_timeouts = TimeoutConfiguration::new(
        Some(Duration::from_secs(60)),
        Some(Duration::from_secs(60)),
        Some(Duration::from_secs(30)),
    );
    c.update_timeouts(new_timeouts.clone()).await?;

    let got_timeouts = c.get_timeouts().await?;
    assert_eq!(got_timeouts, new_timeouts);

    // Ensure partial update also works.
    let update_timeouts = TimeoutConfiguration::new(None, None, Some(Duration::from_secs(0)));
    c.update_timeouts(update_timeouts.clone()).await?;

    let got_timeouts = c.get_timeouts().await?;
    assert_eq!(
        got_timeouts,
        TimeoutConfiguration::new(
            new_timeouts.script(),
            new_timeouts.page_load(),
            update_timeouts.implicit()
        )
    );

    c.set_implicit_wait_timeout(Duration::from_secs(10)).await?;

    let got_timeouts = c.get_timeouts().await?;
    assert_eq!(
        got_timeouts,
        TimeoutConfiguration::new(
            new_timeouts.script(),
            new_timeouts.page_load(),
            Some(Duration::from_secs(10))
        )
    );

    c.set_page_load_timeout(Duration::from_secs(10)).await?;

    let got_timeouts = c.get_timeouts().await?;
    assert_eq!(
        got_timeouts,
        TimeoutConfiguration::new(
            new_timeouts.script(),
            Some(Duration::from_secs(10)),
            Some(Duration::from_secs(10))
        )
    );

    c.set_script_timeout(Duration::from_secs(10)).await?;

    let got_timeouts = c.get_timeouts().await?;
    assert_eq!(
        got_timeouts,
        TimeoutConfiguration::new(
            Some(Duration::from_secs(10)),
            Some(Duration::from_secs(10)),
            Some(Duration::from_secs(10))
        )
    );

    Ok(())
}

// Verifies that basic cookie handling works
async fn handle_cookies_test(c: WebDriver) -> Result<(), WebDriverError> {
    c.goto("https://www.wikipedia.org/").await?;

    let cookies = c.get_all_cookies().await?;
    assert!(!cookies.is_empty());

    // Add a new cookie.
    let mut cookie = Cookie::new("cookietest", "thirtyfour");
    cookie.set_domain(".wikipedia.org");
    cookie.set_path("/");
    cookie.set_same_site(SameSite::Lax);
    c.add_cookie(cookie.clone()).await?;

    // Verify that the cookie exists.
    assert_eq!(c.get_named_cookie(&cookie.name).await?.value, cookie.value);

    // Delete the cookie and make sure it's gone
    c.delete_cookie(&cookie.name).await?;
    assert!(c.get_named_cookie(&cookie.name).await.is_err());

    c.delete_all_cookies().await?;
    let cookies = c.get_all_cookies().await?;
    assert!(dbg!(cookies).is_empty());

    Ok(())
}

mod tests {
    use super::*;

    local_tester!(resolve_execute_async_value, timeouts, handle_cookies_test);

    #[test]
    #[serial]
    fn test_status_firefox() {
        test_inner!(status_firefox, "firefix");
    }

    #[test]
    fn test_status_chrome() {
        test_inner!(status_chrome, "chrome");
    }
}
