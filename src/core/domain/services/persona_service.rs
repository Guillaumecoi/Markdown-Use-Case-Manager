use crate::core::domain::entities::Persona;

/// Domain service for persona-related business logic
pub struct PersonaService;

impl PersonaService {
    /// Validate that a persona has all required fields
    pub fn validate_persona(persona: &Persona) -> Result<(), String> {
        if persona.id.trim().is_empty() {
            return Err("Persona ID cannot be empty".to_string());
        }
        
        if persona.name.trim().is_empty() {
            return Err("Persona name cannot be empty".to_string());
        }
        
        if persona.description.trim().is_empty() {
            return Err("Persona description cannot be empty".to_string());
        }
        
        if persona.goal.trim().is_empty() {
            return Err("Persona goal cannot be empty".to_string());
        }
        
        if let Some(level) = persona.tech_level {
            if level < 1 || level > 5 {
                return Err("Persona tech level must be between 1 and 5".to_string());
            }
        }
        
        Ok(())
    }
    
    /// Suggest usage frequency based on tech level and context
    pub fn suggest_usage_frequency(persona: &Persona) -> String {
        // High tech level users tend to use systems more frequently
        match persona.tech_level.unwrap_or(3) {
            5 => "daily".to_string(),
            4 => "daily".to_string(),
            3 => "weekly".to_string(),
            2 => "monthly".to_string(),
            _ => "rarely".to_string(),
        }
    }
    
    /// Generate a short summary of the persona
    pub fn get_persona_summary(persona: &Persona) -> String {
        format!(
            "{} ({}): {} - Tech Level: {}/5{}",
            persona.name,
            persona.id,
            persona.description,
            persona.tech_level.unwrap_or(3),
            persona.usage_frequency
                .as_ref()
                .map(|f| format!(", Uses: {}", f))
                .unwrap_or_default()
        )
    }
    
    /// Check if a persona ID follows the expected pattern
    pub fn is_valid_persona_id(id: &str) -> bool {
        // Format: PERSONA-XXX or similar
        !id.is_empty() && id.len() >= 3
    }
    
    /// Generate a persona ID from a name
    pub fn generate_persona_id(name: &str) -> String {
        let clean_name = name
            .to_uppercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>();
        
        let words: Vec<&str> = clean_name.split_whitespace().collect();
        
        if words.is_empty() {
            return "PERSONA-001".to_string();
        }
        
        if words.len() == 1 {
            format!("PERSONA-{}", words[0])
        } else {
            let initials: String = words.iter()
                .filter_map(|w| w.chars().next())
                .collect();
            format!("PERSONA-{}", initials)
        }
    }
    
    /// Suggest context based on tech level
    pub fn suggest_context_for_tech_level(tech_level: u8) -> String {
        match tech_level {
            5 => "Expert user with deep technical knowledge and frequent system usage".to_string(),
            4 => "Advanced user comfortable with technical concepts and regular usage".to_string(),
            3 => "Intermediate user with basic technical understanding".to_string(),
            2 => "Novice user with limited technical knowledge".to_string(),
            _ => "Beginner user requiring guidance and simple interfaces".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_persona() -> Persona {
        Persona::new(
            "PERSONA-001".to_string(),
            "Test User".to_string(),
            "A test persona".to_string(),
            "Complete tasks efficiently".to_string(),
        )
    }

    #[test]
    fn test_validate_persona_valid() {
        let persona = create_test_persona();
        assert!(PersonaService::validate_persona(&persona).is_ok());
    }

    #[test]
    fn test_validate_persona_empty_id() {
        let persona = Persona::new(
            "".to_string(),
            "Test User".to_string(),
            "A test persona".to_string(),
            "Complete tasks".to_string(),
        );
        assert!(PersonaService::validate_persona(&persona).is_err());
    }

    #[test]
    fn test_validate_persona_empty_name() {
        let persona = Persona::new(
            "PERSONA-001".to_string(),
            "".to_string(),
            "A test persona".to_string(),
            "Complete tasks".to_string(),
        );
        assert!(PersonaService::validate_persona(&persona).is_err());
    }

    #[test]
    fn test_validate_persona_empty_description() {
        let persona = Persona::new(
            "PERSONA-001".to_string(),
            "Test User".to_string(),
            "".to_string(),
            "Complete tasks".to_string(),
        );
        assert!(PersonaService::validate_persona(&persona).is_err());
    }

    #[test]
    fn test_validate_persona_empty_goal() {
        let persona = Persona::new(
            "PERSONA-001".to_string(),
            "Test User".to_string(),
            "A test persona".to_string(),
            "".to_string(),
        );
        assert!(PersonaService::validate_persona(&persona).is_err());
    }

    #[test]
    fn test_validate_persona_invalid_tech_level() {
        let persona = create_test_persona().with_tech_level(0);
        assert!(PersonaService::validate_persona(&persona).is_err());
        
        // Note: with_tech_level(6) gets capped to 5 which is valid
        // So we need to directly set invalid tech_level for this test
        let mut persona = create_test_persona();
        persona.tech_level = Some(6);
        assert!(PersonaService::validate_persona(&persona).is_err());
    }

    #[test]
    fn test_suggest_usage_frequency() {
        let persona = create_test_persona().with_tech_level(5);
        assert_eq!(PersonaService::suggest_usage_frequency(&persona), "daily");
        
        let persona = create_test_persona().with_tech_level(4);
        assert_eq!(PersonaService::suggest_usage_frequency(&persona), "daily");
        
        let persona = create_test_persona().with_tech_level(3);
        assert_eq!(PersonaService::suggest_usage_frequency(&persona), "weekly");
        
        let persona = create_test_persona().with_tech_level(2);
        assert_eq!(PersonaService::suggest_usage_frequency(&persona), "monthly");
        
        let persona = create_test_persona().with_tech_level(1);
        assert_eq!(PersonaService::suggest_usage_frequency(&persona), "rarely");
    }

    #[test]
    fn test_get_persona_summary() {
        let persona = create_test_persona()
            .with_usage_frequency("weekly".to_string());
        
        let summary = PersonaService::get_persona_summary(&persona);
        assert!(summary.contains("Test User"));
        assert!(summary.contains("PERSONA-001"));
        assert!(summary.contains("A test persona"));
        assert!(summary.contains("Tech Level: 3/5"));
        assert!(summary.contains("Uses: weekly"));
    }

    #[test]
    fn test_is_valid_persona_id() {
        assert!(PersonaService::is_valid_persona_id("PERSONA-001"));
        assert!(PersonaService::is_valid_persona_id("ABC"));
        assert!(!PersonaService::is_valid_persona_id(""));
        assert!(!PersonaService::is_valid_persona_id("AB"));
    }

    #[test]
    fn test_generate_persona_id() {
        assert_eq!(
            PersonaService::generate_persona_id("John Doe"),
            "PERSONA-JD"
        );
        assert_eq!(
            PersonaService::generate_persona_id("Admin"),
            "PERSONA-ADMIN"
        );
        assert_eq!(
            PersonaService::generate_persona_id("System Administrator"),
            "PERSONA-SA"
        );
        assert_eq!(
            PersonaService::generate_persona_id(""),
            "PERSONA-001"
        );
    }

    #[test]
    fn test_suggest_context_for_tech_level() {
        let context = PersonaService::suggest_context_for_tech_level(5);
        assert!(context.contains("Expert"));
        
        let context = PersonaService::suggest_context_for_tech_level(3);
        assert!(context.contains("Intermediate"));
        
        let context = PersonaService::suggest_context_for_tech_level(1);
        assert!(context.contains("Beginner"));
    }
}
