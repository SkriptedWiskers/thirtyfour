//! Element tests
use crate::common::sample_page_url;
use serial_test::serial;
use thirtyfour::prelude::*;

mod common;

async fn element_is(c: WebDriver) -> Result<(), WebDriverError> {
    let sample_url = sample_page_url();
    c.goto(&sample_url).await?;
    let elem = c.find(By::Id("checkbox-option-1")).await?;
    assert!(elem.is_enabled().await?);
    assert!(elem.is_displayed().await?);
    assert!(!elem.is_selected().await?);
    assert!(elem.is_present().await?);
    assert!(elem.is_clickable().await?);
    elem.click().await?;
    let elem = c.find(By::Id("checkbox-option-1")).await?;
    assert!(elem.is_selected().await?);

    assert!(!c.find(By::Id("checkbox-disabled")).await?.is_enabled().await?);
    assert!(!c.find(By::Id("checkbox-hidden")).await?.is_displayed().await?);
    Ok(())
}

async fn element_attr(c: WebDriver) -> Result<(), WebDriverError> {
    let sample_url = sample_page_url();
    c.goto(&sample_url).await?;
    let elem = c.find(By::Id("checkbox-option-1")).await?;
    assert_eq!(elem.attr("id").await?.unwrap(), "checkbox-option-1");
    assert_eq!(elem.id().await?.unwrap(), "checkbox-option-1");
    assert!(elem.attr("invalid-attribute").await?.is_none());
    Ok(())
}

async fn element_prop(c: WebDriver) -> Result<(), WebDriverError> {
    let sample_url = sample_page_url();
    c.goto(&sample_url).await?;
    let elem = c.find(By::Id("checkbox-option-1")).await?;
    assert_eq!(elem.prop("id").await?.unwrap(), "checkbox-option-1");
    assert_eq!(elem.prop("checked").await?.unwrap(), "false");
    assert!(elem.attr("invalid-property").await?.is_none());
    Ok(())
}

async fn element_css_value(c: WebDriver) -> Result<(), WebDriverError> {
    let sample_url = sample_page_url();
    c.goto(&sample_url).await?;
    let elem = c.find(By::Id("checkbox-hidden")).await?;
    assert_eq!(elem.css_value("display").await?, "none");
    assert_eq!(elem.css_value("invalid-css-value").await?, "");
    Ok(())
}

async fn element_tag_name(c: WebDriver) -> Result<(), WebDriverError> {
    let sample_url = sample_page_url();
    c.goto(&sample_url).await?;
    let elem = c.find(By::Id("checkbox-option-1")).await?;
    let tag_name = elem.tag_name().await?;
    assert!(tag_name.eq_ignore_ascii_case("input"), "{} != input", tag_name);
    Ok(())
}

async fn element_class_name(c: WebDriver) -> Result<(), WebDriverError> {
    let sample_url = sample_page_url();
    c.goto(&sample_url).await?;
    let elem = c.find(By::ClassName("vertical")).await?;
    let class_name = elem.class_name().await?.unwrap();
    assert!(class_name.eq_ignore_ascii_case("vertical"), "{} != vertical", class_name);
    Ok(())
}

async fn element_text(c: WebDriver) -> Result<(), WebDriverError> {
    let sample_url = sample_page_url();
    c.goto(&sample_url).await?;
    let elem = c.find(By::Id("button-copy")).await?;
    assert_eq!(elem.text().await?, "Copy");
    Ok(())
}

async fn element_rect(c: WebDriver) -> Result<(), WebDriverError> {
    let sample_url = sample_page_url();
    c.goto(&sample_url).await?;
    let elem = c.find(By::Id("button-alert")).await?;
    let rect = elem.rect().await?;
    // Rather than try to verify the exact position and size of the element,
    // let's just verify that the returned values deserialized ok and
    // are within the expected range.
    assert!(rect.x > 0.0);
    assert!(rect.x < 100.0);
    assert!(rect.y > 0.0);
    assert!(rect.y < 1000.0);
    assert!(rect.width > 0.0);
    assert!(rect.width < 200.0);
    assert!(rect.height > 0.0);
    assert!(rect.height < 200.0);
    Ok(())
}

async fn element_send_keys(c: WebDriver) -> Result<(), WebDriverError> {
    let sample_url = sample_page_url();
    c.goto(&sample_url).await?;
    let elem = c.find(By::Id("text-input")).await?;
    assert_eq!(elem.prop("value").await?.unwrap(), "");
    assert_eq!(elem.value().await?.unwrap(), "");
    elem.send_keys("thirtyfour").await?;
    assert_eq!(elem.prop("value").await?.unwrap(), "thirtyfour");
    assert_eq!(elem.value().await?.unwrap(), "thirtyfour");
    let select_all = if cfg!(target_os = "macos") {
        Key::Command + "a"
    } else {
        Key::Control + "a"
    };
    let backspace = Key::Backspace;
    elem.send_keys(select_all).await?;
    elem.send_keys(backspace).await?;
    assert_eq!(elem.prop("value").await?.unwrap(), "");
    assert_eq!(elem.value().await?.unwrap(), "");

    Ok(())
}

async fn element_clear(c: WebDriver) -> Result<(), WebDriverError> {
    let sample_url = sample_page_url();
    c.goto(&sample_url).await?;
    let elem = c.find(By::Id("text-input")).await?;
    assert_eq!(elem.value().await?.unwrap(), "");
    elem.send_keys("thirtyfour").await?;
    assert_eq!(elem.value().await?.unwrap(), "thirtyfour");
    elem.clear().await?;
    assert_eq!(elem.value().await?.unwrap(), "");
    Ok(())
}

async fn serialize_element(c: WebDriver) -> Result<(), WebDriverError> {
    let url = sample_page_url();
    c.goto(&url).await?;
    let elem = c.find(By::Css("#other_page_id")).await?;

    // Check that webdriver understands it
    c.execute("arguments[0].scrollIntoView(true);", vec![elem.to_json()?]).await?;

    // This does the same thing.
    elem.scroll_into_view().await?;

    // Check that it fails with an invalid serialization (from a previous run of the test)
    let json = r#"{"element-6066-11e4-a52e-4f735466cecf":"fbe5004d-ec8b-4c7b-ad08-642c55d84505"}"#;

    c.execute("arguments[0].scrollIntoView(true);", vec![serde_json::from_str(json)?])
        .await
        .expect_err("Failure expected with an invalid ID");

    // You can easily deserialize elements too.
    let ret = c.execute(r#"return document.getElementById("select1");"#, vec![]).await?;
    let elem = ret.element()?;
    assert_eq!(elem.tag_name().await?, "select");

    c.close_window().await
}

async fn element_screenshot(c: WebDriver) -> Result<(), WebDriverError> {
    let url = sample_page_url();
    c.goto(&url).await?;

    let elem = c.find(By::Id("select1")).await?;

    let screenshot_data = elem.screenshot_as_png().await?;
    assert!(!screenshot_data.is_empty(), "screenshot data is empty");

    Ok(())
}

async fn element_focus(c: WebDriver) -> Result<(), WebDriverError> {
    let url = sample_page_url();
    c.goto(&url).await?;
    let elem = c.find(By::Id("text-input")).await?;
    elem.focus().await?;
    let active_elem = c.active_element().await?;
    assert_eq!(active_elem.id().await?.unwrap(), "text-input");
    Ok(())
}

async fn element_html(c: WebDriver) -> Result<(), WebDriverError> {
    let url = sample_page_url();
    c.goto(&url).await?;
    let elem = c.find(By::Id("button-copy")).await?;
    assert_eq!(elem.inner_html().await?, "Copy");

    let elem = c.find(By::Id("text-output")).await?;
    assert_eq!(elem.outer_html().await?, r#"<div id="text-output"></div>"#);
    Ok(())
}

async fn element_get_parent(c: WebDriver) -> Result<(), WebDriverError> {
    let url = sample_page_url();
    c.goto(&url).await?;
    let elem = c.find(By::Id("other_page_id")).await?;
    let parent = elem.parent().await?;
    assert_eq!(parent.id().await?.unwrap(), "navigation");
    Ok(())
}

mod tests {
    use super::*;

    local_tester!(
        element_is,
        element_attr,
        element_prop,
        element_css_value,
        element_tag_name,
        element_class_name,
        element_text,
        element_rect,
        element_send_keys,
        element_clear,
        serialize_element,
        element_screenshot,
        element_focus,
        element_html,
        element_get_parent
    );
}
