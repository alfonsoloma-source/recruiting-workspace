use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftContext {
    pub candidate_name: String,
    pub job_title: String,
    pub stage: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftRequest {
    pub purpose: String,
    pub context: DraftContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftResponse {
    pub provider: String,
    pub content: String,
}

pub trait AiProvider: Send + Sync {
    fn id(&self) -> &'static str;
    fn generate_draft(&self, request: &DraftRequest) -> Result<DraftResponse, String>;
}

pub struct LocalTemplateProvider;

impl AiProvider for LocalTemplateProvider {
    fn id(&self) -> &'static str { "local-template" }

    fn generate_draft(&self, request: &DraftRequest) -> Result<DraftResponse, String> {
        let first_name = request.context.candidate_name.split_whitespace().next().unwrap_or("Hola");
        let content = match request.purpose.as_str() {
            "follow_up" => format!(
                "Hola {}, ¿cómo estás? Quería dar seguimiento a tu proceso para la posición de {}. Seguimos avanzando con tu candidatura y quería mantenerte al tanto. Si tienes alguna duda o actualización de tu lado, con gusto la revisamos.",
                first_name, request.context.job_title
            ),
            _ => format!("Hola {}, te escribo para dar seguimiento a tu proceso de {}.", first_name, request.context.job_title),
        };
        Ok(DraftResponse { provider: self.id().into(), content })
    }
}

pub fn provider() -> Box<dyn AiProvider> { Box::new(LocalTemplateProvider) }


pub struct OpenAiProvider {
    api_key: String,
    model: String,
}

impl OpenAiProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self { api_key, model: model.filter(|m| !m.trim().is_empty()).unwrap_or_else(|| "gpt-5-mini".into()) }
    }
}

impl AiProvider for OpenAiProvider {
    fn id(&self) -> &'static str { "openai" }

    fn generate_draft(&self, request: &DraftRequest) -> Result<DraftResponse, String> {
        let notes = if request.context.notes.is_empty() { "Sin notas adicionales.".into() } else { request.context.notes.join("\n- ") };
        let prompt = format!(
            "Redacta un mensaje breve y profesional en español para un candidato. No inventes información. Propósito: {}. Candidato: {}. Vacante: {}. Etapa: {}. Notas: {}. Devuelve únicamente el mensaje listo para editar por el recruiter.",
            request.purpose, request.context.candidate_name, request.context.job_title, request.context.stage, notes
        );
        let body = serde_json::json!({
            "model": self.model,
            "input": prompt
        });
        let response = ureq::post("https://api.openai.com/v1/responses")
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .send_json(body)
            .map_err(|e| format!("OpenAI request failed: {e}"))?;
        let value: serde_json::Value = response.into_json().map_err(|e| e.to_string())?;
        let content = value.get("output")
            .and_then(|v| v.as_array())
            .and_then(|items| items.iter().flat_map(|i| i.get("content").and_then(|c| c.as_array()).into_iter().flatten()).find_map(|c| c.get("text").and_then(|t| t.as_str())))
            .ok_or_else(|| "OpenAI response did not contain text output".to_string())?
            .trim().to_string();
        Ok(DraftResponse { provider: self.id().into(), content })
    }
}
