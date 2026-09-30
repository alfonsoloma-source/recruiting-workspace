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
