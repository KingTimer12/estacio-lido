use serde::{Deserialize, Serialize};

pub struct Data {
    pub matricula_id: String,
    pub content_id: String,
    pub theme_id: String,
    pub course_id: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    #[serde(rename = "idExterno")]
    pub id_externo: String,
    pub nome: String,
    #[serde(rename = "emailPessoal")]
    pub email_pessoal: String,
    pub origem: String,
    #[serde(rename = "emailUtilizado")]
    pub email_utilizado: String,
    pub plataforma: String,
    pub config: Option<Config>,
    pub credenciais: Vec<String>,
    pub marca: String,
    pub matriculas: Vec<Matricula>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "visualizouAvisoBDQ")]
    pub visualizou_aviso_bdq: Option<bool>,
    #[serde(rename = "visualizouOnboarding")]
    pub visualizou_onboarding: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matricula {
    pub id: String,
    #[serde(rename = "idExterno")]
    pub id_externo: String,
    pub calendario: String,
    pub campus: String,
    pub entregas: Vec<String>,
    pub turmas: Vec<String>,
    pub matricula: String,
    pub modalidade: String,
    #[serde(rename = "nomeCurso")]
    pub nome_curso: String,
    pub situacao: String,
    pub periodo: String,
    pub regional: String,
    #[serde(rename = "tipoCurso")]
    pub tipo_curso: String,
    #[serde(rename = "periodosAnteriores")]
    pub periodos_anteriores: Vec<String>,
    pub turno: String,
    #[serde(rename = "codigoCurso")]
    pub codigo_curso: String,
    #[serde(rename = "turmasCursadas")]
    pub turmas_cursadas: Vec<TurmaCursada>,
    #[serde(rename = "periodoFlexivelAtual")]
    pub periodo_flexivel_atual: String,
    #[serde(rename = "totalPeriodosCurso")]
    pub total_periodos_curso: String,
    pub safra: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurmaCursada {
    pub id: String,
    pub situacao: String,
    #[serde(rename = "periodoMatricula")]
    pub periodo_matricula: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    #[serde(rename = "codigoEntrega")]
    pub codigo_entrega: String,
    pub formato: String,
    pub id: String,
    #[serde(rename = "idDisciplina")]
    pub id_disciplina: String,
    pub marca: String,
    #[serde(rename = "totalAlunosMatriculados")]
    pub total_alunos_matriculados: u32,
    pub campus: String,
    pub educadores: Vec<Educador>,
    pub horarios: Vec<Horario>,
    pub disciplina: Discipline,
    #[serde(rename = "habilitarAgendamentoLaboratorio")]
    pub habilitar_agendamento_laboratorio: bool,
    #[serde(rename = "habilitarTutoriaZeroDuvidas")]
    pub habilitar_tutoria_zero_duvidas: bool,
    #[serde(rename = "habilitarComunidadeZeroDuvidas")]
    pub habilitar_comunidade_zero_duvidas: bool,
    #[serde(rename = "habilitarAtendimentoCoordenacaoZeroDuvidas")]
    pub habilitar_atendimento_coordenacao_zero_duvidas: bool,
    #[serde(rename = "habilitarRedirecionamentoTeams")]
    pub habilitar_redirecionamento_teams: bool,
    #[serde(rename = "tipoTurma")]
    pub tipo_turma: String,
    #[serde(rename = "tipoCurso")]
    pub tipo_curso: String,
    #[serde(rename = "educadorResponsavel")]
    pub educador_responsavel: Educador,
    #[serde(rename = "periodoAcademico")]
    pub periodo_academico: String,
    pub local: Local,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Educador {
    pub nome: String,
    pub perfil: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Horario {
    #[serde(rename = "diaSemana")]
    pub dia_semana: String,
    #[serde(rename = "horaInicio")]
    pub hora_inicio: String,
    #[serde(rename = "horaFim")]
    pub hora_fim: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Local {
    pub blocos: Vec<String>,
    pub salas: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discipline {
    pub id: String,
    #[serde(rename = "grupoMarca")]
    pub grupo_marca: String,
    pub codigo: String,
    pub nome: String,
    pub embaixadora: bool,
    pub temas: Vec<Theme>,
    pub objetos: Vec<Objeto>,
    #[serde(rename = "dataAtualizacao")]
    pub data_atualizacao: String,
    pub versionamento: Versionamento,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Versionamento {
    #[serde(rename = "periodosAcademicos")]
    pub periodos_academicos: Vec<String>,
    #[serde(rename = "dataCriacao")]
    pub data_criacao: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub id: String,
    pub titulo: String,
    pub ordem: u32,
    pub tipo: String,
    #[serde(rename = "possuiExercicios")]
    pub possui_exercicios: bool,
    pub categoria: String,
    pub rotulo: String,
    pub objetos: Vec<Objeto>,
    #[serde(rename = "creditoDigital")]
    pub credito_digital: String,
    pub versionado: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objeto {
    pub id: String,
    pub ordem: u32,
    pub titulo: String,
    #[serde(rename = "quemPodeVer")]
    pub quem_pode_ver: String,
    pub tipo: String,
    pub url: String,
    #[serde(rename = "possuiTempoMinimo")]
    pub possui_tempo_minimo: Option<bool>,
    #[serde(rename = "tempoMinimo")]
    pub tempo_minimo: Option<u32>,
    pub extensionista: Option<bool>,
    pub obrigatorio: Option<bool>,
    pub versionada: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtherTheme {
    pub id: String,
    pub titulo: String,
    pub ordem: u32,
    pub rotulo: String,
    pub categoria: String,
    pub conteudos: Vec<Content>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub tipo: String,
    pub id: String,
    pub ordem: u32,
    pub titulo: String,
    pub conclusivel: bool,
    #[serde(rename = "statusConclusao")]
    pub status_conclusao: Status,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub concluido: bool,
    #[serde(rename = "dataConclusao")]
    pub data_conclusao: Option<String>,
    #[serde(rename = "tempoMinimoConclusaoConteudo")]
    pub tempo_minimo_conclusao_conteudo: u32,
    #[serde(rename = "tempoRestanteConclusaoEmSegundos")]
    pub tempo_restante_conclusao_em_segundos: u32,
}
