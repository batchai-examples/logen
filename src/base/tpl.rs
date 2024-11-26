#[cfg(test)]
mod tests {
    use super::*;
    use tera::Context;

    /// Test cases for the Engine and Template structs.
    /// This includes tests for adding templates, rendering templates,
    /// setting and getting values in the context, and handling errors.
    #[test]
    fn test_engine_add_template_success() {
        // Step 1: Create a new Engine instance
        let mut engine = Engine::new();
        // Step 2: Add a template successfully
        let result = engine.add_template("test_template", "Hello, {{ name }}!");
        // Step 3: Assert that the result is Ok
        assert!(result.is_ok());
    }

    #[test]
    fn test_engine_render_success() {
        // Step 1: Create a new Engine instance and add a template
        let mut engine = Engine::new();
        engine.add_template("greeting", "Hello, {{ name }}!").unwrap();
        // Step 2: Create a context with data
        let mut context = Context::new();
        context.insert("name", "Alice");
        // Step 3: Render the template
        let result = engine.render("greeting", &context);
        // Step 4: Assert that the rendered result is as expected
        assert_eq!(result.unwrap(), "Hello, Alice!");
    }

    #[test]
    fn test_engine_render_template_not_found() {
        // Step 1: Create a new Engine instance
        let engine = Engine::new();
        // Step 2: Create a context with data
        let context = Context::new();
        // Step 3: Attempt to render a non-existent template
        let result = engine.render("non_existent_template", &context);
        // Step 4: Assert that the result is an error
        assert!(result.is_err());
    }

    #[test]
    fn test_template_set_and_get() {
        // Step 1: Create a new Engine instance
        let engine = Engine::new();
        // Step 2: Create a new Template instance
        let mut template = Template::new(&engine);
        // Step 3: Set a value in the template context
        template.set("name", "Bob");
        // Step 4: Get the value from the template context
        let value = template.get("name").unwrap();
        // Step 5: Assert that the value is as expected
        assert_eq!(value.as_str().unwrap(), "Bob");
    }

    #[test]
    fn test_template_render_with_no_data() {
        // Step 1: Create a new Engine instance and add a template
        let mut engine = Engine::new();
        engine.add_template("empty_template", "No data here.").unwrap();
        // Step 2: Create a new Template instance
        let template = Template::new(&engine);
        // Step 3: Render the template without any data
        let result = template.render("empty_template");
        // Step 4: Assert that the rendered result is as expected
        assert_eq!(result.unwrap(), "No data here.");
    }

    #[test]
    fn test_template_render_with_missing_key() {
        // Step 1: Create a new Engine instance and add a template
        let mut engine = Engine::new();
        engine.add_template("greeting", "Hello, {{ name }}!").unwrap();
        // Step 2: Create a context without the 'name' key
        let context = Context::new();
        // Step 3: Attempt to render the template
        let result = engine.render("greeting", &context);
        // Step 4: Assert that the result is an error
        assert!(result.is_err());
    }
}
