//! Exercise 10: Dynamic Dispatch - Understand vtables and dynamic dispatch
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand dynamic dispatch mechanism
//! - Compare static vs dynamic dispatch
//! - Use dyn Trait references

pub trait Renderer {
    fn render(&self) -> String;
    fn get_type(&self) -> &'static str;
}

pub struct HtmlRenderer {
    pub content: String,
}

impl Renderer for HtmlRenderer {
    fn render(&self) -> String {
        format!("<html>{}</html>", self.content)
    }
    
    fn get_type(&self) -> &'static str {
        "HTML"
    }
}

pub struct JsonRenderer {
    pub content: String,
}

impl Renderer for JsonRenderer {
    fn render(&self) -> String {
        format!("{{\"content\":\"{}\"}}", self.content)
    }
    
    fn get_type(&self) -> &'static str {
        "JSON"
    }
}

pub struct XmlRenderer {
    pub content: String,
}

impl Renderer for XmlRenderer {
    fn render(&self) -> String {
        format!("<xml>{}</xml>", self.content)
    }
    
    fn get_type(&self) -> &'static str {
        "XML"
    }
}

/// Dynamic dispatch using trait object reference
pub fn render_dynamic(renderer: &dyn Renderer) -> String {
    format!("[{}] {}", renderer.get_type(), renderer.render())
}

/// Static dispatch using generic
pub fn render_static<T: Renderer>(renderer: &T) -> String {
    format!("[{}] {}", renderer.get_type(), renderer.render())
}

/// Process multiple renderers dynamically
pub fn render_all(renderers: &[&dyn Renderer]) -> Vec<String> {
    renderers.iter().map(|r| render_dynamic(*r)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_renderer() {
        let html = HtmlRenderer { content: "Hello".to_string() };
        assert_eq!(html.render(), "<html>Hello</html>");
        assert_eq!(html.get_type(), "HTML");
    }

    #[test]
    fn test_json_renderer() {
        let json = JsonRenderer { content: "World".to_string() };
        assert_eq!(json.render(), "{\"content\":\"World\"}");
        assert_eq!(json.get_type(), "JSON");
    }

    #[test]
    fn test_dynamic_dispatch() {
        let html = HtmlRenderer { content: "Test".to_string() };
        let result = render_dynamic(&html);
        assert!(result.contains("[HTML]"));
        assert!(result.contains("<html>"));
    }

    #[test]
    fn test_static_dispatch() {
        let json = JsonRenderer { content: "Static".to_string() };
        let result = render_static(&json);
        assert!(result.contains("[JSON]"));
        assert!(result.contains("Static"));
    }

    #[test]
    fn test_render_all_heterogeneous() {
        let html = HtmlRenderer { content: "A".to_string() };
        let json = JsonRenderer { content: "B".to_string() };
        let xml = XmlRenderer { content: "C".to_string() };
        
        let renderers: Vec<&dyn Renderer> = vec![&html, &json, &xml];
        let results = render_all(&renderers);
        
        assert_eq!(results.len(), 3);
        assert!(results[0].contains("HTML"));
        assert!(results[1].contains("JSON"));
        assert!(results[2].contains("XML"));
    }

    #[test]
    fn test_trait_object_reference() {
        let html = HtmlRenderer { content: "Ref".to_string() };
        let renderer: &dyn Renderer = &html;
        
        assert_eq!(renderer.get_type(), "HTML");
        assert!(renderer.render().contains("Ref"));
    }
}
