use std::borrow::Cow;
use std::fmt::Display;
use std::sync::{OnceLock, RwLock};

use clap::ValueEnum;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, ValueEnum)]
pub enum Language {
    #[default]
    #[value(name = "en", alias = "english")]
    English,
    #[value(name = "fr", alias = "french")]
    French,
    #[value(name = "de", alias = "german")]
    German,
    #[value(name = "nl", alias = "dutch")]
    Dutch,
    #[value(name = "es", alias = "spanish")]
    Spanish,
    #[value(name = "pt", alias = "portuguese")]
    Portuguese,
    #[value(name = "zh", alias = "chinese")]
    Chinese,
    #[value(name = "ko", alias = "korean")]
    Korean,
    #[value(name = "ar", alias = "arabic")]
    Arabic,
    #[value(name = "ja", alias = "japanese")]
    Japanese,
}

#[derive(Debug, Clone, Copy)]
pub enum Text {
    Ok,
    Warning,
    Error,
    Profile,
    Url,
    ApiKey,
    Config,
    ConfigFile,
    DefaultProfile,
    Profiles,
    Id,
    Name,
    Description,
    Score,
    Analysis,
    Status,
    Type,
    Version,
    Licenses,
    Feature,
    Function,
    Username,
    Password,
    Filename,
    Engine,
    Product,
    Summary,
    Vendor,
    KeySize,
    Aux,
    Behaviors,
    Syscalls,
    Canary,
    Nx,
    Pie,
    Relro,
    Fortify,
    Severity,
    Objects,
    Scan,
    OverallScore,
    Default,
    CveVulnerabilities,
    MalwareDetections,
    PasswordIssues,
    HardeningIssues,
    Capabilities,
    Crypto,
    SoftwareBom,
    Kernel,
    Symbols,
    Tasks,
    StackOverflow,
    KernelConfig,
    Bind,
    SuccessStatus,
    PendingStatus,
    InProgressStatus,
    CanceledStatus,
    ErrorStatus,
    Running,
    Queued,
    Done,
}

pub fn set_language(language: Language) {
    *language_lock().write().expect("language lock poisoned") = language;
}

pub fn language() -> Language {
    *language_lock().read().expect("language lock poisoned")
}

pub fn language_name() -> &'static str {
    match language() {
        Language::English => "English",
        Language::French => "Francais",
        Language::German => "Deutsch",
        Language::Dutch => "Nederlands",
        Language::Spanish => "Espanol",
        Language::Portuguese => "Portugues",
        Language::Chinese => "中文",
        Language::Korean => "한국어",
        Language::Arabic => "العربية",
        Language::Japanese => "日本語",
    }
}

pub fn text(key: Text) -> &'static str {
    match language() {
        Language::English => text_en(key),
        Language::French => text_fr(key),
        Language::German => text_de(key),
        Language::Dutch => text_nl(key),
        Language::Spanish => text_es(key),
        Language::Portuguese => text_pt(key),
        Language::Chinese => text_zh(key),
        Language::Korean => text_ko(key),
        Language::Arabic => text_ar(key),
        Language::Japanese => text_ja(key),
    }
}

pub fn tagline() -> &'static str {
    match language() {
        Language::English => "Secure every artifact",
        Language::French => "Securisez chaque artefact",
        Language::German => "Sichern Sie jedes Artefakt",
        Language::Dutch => "Beveilig elk artefact",
        Language::Spanish => "Protege cada artefacto",
        Language::Portuguese => "Proteja cada artefato",
        Language::Chinese => "为每个制品提供安全保障",
        Language::Korean => "모든 아티팩트를 안전하게 보호하세요",
        Language::Arabic => "أمّن كل أصل برمجي",
        Language::Japanese => "すべての成果物を安全に",
    }
}

pub fn subtitle() -> String {
    match language() {
        Language::English => format!(
            "Copilot-style terminal theme • active language: {}",
            language_name()
        ),
        Language::French => format!(
            "Theme terminal moderne • langue active : {}",
            language_name()
        ),
        Language::German => format!(
            "Modernes Terminal-Theme • aktive Sprache: {}",
            language_name()
        ),
        Language::Dutch => format!("Modern terminalthema • actieve taal: {}", language_name()),
        Language::Spanish => format!(
            "Tema de terminal moderno • idioma activo: {}",
            language_name()
        ),
        Language::Portuguese => format!(
            "Tema moderno de terminal • idioma ativo: {}",
            language_name()
        ),
        Language::Chinese => format!("现代终端主题 • 当前语言：{}", language_name()),
        Language::Korean => format!("모던 터미널 테마 • 현재 언어: {}", language_name()),
        Language::Arabic => format!("سمة طرفية حديثة • اللغة النشطة: {}", language_name()),
        Language::Japanese => format!("モダンなターミナルテーマ • 現在の言語: {}", language_name()),
    }
}

pub fn analyzer_cli() -> &'static str {
    "Analyzer CLI"
}

pub fn analyzer_cli_configuration() -> &'static str {
    match language() {
        Language::English => "Analyzer CLI Configuration",
        Language::French => "Configuration d'Analyzer CLI",
        Language::German => "Analyzer CLI Konfiguration",
        Language::Dutch => "Analyzer CLI-configuratie",
        Language::Spanish => "Configuracion de Analyzer CLI",
        Language::Portuguese => "Configuracao do Analyzer CLI",
        Language::Chinese => "Analyzer CLI 配置",
        Language::Korean => "Analyzer CLI 구성",
        Language::Arabic => "إعدادات Analyzer CLI",
        Language::Japanese => "Analyzer CLI の設定",
    }
}

pub fn configuring_profile(profile: &str) -> String {
    match language() {
        Language::English => format!("Configuring profile '{profile}'"),
        Language::French => format!("Configuration du profil '{profile}'"),
        Language::German => format!("Profil '{profile}' wird konfiguriert"),
        Language::Dutch => format!("Profiel '{profile}' wordt geconfigureerd"),
        Language::Spanish => format!("Configurando el perfil '{profile}'"),
        Language::Portuguese => format!("Configurando o perfil '{profile}'"),
        Language::Chinese => format!("正在配置配置文件“{profile}”"),
        Language::Korean => format!("'{profile}' 프로필을 구성하는 중입니다"),
        Language::Arabic => format!("جارٍ إعداد الملف الشخصي '{profile}'"),
        Language::Japanese => format!("プロファイル '{profile}' を設定中"),
    }
}

pub fn enter_api_key() -> &'static str {
    match language() {
        Language::English => "Enter your API key:",
        Language::French => "Saisissez votre cle API :",
        Language::German => "API-Schlussel eingeben:",
        Language::Dutch => "Voer uw API-sleutel in:",
        Language::Spanish => "Introduce tu clave API:",
        Language::Portuguese => "Informe sua chave de API:",
        Language::Chinese => "请输入您的 API 密钥：",
        Language::Korean => "API 키를 입력하세요:",
        Language::Arabic => "أدخل مفتاح API:",
        Language::Japanese => "API キーを入力してください:",
    }
}

pub fn api_key_cannot_be_empty() -> &'static str {
    match language() {
        Language::English => "API key cannot be empty",
        Language::French => "La cle API ne peut pas etre vide",
        Language::German => "Der API-Schlussel darf nicht leer sein",
        Language::Dutch => "De API-sleutel mag niet leeg zijn",
        Language::Spanish => "La clave API no puede estar vacia",
        Language::Portuguese => "A chave de API nao pode estar vazia",
        Language::Chinese => "API 密钥不能为空",
        Language::Korean => "API 키는 비워 둘 수 없습니다",
        Language::Arabic => "لا يمكن أن يكون مفتاح API فارغًا",
        Language::Japanese => "API キーは空にできません",
    }
}

pub fn validating_api_key() -> &'static str {
    match language() {
        Language::English => "Validating API key...",
        Language::French => "Validation de la cle API...",
        Language::German => "API-Schlussel wird gepruft...",
        Language::Dutch => "API-sleutel wordt gevalideerd...",
        Language::Spanish => "Validando la clave API...",
        Language::Portuguese => "Validando a chave de API...",
        Language::Chinese => "正在验证 API 密钥...",
        Language::Korean => "API 키를 확인하는 중입니다...",
        Language::Arabic => "جارٍ التحقق من مفتاح API...",
        Language::Japanese => "API キーを検証しています...",
    }
}

pub fn key_accepted() -> &'static str {
    match language() {
        Language::English => "Key accepted. You're in.",
        Language::French => "Cle acceptee. Vous etes connecte.",
        Language::German => "Schlussel akzeptiert. Zugriff gewahrt.",
        Language::Dutch => "Sleutel geaccepteerd. U bent binnen.",
        Language::Spanish => "Clave aceptada. Ya estas dentro.",
        Language::Portuguese => "Chave aceita. Acesso liberado.",
        Language::Chinese => "密钥已接受，登录成功。",
        Language::Korean => "키가 승인되었습니다. 접속 완료.",
        Language::Arabic => "تم قبول المفتاح. تم تسجيل الدخول.",
        Language::Japanese => "キーが受け入れられました。準備完了です。",
    }
}

pub fn could_not_validate<E: Display>(error: E) -> String {
    match language() {
        Language::English => format!(
            "Could not validate key ({error}). Saving anyway - the server may be unreachable."
        ),
        Language::French => format!(
            "Impossible de valider la cle ({error}). Enregistrement quand meme - le serveur est peut-etre indisponible."
        ),
        Language::German => format!(
            "Schlussel konnte nicht gepruft werden ({error}). Er wird trotzdem gespeichert - der Server ist moglicherweise nicht erreichbar."
        ),
        Language::Dutch => format!(
            "De sleutel kon niet worden gevalideerd ({error}). Toch opgeslagen - de server is mogelijk onbereikbaar."
        ),
        Language::Spanish => format!(
            "No se pudo validar la clave ({error}). Se guardara igualmente - puede que el servidor no este disponible."
        ),
        Language::Portuguese => format!(
            "Nao foi possivel validar a chave ({error}). Ela sera salva mesmo assim - talvez o servidor esteja indisponivel."
        ),
        Language::Chinese => format!("无法验证密钥（{error}）。仍将保存，服务器可能暂时不可达。"),
        Language::Korean => format!(
            "키를 검증할 수 없습니다 ({error}). 서버에 연결할 수 없을 수 있어도 그대로 저장합니다."
        ),
        Language::Arabic => format!(
            "تعذر التحقق من المفتاح ({error}). سيتم حفظه على أي حال لأن الخادم قد يكون غير متاح."
        ),
        Language::Japanese => format!(
            "キーを検証できませんでした ({error})。サーバーに到達できない可能性があるため、そのまま保存します。"
        ),
    }
}

pub fn config_saved(path: impl Display) -> String {
    match language() {
        Language::English => format!("Config saved to {path}"),
        Language::French => format!("Configuration enregistree dans {path}"),
        Language::German => format!("Konfiguration gespeichert unter {path}"),
        Language::Dutch => format!("Configuratie opgeslagen in {path}"),
        Language::Spanish => format!("Configuracion guardada en {path}"),
        Language::Portuguese => format!("Configuracao salva em {path}"),
        Language::Chinese => format!("配置已保存到 {path}"),
        Language::Korean => format!("구성이 {path} 에 저장되었습니다"),
        Language::Arabic => format!("تم حفظ الإعدادات في {path}"),
        Language::Japanese => format!("設定を {path} に保存しました"),
    }
}

pub fn ready_to_hunt() -> &'static str {
    match language() {
        Language::English => "Ready to hunt vulnerabilities. Try:",
        Language::French => "Pret a traquer les vulnerabilites. Essayez :",
        Language::German => "Bereit, Schwachstellen zu finden. Probieren Sie:",
        Language::Dutch => "Klaar om kwetsbaarheden op te sporen. Probeer:",
        Language::Spanish => "Listo para buscar vulnerabilidades. Prueba:",
        Language::Portuguese => "Pronto para encontrar vulnerabilidades. Tente:",
        Language::Chinese => "已准备好开始排查漏洞。试试：",
        Language::Korean => "취약점 탐색 준비가 끝났습니다. 예시:",
        Language::Arabic => "أنت جاهز للبحث عن الثغرات. جرّب:",
        Language::Japanese => "脆弱性の調査を始めましょう。次を試してください:",
    }
}

pub fn list_your_objects() -> &'static str {
    match language() {
        Language::English => "list your objects",
        Language::French => "liste vos objets",
        Language::German => "Objekte auflisten",
        Language::Dutch => "toon uw objecten",
        Language::Spanish => "listar tus objetos",
        Language::Portuguese => "listar seus objetos",
        Language::Chinese => "列出对象",
        Language::Korean => "오브젝트 목록 보기",
        Language::Arabic => "اعرض العناصر",
        Language::Japanese => "オブジェクト一覧を表示",
    }
}

pub fn available_scan_types() -> &'static str {
    match language() {
        Language::English => "available scan types",
        Language::French => "types d'analyse disponibles",
        Language::German => "verfugbare Scan-Typen",
        Language::Dutch => "beschikbare scantypen",
        Language::Spanish => "tipos de analisis disponibles",
        Language::Portuguese => "tipos de analise disponiveis",
        Language::Chinese => "可用扫描类型",
        Language::Korean => "사용 가능한 스캔 유형",
        Language::Arabic => "أنواع الفحص المتاحة",
        Language::Japanese => "利用可能なスキャン種別",
    }
}

pub fn start_a_scan() -> &'static str {
    match language() {
        Language::English => "start a scan",
        Language::French => "demarrer une analyse",
        Language::German => "einen Scan starten",
        Language::Dutch => "een scan starten",
        Language::Spanish => "iniciar un analisis",
        Language::Portuguese => "iniciar uma analise",
        Language::Chinese => "开始扫描",
        Language::Korean => "스캔 시작",
        Language::Arabic => "ابدأ فحصًا",
        Language::Japanese => "スキャンを開始",
    }
}

pub fn no_profiles_configured() -> &'static str {
    match language() {
        Language::English => "No profiles configured. Run: analyzer login",
        Language::French => "Aucun profil configure. Lancez : analyzer login",
        Language::German => "Keine Profile konfiguriert. Ausfuhren: analyzer login",
        Language::Dutch => "Geen profielen geconfigureerd. Voer uit: analyzer login",
        Language::Spanish => "No hay perfiles configurados. Ejecuta: analyzer login",
        Language::Portuguese => "Nenhum perfil configurado. Execute: analyzer login",
        Language::Chinese => "尚未配置任何 profile。请运行：analyzer login",
        Language::Korean => "구성된 프로필이 없습니다. 다음을 실행하세요: analyzer login",
        Language::Arabic => "لا توجد ملفات شخصية مهيأة. شغّل: analyzer login",
        Language::Japanese => {
            "設定されたプロファイルがありません。`analyzer login` を実行してください"
        }
    }
}

pub fn value_set() -> &'static str {
    match language() {
        Language::English => "set",
        Language::French => "definie",
        Language::German => "gesetzt",
        Language::Dutch => "ingesteld",
        Language::Spanish => "configurada",
        Language::Portuguese => "definida",
        Language::Chinese => "已设置",
        Language::Korean => "설정됨",
        Language::Arabic => "تم التعيين",
        Language::Japanese => "設定済み",
    }
}

pub fn value_not_set() -> &'static str {
    match language() {
        Language::English => "not set",
        Language::French => "non definie",
        Language::German => "nicht gesetzt",
        Language::Dutch => "niet ingesteld",
        Language::Spanish => "sin configurar",
        Language::Portuguese => "nao definida",
        Language::Chinese => "未设置",
        Language::Korean => "설정되지 않음",
        Language::Arabic => "غير معيّن",
        Language::Japanese => "未設定",
    }
}

pub fn default_value() -> &'static str {
    match language() {
        Language::English => "(default)",
        Language::French => "(par defaut)",
        Language::German => "(Standard)",
        Language::Dutch => "(standaard)",
        Language::Spanish => "(predeterminado)",
        Language::Portuguese => "(padrao)",
        Language::Chinese => "（默认）",
        Language::Korean => "(기본값)",
        Language::Arabic => "(افتراضي)",
        Language::Japanese => "（既定）",
    }
}

pub fn not_set_value() -> &'static str {
    match language() {
        Language::English => "(not set)",
        Language::French => "(non defini)",
        Language::German => "(nicht gesetzt)",
        Language::Dutch => "(niet ingesteld)",
        Language::Spanish => "(sin configurar)",
        Language::Portuguese => "(nao definido)",
        Language::Chinese => "（未设置）",
        Language::Korean => "(설정되지 않음)",
        Language::Arabic => "(غير معيّن)",
        Language::Japanese => "（未設定）",
    }
}

pub fn valid_config_keys() -> &'static str {
    "Valid keys: url, api-key, default-profile"
}

pub fn unknown_config_key(other: &str) -> String {
    match language() {
        Language::English => format!("Unknown config key: {other}\n\n{}", valid_config_keys()),
        Language::French => format!(
            "Cle de configuration inconnue : {other}\n\n{}",
            valid_config_keys()
        ),
        Language::German => format!(
            "Unbekannter Konfigurationsschlussel: {other}\n\n{}",
            valid_config_keys()
        ),
        Language::Dutch => format!(
            "Onbekende configuratiesleutel: {other}\n\n{}",
            valid_config_keys()
        ),
        Language::Spanish => format!(
            "Clave de configuracion desconocida: {other}\n\n{}",
            valid_config_keys()
        ),
        Language::Portuguese => format!(
            "Chave de configuracao desconhecida: {other}\n\n{}",
            valid_config_keys()
        ),
        Language::Chinese => format!("未知配置键：{other}\n\n{}", valid_config_keys()),
        Language::Korean => format!("알 수 없는 구성 키: {other}\n\n{}", valid_config_keys()),
        Language::Arabic => format!("مفتاح إعداد غير معروف: {other}\n\n{}", valid_config_keys()),
        Language::Japanese => format!("不明な設定キーです: {other}\n\n{}", valid_config_keys()),
    }
}

pub fn set_config_value(key: &str, value: &str, profile: &str) -> String {
    match language() {
        Language::English => format!("Set {key} = {value} (profile: {profile})"),
        Language::French => format!("{key} = {value} defini (profil : {profile})"),
        Language::German => format!("{key} = {value} gesetzt (Profil: {profile})"),
        Language::Dutch => format!("{key} = {value} ingesteld (profiel: {profile})"),
        Language::Spanish => format!("{key} = {value} configurado (perfil: {profile})"),
        Language::Portuguese => format!("{key} = {value} definido (perfil: {profile})"),
        Language::Chinese => format!("已设置 {key} = {value}（profile: {profile}）"),
        Language::Korean => format!("{key} = {value} 로 설정했습니다 (프로필: {profile})"),
        Language::Arabic => format!("تم تعيين {key} = {value} (الملف الشخصي: {profile})"),
        Language::Japanese => format!("{key} = {value} を設定しました (プロファイル: {profile})"),
    }
}

pub fn objects_empty() -> &'static str {
    match language() {
        Language::English => "None found. Create one with: analyzer object new <name>",
        Language::French => "Aucun objet trouve. Creez-en un avec : analyzer object new <name>",
        Language::German => {
            "Keine Objekte gefunden. Erstellen Sie eines mit: analyzer object new <name>"
        }
        Language::Dutch => "Geen objecten gevonden. Maak er een met: analyzer object new <name>",
        Language::Spanish => "No se encontraron objetos. Crea uno con: analyzer object new <name>",
        Language::Portuguese => "Nenhum objeto encontrado. Crie um com: analyzer object new <name>",
        Language::Chinese => "未找到对象。使用以下命令创建：analyzer object new <name>",
        Language::Korean => "오브젝트가 없습니다. 다음으로 생성하세요: analyzer object new <name>",
        Language::Arabic => "لم يتم العثور على عناصر. أنشئ واحدًا عبر: analyzer object new <name>",
        Language::Japanese => {
            "オブジェクトが見つかりません。`analyzer object new <name>` で作成してください"
        }
    }
}

pub fn created_object(name: &str, id: impl Display) -> String {
    match language() {
        Language::English => format!("Created object '{name}' ({id})"),
        Language::French => format!("Objet '{name}' cree ({id})"),
        Language::German => format!("Objekt '{name}' erstellt ({id})"),
        Language::Dutch => format!("Object '{name}' aangemaakt ({id})"),
        Language::Spanish => format!("Objeto '{name}' creado ({id})"),
        Language::Portuguese => format!("Objeto '{name}' criado ({id})"),
        Language::Chinese => format!("已创建对象“{name}”（{id}）"),
        Language::Korean => format!("오브젝트 '{name}' 생성됨 ({id})"),
        Language::Arabic => format!("تم إنشاء العنصر '{name}' ({id})"),
        Language::Japanese => format!("オブジェクト '{name}' を作成しました ({id})"),
    }
}

pub fn deleted_object(id: impl Display) -> String {
    match language() {
        Language::English => format!("Deleted object {id}"),
        Language::French => format!("Objet supprime {id}"),
        Language::German => format!("Objekt {id} geloscht"),
        Language::Dutch => format!("Object {id} verwijderd"),
        Language::Spanish => format!("Objeto {id} eliminado"),
        Language::Portuguese => format!("Objeto {id} removido"),
        Language::Chinese => format!("已删除对象 {id}"),
        Language::Korean => format!("오브젝트 {id} 삭제됨"),
        Language::Arabic => format!("تم حذف العنصر {id}"),
        Language::Japanese => format!("オブジェクト {id} を削除しました"),
    }
}

pub fn scan_created(id: impl Display) -> String {
    match language() {
        Language::English => format!("Scan {id} created"),
        Language::French => format!("Analyse {id} creee"),
        Language::German => format!("Scan {id} erstellt"),
        Language::Dutch => format!("Scan {id} aangemaakt"),
        Language::Spanish => format!("Analisis {id} creado"),
        Language::Portuguese => format!("Analise {id} criada"),
        Language::Chinese => format!("扫描 {id} 已创建"),
        Language::Korean => format!("스캔 {id} 생성됨"),
        Language::Arabic => format!("تم إنشاء الفحص {id}"),
        Language::Japanese => format!("スキャン {id} を作成しました"),
    }
}

pub fn check_status_command(object_id: impl Display) -> String {
    match language() {
        Language::English => {
            format!("Check status with: analyzer scan status --object {object_id}")
        }
        Language::French => {
            format!("Verifier l'etat avec : analyzer scan status --object {object_id}")
        }
        Language::German => format!("Status prufen mit: analyzer scan status --object {object_id}"),
        Language::Dutch => {
            format!("Controleer de status met: analyzer scan status --object {object_id}")
        }
        Language::Spanish => {
            format!("Consulta el estado con: analyzer scan status --object {object_id}")
        }
        Language::Portuguese => {
            format!("Verifique o status com: analyzer scan status --object {object_id}")
        }
        Language::Chinese => {
            format!("使用以下命令检查状态：analyzer scan status --object {object_id}")
        }
        Language::Korean => format!("상태 확인: analyzer scan status --object {object_id}"),
        Language::Arabic => {
            format!("تحقق من الحالة عبر: analyzer scan status --object {object_id}")
        }
        Language::Japanese => format!("状態確認: analyzer scan status --object {object_id}"),
    }
}

pub fn deleted_scan(id: impl Display) -> String {
    match language() {
        Language::English => format!("Deleted scan {id}"),
        Language::French => format!("Analyse supprimee {id}"),
        Language::German => format!("Scan {id} geloscht"),
        Language::Dutch => format!("Scan {id} verwijderd"),
        Language::Spanish => format!("Analisis {id} eliminado"),
        Language::Portuguese => format!("Analise {id} removida"),
        Language::Chinese => format!("已删除扫描 {id}"),
        Language::Korean => format!("스캔 {id} 삭제됨"),
        Language::Arabic => format!("تم حذف الفحص {id}"),
        Language::Japanese => format!("スキャン {id} を削除しました"),
    }
}

pub fn cancelled_scan(id: impl Display) -> String {
    match language() {
        Language::English => format!("Cancelled scan {id}"),
        Language::French => format!("Analyse annulee {id}"),
        Language::German => format!("Scan {id} abgebrochen"),
        Language::Dutch => format!("Scan {id} geannuleerd"),
        Language::Spanish => format!("Analisis {id} cancelado"),
        Language::Portuguese => format!("Analise {id} cancelada"),
        Language::Chinese => format!("已取消扫描 {id}"),
        Language::Korean => format!("스캔 {id} 취소됨"),
        Language::Arabic => format!("تم إلغاء الفحص {id}"),
        Language::Japanese => format!("スキャン {id} をキャンセルしました"),
    }
}

pub fn downloading_pdf_report() -> &'static str {
    match language() {
        Language::English => "Downloading PDF report...",
        Language::French => "Telechargement du rapport PDF...",
        Language::German => "PDF-Bericht wird heruntergeladen...",
        Language::Dutch => "PDF-rapport wordt gedownload...",
        Language::Spanish => "Descargando informe PDF...",
        Language::Portuguese => "Baixando relatorio PDF...",
        Language::Chinese => "正在下载 PDF 报告...",
        Language::Korean => "PDF 보고서를 다운로드하는 중입니다...",
        Language::Arabic => "جارٍ تنزيل تقرير PDF...",
        Language::Japanese => "PDF レポートをダウンロードしています...",
    }
}

pub fn report_saved(path: impl Display) -> String {
    match language() {
        Language::English => format!("Report saved to {path}"),
        Language::French => format!("Rapport enregistre dans {path}"),
        Language::German => format!("Bericht gespeichert unter {path}"),
        Language::Dutch => format!("Rapport opgeslagen in {path}"),
        Language::Spanish => format!("Informe guardado en {path}"),
        Language::Portuguese => format!("Relatorio salvo em {path}"),
        Language::Chinese => format!("报告已保存到 {path}"),
        Language::Korean => format!("보고서가 {path} 에 저장되었습니다"),
        Language::Arabic => format!("تم حفظ التقرير في {path}"),
        Language::Japanese => format!("レポートを {path} に保存しました"),
    }
}

pub fn downloading_sbom() -> &'static str {
    match language() {
        Language::English => "Downloading SBOM...",
        Language::French => "Telechargement du SBOM...",
        Language::German => "SBOM wird heruntergeladen...",
        Language::Dutch => "SBOM wordt gedownload...",
        Language::Spanish => "Descargando SBOM...",
        Language::Portuguese => "Baixando SBOM...",
        Language::Chinese => "正在下载 SBOM...",
        Language::Korean => "SBOM을 다운로드하는 중입니다...",
        Language::Arabic => "جارٍ تنزيل SBOM...",
        Language::Japanese => "SBOM をダウンロードしています...",
    }
}

pub fn sbom_saved(path: impl Display) -> String {
    match language() {
        Language::English => format!("SBOM saved to {path}"),
        Language::French => format!("SBOM enregistre dans {path}"),
        Language::German => format!("SBOM gespeichert unter {path}"),
        Language::Dutch => format!("SBOM opgeslagen in {path}"),
        Language::Spanish => format!("SBOM guardado en {path}"),
        Language::Portuguese => format!("SBOM salvo em {path}"),
        Language::Chinese => format!("SBOM 已保存到 {path}"),
        Language::Korean => format!("SBOM이 {path} 에 저장되었습니다"),
        Language::Arabic => format!("تم حفظ SBOM في {path}"),
        Language::Japanese => format!("SBOM を {path} に保存しました"),
    }
}

pub fn downloading_compliance_report(name: &str) -> String {
    match language() {
        Language::English => format!("Downloading {name} compliance report..."),
        Language::French => format!("Telechargement du rapport de conformite {name}..."),
        Language::German => format!("{name}-Compliance-Bericht wird heruntergeladen..."),
        Language::Dutch => format!("{name}-compliancerapport wordt gedownload..."),
        Language::Spanish => format!("Descargando el informe de cumplimiento {name}..."),
        Language::Portuguese => format!("Baixando relatorio de conformidade {name}..."),
        Language::Chinese => format!("正在下载 {name} 合规报告..."),
        Language::Korean => format!("{name} 규정 준수 보고서를 다운로드하는 중입니다..."),
        Language::Arabic => format!("جارٍ تنزيل تقرير الامتثال {name}..."),
        Language::Japanese => format!("{name} コンプライアンスレポートをダウンロードしています..."),
    }
}

pub fn compliance_report_saved(name: &str, path: impl Display) -> String {
    match language() {
        Language::English => format!("{name} report saved to {path}"),
        Language::French => format!("Rapport {name} enregistre dans {path}"),
        Language::German => format!("{name}-Bericht gespeichert unter {path}"),
        Language::Dutch => format!("{name}-rapport opgeslagen in {path}"),
        Language::Spanish => format!("Informe {name} guardado en {path}"),
        Language::Portuguese => format!("Relatorio {name} salvo em {path}"),
        Language::Chinese => format!("{name} 报告已保存到 {path}"),
        Language::Korean => format!("{name} 보고서가 {path} 에 저장되었습니다"),
        Language::Arabic => format!("تم حفظ تقرير {name} في {path}"),
        Language::Japanese => format!("{name} レポートを {path} に保存しました"),
    }
}

pub fn waiting_for_scan() -> &'static str {
    match language() {
        Language::English => "Waiting for scan to complete...",
        Language::French => "En attente de la fin de l'analyse...",
        Language::German => "Warten auf den Abschluss des Scans...",
        Language::Dutch => "Wachten tot de scan is voltooid...",
        Language::Spanish => "Esperando a que finalice el analisis...",
        Language::Portuguese => "Aguardando a conclusao da analise...",
        Language::Chinese => "正在等待扫描完成...",
        Language::Korean => "스캔 완료를 기다리는 중입니다...",
        Language::Arabic => "جارٍ انتظار اكتمال الفحص...",
        Language::Japanese => "スキャンの完了を待機しています...",
    }
}

pub fn scan_completed_successfully() -> &'static str {
    match language() {
        Language::English => "Scan completed successfully!",
        Language::French => "Analyse terminee avec succes !",
        Language::German => "Scan erfolgreich abgeschlossen!",
        Language::Dutch => "Scan succesvol voltooid!",
        Language::Spanish => "Analisis completado correctamente.",
        Language::Portuguese => "Analise concluida com sucesso!",
        Language::Chinese => "扫描已成功完成！",
        Language::Korean => "스캔이 성공적으로 완료되었습니다!",
        Language::Arabic => "اكتمل الفحص بنجاح!",
        Language::Japanese => "スキャンが正常に完了しました！",
    }
}

pub fn scan_failed_with_error_status() -> &'static str {
    match language() {
        Language::English => "Scan failed with error status",
        Language::French => "L'analyse a echoue avec un statut d'erreur",
        Language::German => "Scan mit Fehlerstatus fehlgeschlagen",
        Language::Dutch => "Scan is mislukt met foutstatus",
        Language::Spanish => "El analisis fallo con estado de error",
        Language::Portuguese => "A analise falhou com status de erro",
        Language::Chinese => "扫描以错误状态失败",
        Language::Korean => "스캔이 오류 상태로 실패했습니다",
        Language::Arabic => "فشل الفحص بحالة خطأ",
        Language::Japanese => "スキャンがエラーステータスで失敗しました",
    }
}

pub fn scan_was_cancelled() -> &'static str {
    match language() {
        Language::English => "Scan was cancelled",
        Language::French => "L'analyse a ete annulee",
        Language::German => "Scan wurde abgebrochen",
        Language::Dutch => "Scan is geannuleerd",
        Language::Spanish => "El analisis fue cancelado",
        Language::Portuguese => "A analise foi cancelada",
        Language::Chinese => "扫描已取消",
        Language::Korean => "스캔이 취소되었습니다",
        Language::Arabic => "تم إلغاء الفحص",
        Language::Japanese => "スキャンはキャンセルされました",
    }
}

pub fn analyzing(parts: &str) -> String {
    match language() {
        Language::English => format!("Analyzing... [{parts}]"),
        Language::French => format!("Analyse en cours... [{parts}]"),
        Language::German => format!("Analyse lauft... [{parts}]"),
        Language::Dutch => format!("Bezig met analyseren... [{parts}]"),
        Language::Spanish => format!("Analizando... [{parts}]"),
        Language::Portuguese => format!("Analisando... [{parts}]"),
        Language::Chinese => format!("正在分析... [{parts}]"),
        Language::Korean => format!("분석 중... [{parts}]"),
        Language::Arabic => format!("جارٍ التحليل... [{parts}]"),
        Language::Japanese => format!("解析中... [{parts}]"),
    }
}

pub fn timed_out_waiting_for_scan(seconds: u64) -> String {
    match language() {
        Language::English => format!("Timed out waiting for scan to complete ({seconds}s)"),
        Language::French => format!("Delai depasse en attendant la fin de l'analyse ({seconds}s)"),
        Language::German => format!("Zeituberschreitung beim Warten auf den Scan ({seconds}s)"),
        Language::Dutch => format!("Time-out tijdens wachten op scanvoltooiing ({seconds}s)"),
        Language::Spanish => {
            format!("Tiempo de espera agotado al esperar el analisis ({seconds}s)")
        }
        Language::Portuguese => format!("Tempo esgotado aguardando a analise ({seconds}s)"),
        Language::Chinese => format!("等待扫描完成超时（{seconds}s）"),
        Language::Korean => format!("스캔 완료 대기 시간이 초과되었습니다 ({seconds}s)"),
        Language::Arabic => format!("انتهت مهلة انتظار اكتمال الفحص ({seconds}s)"),
        Language::Japanese => format!("スキャン完了待機がタイムアウトしました ({seconds}s)"),
    }
}

pub fn no_findings() -> &'static str {
    match language() {
        Language::English => "No findings.",
        Language::French => "Aucun resultat.",
        Language::German => "Keine Befunde.",
        Language::Dutch => "Geen bevindingen.",
        Language::Spanish => "Sin hallazgos.",
        Language::Portuguese => "Nenhum achado.",
        Language::Chinese => "没有发现项。",
        Language::Korean => "발견된 항목이 없습니다.",
        Language::Arabic => "لا توجد نتائج.",
        Language::Japanese => "検出結果はありません。",
    }
}

pub fn page_navigation(page: u32, total_pages: u64, total_findings: u64) -> String {
    match language() {
        Language::English => {
            format!("Page {page}/{total_pages} ({total_findings} total) - use --page N to navigate")
        }
        Language::French => format!(
            "Page {page}/{total_pages} ({total_findings} au total) - utilisez --page N pour naviguer"
        ),
        Language::German => format!(
            "Seite {page}/{total_pages} ({total_findings} gesamt) - mit --page N navigieren"
        ),
        Language::Dutch => format!(
            "Pagina {page}/{total_pages} ({total_findings} totaal) - gebruik --page N om te navigeren"
        ),
        Language::Spanish => format!(
            "Pagina {page}/{total_pages} ({total_findings} en total) - usa --page N para navegar"
        ),
        Language::Portuguese => format!(
            "Pagina {page}/{total_pages} ({total_findings} no total) - use --page N para navegar"
        ),
        Language::Chinese => {
            format!("第 {page}/{total_pages} 页（共 {total_findings} 条）- 使用 --page N 翻页")
        }
        Language::Korean => {
            format!(
                "{page}/{total_pages} 페이지 (총 {total_findings}개) - 이동하려면 --page N 사용"
            )
        }
        Language::Arabic => {
            format!(
                "الصفحة {page}/{total_pages} (الإجمالي {total_findings}) - استخدم --page N للتنقل"
            )
        }
        Language::Japanese => {
            format!(
                "{page}/{total_pages} ページ / 全 {total_findings} 件 - 移動には --page N を使用"
            )
        }
    }
}

pub fn status_display(status: &str) -> Cow<'static, str> {
    match status {
        "success" => Cow::Borrowed(text(Text::SuccessStatus)),
        "pending" => Cow::Borrowed(text(Text::PendingStatus)),
        "in-progress" => Cow::Borrowed(text(Text::InProgressStatus)),
        "canceled" => Cow::Borrowed(text(Text::CanceledStatus)),
        "error" => Cow::Borrowed(text(Text::ErrorStatus)),
        _ => Cow::Owned(status.to_string()),
    }
}

pub fn progress_word(status: &str) -> &'static str {
    match status {
        "success" => text(Text::Done),
        "in-progress" => text(Text::Running),
        "pending" => text(Text::Queued),
        _ => "?",
    }
}

fn language_lock() -> &'static RwLock<Language> {
    static LANGUAGE: OnceLock<RwLock<Language>> = OnceLock::new();
    LANGUAGE.get_or_init(|| RwLock::new(Language::English))
}

fn text_en(key: Text) -> &'static str {
    match key {
        Text::Ok => "OK",
        Text::Warning => "WARN",
        Text::Error => "ERR",
        Text::Profile => "Profile",
        Text::Url => "URL",
        Text::ApiKey => "API Key",
        Text::Config => "Config",
        Text::ConfigFile => "Config file",
        Text::DefaultProfile => "Default profile",
        Text::Profiles => "Profiles",
        Text::Id => "ID",
        Text::Name => "Name",
        Text::Description => "Description",
        Text::Score => "Score",
        Text::Analysis => "Analysis",
        Text::Status => "Status",
        Text::Type => "Type",
        Text::Version => "Version",
        Text::Licenses => "Licenses",
        Text::Feature => "Feature",
        Text::Function => "Function",
        Text::Username => "Username",
        Text::Password => "Password",
        Text::Filename => "Filename",
        Text::Engine => "Engine",
        Text::Product => "Product",
        Text::Summary => "Summary",
        Text::Vendor => "Vendor",
        Text::KeySize => "Key Size",
        Text::Aux => "Aux",
        Text::Behaviors => "Behaviors",
        Text::Syscalls => "Syscalls",
        Text::Canary => "Canary",
        Text::Nx => "NX",
        Text::Pie => "PIE",
        Text::Relro => "RELRO",
        Text::Fortify => "Fortify",
        Text::Severity => "Severity",
        Text::Objects => "Objects",
        Text::Scan => "Scan",
        Text::OverallScore => "Overall Score",
        Text::Default => "default",
        Text::CveVulnerabilities => "CVE Vulnerabilities",
        Text::MalwareDetections => "Malware Detections",
        Text::PasswordIssues => "Password Issues",
        Text::HardeningIssues => "Hardening Issues",
        Text::Capabilities => "Capabilities",
        Text::Crypto => "Crypto",
        Text::SoftwareBom => "Software BOM",
        Text::Kernel => "Kernel",
        Text::Symbols => "Symbols",
        Text::Tasks => "Tasks",
        Text::StackOverflow => "Stack Overflow",
        Text::KernelConfig => "Kernel Config",
        Text::Bind => "Bind",
        Text::SuccessStatus => "success",
        Text::PendingStatus => "pending",
        Text::InProgressStatus => "in progress",
        Text::CanceledStatus => "canceled",
        Text::ErrorStatus => "error",
        Text::Running => "running",
        Text::Queued => "queued",
        Text::Done => "done",
    }
}

fn text_fr(key: Text) -> &'static str {
    match key {
        Text::Ok => "OK",
        Text::Warning => "ATTN",
        Text::Error => "ERR",
        Text::Profile => "Profil",
        Text::Url => "URL",
        Text::ApiKey => "Cle API",
        Text::Config => "Config",
        Text::ConfigFile => "Fichier config",
        Text::DefaultProfile => "Profil par defaut",
        Text::Profiles => "Profils",
        Text::Id => "ID",
        Text::Name => "Nom",
        Text::Description => "Description",
        Text::Score => "Score",
        Text::Analysis => "Analyse",
        Text::Status => "Statut",
        Text::Type => "Type",
        Text::Version => "Version",
        Text::Licenses => "Licences",
        Text::Feature => "Fonction",
        Text::Function => "Fonction",
        Text::Username => "Utilisateur",
        Text::Password => "Mot de passe",
        Text::Filename => "Fichier",
        Text::Engine => "Moteur",
        Text::Product => "Produit",
        Text::Summary => "Resume",
        Text::Vendor => "Editeur",
        Text::KeySize => "Taille cle",
        Text::Aux => "Aux",
        Text::Behaviors => "Comportements",
        Text::Syscalls => "Syscalls",
        Text::Canary => "Canary",
        Text::Nx => "NX",
        Text::Pie => "PIE",
        Text::Relro => "RELRO",
        Text::Fortify => "Fortify",
        Text::Severity => "Gravite",
        Text::Objects => "Objets",
        Text::Scan => "Analyse",
        Text::OverallScore => "Score global",
        Text::Default => "defaut",
        Text::CveVulnerabilities => "Vulnerabilites CVE",
        Text::MalwareDetections => "Detections malware",
        Text::PasswordIssues => "Problemes de mot de passe",
        Text::HardeningIssues => "Problemes de durcissement",
        Text::Capabilities => "Capacites",
        Text::Crypto => "Crypto",
        Text::SoftwareBom => "BOM logicielle",
        Text::Kernel => "Noyau",
        Text::Symbols => "Symboles",
        Text::Tasks => "Taches",
        Text::StackOverflow => "Depassement de pile",
        Text::KernelConfig => "Config noyau",
        Text::Bind => "Lien",
        Text::SuccessStatus => "reussi",
        Text::PendingStatus => "en attente",
        Text::InProgressStatus => "en cours",
        Text::CanceledStatus => "annule",
        Text::ErrorStatus => "erreur",
        Text::Running => "en cours",
        Text::Queued => "file",
        Text::Done => "termine",
    }
}

fn text_de(key: Text) -> &'static str {
    match key {
        Text::Ok => "OK",
        Text::Warning => "WARN",
        Text::Error => "FEHL",
        Text::Profile => "Profil",
        Text::Url => "URL",
        Text::ApiKey => "API-Schlussel",
        Text::Config => "Konfig",
        Text::ConfigFile => "Konfigdatei",
        Text::DefaultProfile => "Standardprofil",
        Text::Profiles => "Profile",
        Text::Id => "ID",
        Text::Name => "Name",
        Text::Description => "Beschreibung",
        Text::Score => "Score",
        Text::Analysis => "Analyse",
        Text::Status => "Status",
        Text::Type => "Typ",
        Text::Version => "Version",
        Text::Licenses => "Lizenzen",
        Text::Feature => "Feature",
        Text::Function => "Funktion",
        Text::Username => "Benutzer",
        Text::Password => "Passwort",
        Text::Filename => "Datei",
        Text::Engine => "Engine",
        Text::Product => "Produkt",
        Text::Summary => "Zusammenfassung",
        Text::Vendor => "Hersteller",
        Text::KeySize => "Schlusselgrosse",
        Text::Aux => "Aux",
        Text::Behaviors => "Verhalten",
        Text::Syscalls => "Syscalls",
        Text::Canary => "Canary",
        Text::Nx => "NX",
        Text::Pie => "PIE",
        Text::Relro => "RELRO",
        Text::Fortify => "Fortify",
        Text::Severity => "Schweregrad",
        Text::Objects => "Objekte",
        Text::Scan => "Scan",
        Text::OverallScore => "Gesamtwert",
        Text::Default => "standard",
        Text::CveVulnerabilities => "CVE-Schwachstellen",
        Text::MalwareDetections => "Malware-Funde",
        Text::PasswordIssues => "Passwortprobleme",
        Text::HardeningIssues => "Hardening-Probleme",
        Text::Capabilities => "Fahigkeiten",
        Text::Crypto => "Krypto",
        Text::SoftwareBom => "Software-BOM",
        Text::Kernel => "Kernel",
        Text::Symbols => "Symbole",
        Text::Tasks => "Tasks",
        Text::StackOverflow => "Stack Overflow",
        Text::KernelConfig => "Kernel-Konfig",
        Text::Bind => "Bind",
        Text::SuccessStatus => "erfolgreich",
        Text::PendingStatus => "ausstehend",
        Text::InProgressStatus => "laufend",
        Text::CanceledStatus => "abgebrochen",
        Text::ErrorStatus => "fehler",
        Text::Running => "laufend",
        Text::Queued => "wartend",
        Text::Done => "fertig",
    }
}

fn text_nl(key: Text) -> &'static str {
    match key {
        Text::Ok => "OK",
        Text::Warning => "WAARS",
        Text::Error => "FOUT",
        Text::Profile => "Profiel",
        Text::Url => "URL",
        Text::ApiKey => "API-sleutel",
        Text::Config => "Config",
        Text::ConfigFile => "Configbestand",
        Text::DefaultProfile => "Standaardprofiel",
        Text::Profiles => "Profielen",
        Text::Id => "ID",
        Text::Name => "Naam",
        Text::Description => "Beschrijving",
        Text::Score => "Score",
        Text::Analysis => "Analyse",
        Text::Status => "Status",
        Text::Type => "Type",
        Text::Version => "Versie",
        Text::Licenses => "Licenties",
        Text::Feature => "Feature",
        Text::Function => "Functie",
        Text::Username => "Gebruiker",
        Text::Password => "Wachtwoord",
        Text::Filename => "Bestand",
        Text::Engine => "Engine",
        Text::Product => "Product",
        Text::Summary => "Samenvatting",
        Text::Vendor => "Leverancier",
        Text::KeySize => "Sleutelgrootte",
        Text::Aux => "Aux",
        Text::Behaviors => "Gedragingen",
        Text::Syscalls => "Syscalls",
        Text::Canary => "Canary",
        Text::Nx => "NX",
        Text::Pie => "PIE",
        Text::Relro => "RELRO",
        Text::Fortify => "Fortify",
        Text::Severity => "Ernst",
        Text::Objects => "Objecten",
        Text::Scan => "Scan",
        Text::OverallScore => "Totale score",
        Text::Default => "standaard",
        Text::CveVulnerabilities => "CVE-kwetsbaarheden",
        Text::MalwareDetections => "Malware-detecties",
        Text::PasswordIssues => "Wachtwoordproblemen",
        Text::HardeningIssues => "Hardening-problemen",
        Text::Capabilities => "Capaciteiten",
        Text::Crypto => "Crypto",
        Text::SoftwareBom => "Software BOM",
        Text::Kernel => "Kernel",
        Text::Symbols => "Symbolen",
        Text::Tasks => "Taken",
        Text::StackOverflow => "Stack Overflow",
        Text::KernelConfig => "Kernelconfig",
        Text::Bind => "Binding",
        Text::SuccessStatus => "geslaagd",
        Text::PendingStatus => "wachtend",
        Text::InProgressStatus => "bezig",
        Text::CanceledStatus => "geannuleerd",
        Text::ErrorStatus => "fout",
        Text::Running => "actief",
        Text::Queued => "in wachtrij",
        Text::Done => "klaar",
    }
}

fn text_es(key: Text) -> &'static str {
    match key {
        Text::Ok => "OK",
        Text::Warning => "AVISO",
        Text::Error => "ERR",
        Text::Profile => "Perfil",
        Text::Url => "URL",
        Text::ApiKey => "Clave API",
        Text::Config => "Config",
        Text::ConfigFile => "Archivo config",
        Text::DefaultProfile => "Perfil predeterminado",
        Text::Profiles => "Perfiles",
        Text::Id => "ID",
        Text::Name => "Nombre",
        Text::Description => "Descripcion",
        Text::Score => "Puntuacion",
        Text::Analysis => "Analisis",
        Text::Status => "Estado",
        Text::Type => "Tipo",
        Text::Version => "Version",
        Text::Licenses => "Licencias",
        Text::Feature => "Caracteristica",
        Text::Function => "Funcion",
        Text::Username => "Usuario",
        Text::Password => "Contrasena",
        Text::Filename => "Archivo",
        Text::Engine => "Motor",
        Text::Product => "Producto",
        Text::Summary => "Resumen",
        Text::Vendor => "Proveedor",
        Text::KeySize => "Tamano clave",
        Text::Aux => "Aux",
        Text::Behaviors => "Comportamientos",
        Text::Syscalls => "Syscalls",
        Text::Canary => "Canary",
        Text::Nx => "NX",
        Text::Pie => "PIE",
        Text::Relro => "RELRO",
        Text::Fortify => "Fortify",
        Text::Severity => "Severidad",
        Text::Objects => "Objetos",
        Text::Scan => "Analisis",
        Text::OverallScore => "Puntuacion global",
        Text::Default => "predeterminado",
        Text::CveVulnerabilities => "Vulnerabilidades CVE",
        Text::MalwareDetections => "Detecciones de malware",
        Text::PasswordIssues => "Problemas de contrasena",
        Text::HardeningIssues => "Problemas de hardening",
        Text::Capabilities => "Capacidades",
        Text::Crypto => "Cripto",
        Text::SoftwareBom => "SBOM",
        Text::Kernel => "Kernel",
        Text::Symbols => "Simbolos",
        Text::Tasks => "Tareas",
        Text::StackOverflow => "Desbordamiento de pila",
        Text::KernelConfig => "Config kernel",
        Text::Bind => "Enlace",
        Text::SuccessStatus => "correcto",
        Text::PendingStatus => "pendiente",
        Text::InProgressStatus => "en curso",
        Text::CanceledStatus => "cancelado",
        Text::ErrorStatus => "error",
        Text::Running => "ejecutando",
        Text::Queued => "en cola",
        Text::Done => "hecho",
    }
}

fn text_pt(key: Text) -> &'static str {
    match key {
        Text::Ok => "OK",
        Text::Warning => "AVISO",
        Text::Error => "ERR",
        Text::Profile => "Perfil",
        Text::Url => "URL",
        Text::ApiKey => "Chave API",
        Text::Config => "Config",
        Text::ConfigFile => "Arquivo config",
        Text::DefaultProfile => "Perfil padrao",
        Text::Profiles => "Perfis",
        Text::Id => "ID",
        Text::Name => "Nome",
        Text::Description => "Descricao",
        Text::Score => "Pontuacao",
        Text::Analysis => "Analise",
        Text::Status => "Status",
        Text::Type => "Tipo",
        Text::Version => "Versao",
        Text::Licenses => "Licencas",
        Text::Feature => "Recurso",
        Text::Function => "Funcao",
        Text::Username => "Usuario",
        Text::Password => "Senha",
        Text::Filename => "Arquivo",
        Text::Engine => "Motor",
        Text::Product => "Produto",
        Text::Summary => "Resumo",
        Text::Vendor => "Fornecedor",
        Text::KeySize => "Tam chave",
        Text::Aux => "Aux",
        Text::Behaviors => "Comportamentos",
        Text::Syscalls => "Syscalls",
        Text::Canary => "Canary",
        Text::Nx => "NX",
        Text::Pie => "PIE",
        Text::Relro => "RELRO",
        Text::Fortify => "Fortify",
        Text::Severity => "Severidade",
        Text::Objects => "Objetos",
        Text::Scan => "Analise",
        Text::OverallScore => "Pontuacao geral",
        Text::Default => "padrao",
        Text::CveVulnerabilities => "Vulnerabilidades CVE",
        Text::MalwareDetections => "Deteccoes de malware",
        Text::PasswordIssues => "Problemas de senha",
        Text::HardeningIssues => "Problemas de hardening",
        Text::Capabilities => "Capacidades",
        Text::Crypto => "Cripto",
        Text::SoftwareBom => "SBOM",
        Text::Kernel => "Kernel",
        Text::Symbols => "Simbolos",
        Text::Tasks => "Tarefas",
        Text::StackOverflow => "Estouro de pilha",
        Text::KernelConfig => "Config kernel",
        Text::Bind => "Bind",
        Text::SuccessStatus => "sucesso",
        Text::PendingStatus => "pendente",
        Text::InProgressStatus => "em andamento",
        Text::CanceledStatus => "cancelado",
        Text::ErrorStatus => "erro",
        Text::Running => "executando",
        Text::Queued => "na fila",
        Text::Done => "feito",
    }
}

fn text_zh(key: Text) -> &'static str {
    match key {
        Text::Ok => "成功",
        Text::Warning => "警告",
        Text::Error => "错误",
        Text::Profile => "配置文件",
        Text::Url => "URL",
        Text::ApiKey => "API 密钥",
        Text::Config => "配置",
        Text::ConfigFile => "配置文件",
        Text::DefaultProfile => "默认 profile",
        Text::Profiles => "Profiles",
        Text::Id => "ID",
        Text::Name => "名称",
        Text::Description => "描述",
        Text::Score => "评分",
        Text::Analysis => "分析",
        Text::Status => "状态",
        Text::Type => "类型",
        Text::Version => "版本",
        Text::Licenses => "许可证",
        Text::Feature => "特性",
        Text::Function => "函数",
        Text::Username => "用户名",
        Text::Password => "密码",
        Text::Filename => "文件名",
        Text::Engine => "引擎",
        Text::Product => "产品",
        Text::Summary => "摘要",
        Text::Vendor => "厂商",
        Text::KeySize => "密钥长度",
        Text::Aux => "辅助",
        Text::Behaviors => "行为",
        Text::Syscalls => "系统调用",
        Text::Canary => "Canary",
        Text::Nx => "NX",
        Text::Pie => "PIE",
        Text::Relro => "RELRO",
        Text::Fortify => "Fortify",
        Text::Severity => "严重级别",
        Text::Objects => "对象",
        Text::Scan => "扫描",
        Text::OverallScore => "总体评分",
        Text::Default => "默认",
        Text::CveVulnerabilities => "CVE 漏洞",
        Text::MalwareDetections => "恶意软件检出",
        Text::PasswordIssues => "密码问题",
        Text::HardeningIssues => "加固问题",
        Text::Capabilities => "能力",
        Text::Crypto => "密码学",
        Text::SoftwareBom => "软件 BOM",
        Text::Kernel => "内核",
        Text::Symbols => "符号",
        Text::Tasks => "任务",
        Text::StackOverflow => "栈溢出",
        Text::KernelConfig => "内核配置",
        Text::Bind => "绑定",
        Text::SuccessStatus => "成功",
        Text::PendingStatus => "等待中",
        Text::InProgressStatus => "进行中",
        Text::CanceledStatus => "已取消",
        Text::ErrorStatus => "错误",
        Text::Running => "运行中",
        Text::Queued => "排队中",
        Text::Done => "完成",
    }
}

fn text_ko(key: Text) -> &'static str {
    match key {
        Text::Ok => "확인",
        Text::Warning => "경고",
        Text::Error => "오류",
        Text::Profile => "프로필",
        Text::Url => "URL",
        Text::ApiKey => "API 키",
        Text::Config => "구성",
        Text::ConfigFile => "구성 파일",
        Text::DefaultProfile => "기본 프로필",
        Text::Profiles => "프로필",
        Text::Id => "ID",
        Text::Name => "이름",
        Text::Description => "설명",
        Text::Score => "점수",
        Text::Analysis => "분석",
        Text::Status => "상태",
        Text::Type => "유형",
        Text::Version => "버전",
        Text::Licenses => "라이선스",
        Text::Feature => "기능",
        Text::Function => "함수",
        Text::Username => "사용자 이름",
        Text::Password => "비밀번호",
        Text::Filename => "파일명",
        Text::Engine => "엔진",
        Text::Product => "제품",
        Text::Summary => "요약",
        Text::Vendor => "벤더",
        Text::KeySize => "키 크기",
        Text::Aux => "보조",
        Text::Behaviors => "동작",
        Text::Syscalls => "시스템 호출",
        Text::Canary => "Canary",
        Text::Nx => "NX",
        Text::Pie => "PIE",
        Text::Relro => "RELRO",
        Text::Fortify => "Fortify",
        Text::Severity => "심각도",
        Text::Objects => "오브젝트",
        Text::Scan => "스캔",
        Text::OverallScore => "전체 점수",
        Text::Default => "기본",
        Text::CveVulnerabilities => "CVE 취약점",
        Text::MalwareDetections => "멀웨어 탐지",
        Text::PasswordIssues => "비밀번호 문제",
        Text::HardeningIssues => "하드닝 문제",
        Text::Capabilities => "기능",
        Text::Crypto => "암호화",
        Text::SoftwareBom => "소프트웨어 BOM",
        Text::Kernel => "커널",
        Text::Symbols => "심볼",
        Text::Tasks => "작업",
        Text::StackOverflow => "스택 오버플로",
        Text::KernelConfig => "커널 설정",
        Text::Bind => "바인드",
        Text::SuccessStatus => "성공",
        Text::PendingStatus => "대기 중",
        Text::InProgressStatus => "진행 중",
        Text::CanceledStatus => "취소됨",
        Text::ErrorStatus => "오류",
        Text::Running => "실행 중",
        Text::Queued => "대기열",
        Text::Done => "완료",
    }
}

fn text_ar(key: Text) -> &'static str {
    match key {
        Text::Ok => "تم",
        Text::Warning => "تحذير",
        Text::Error => "خطأ",
        Text::Profile => "الملف الشخصي",
        Text::Url => "الرابط",
        Text::ApiKey => "مفتاح API",
        Text::Config => "الإعدادات",
        Text::ConfigFile => "ملف الإعدادات",
        Text::DefaultProfile => "الملف الافتراضي",
        Text::Profiles => "الملفات الشخصية",
        Text::Id => "المعرف",
        Text::Name => "الاسم",
        Text::Description => "الوصف",
        Text::Score => "النتيجة",
        Text::Analysis => "التحليل",
        Text::Status => "الحالة",
        Text::Type => "النوع",
        Text::Version => "الإصدار",
        Text::Licenses => "التراخيص",
        Text::Feature => "الميزة",
        Text::Function => "الدالة",
        Text::Username => "اسم المستخدم",
        Text::Password => "كلمة المرور",
        Text::Filename => "اسم الملف",
        Text::Engine => "المحرّك",
        Text::Product => "المنتج",
        Text::Summary => "الملخص",
        Text::Vendor => "المورّد",
        Text::KeySize => "حجم المفتاح",
        Text::Aux => "إضافي",
        Text::Behaviors => "السلوكيات",
        Text::Syscalls => "استدعاءات النظام",
        Text::Canary => "Canary",
        Text::Nx => "NX",
        Text::Pie => "PIE",
        Text::Relro => "RELRO",
        Text::Fortify => "Fortify",
        Text::Severity => "الخطورة",
        Text::Objects => "العناصر",
        Text::Scan => "الفحص",
        Text::OverallScore => "النتيجة العامة",
        Text::Default => "افتراضي",
        Text::CveVulnerabilities => "ثغرات CVE",
        Text::MalwareDetections => "اكتشافات البرمجيات الخبيثة",
        Text::PasswordIssues => "مشكلات كلمات المرور",
        Text::HardeningIssues => "مشكلات التقوية",
        Text::Capabilities => "القدرات",
        Text::Crypto => "التشفير",
        Text::SoftwareBom => "فاتورة البرمجيات",
        Text::Kernel => "النواة",
        Text::Symbols => "الرموز",
        Text::Tasks => "المهام",
        Text::StackOverflow => "تجاوز المكدس",
        Text::KernelConfig => "إعدادات النواة",
        Text::Bind => "الربط",
        Text::SuccessStatus => "ناجح",
        Text::PendingStatus => "قيد الانتظار",
        Text::InProgressStatus => "قيد التنفيذ",
        Text::CanceledStatus => "ملغي",
        Text::ErrorStatus => "خطأ",
        Text::Running => "يعمل",
        Text::Queued => "في الطابور",
        Text::Done => "مكتمل",
    }
}

fn text_ja(key: Text) -> &'static str {
    match key {
        Text::Ok => "OK",
        Text::Warning => "警告",
        Text::Error => "エラー",
        Text::Profile => "プロファイル",
        Text::Url => "URL",
        Text::ApiKey => "API キー",
        Text::Config => "設定",
        Text::ConfigFile => "設定ファイル",
        Text::DefaultProfile => "既定のプロファイル",
        Text::Profiles => "プロファイル",
        Text::Id => "ID",
        Text::Name => "名前",
        Text::Description => "説明",
        Text::Score => "スコア",
        Text::Analysis => "解析",
        Text::Status => "状態",
        Text::Type => "種類",
        Text::Version => "バージョン",
        Text::Licenses => "ライセンス",
        Text::Feature => "機能",
        Text::Function => "関数",
        Text::Username => "ユーザー名",
        Text::Password => "パスワード",
        Text::Filename => "ファイル名",
        Text::Engine => "エンジン",
        Text::Product => "製品",
        Text::Summary => "概要",
        Text::Vendor => "ベンダー",
        Text::KeySize => "鍵長",
        Text::Aux => "補助",
        Text::Behaviors => "挙動",
        Text::Syscalls => "システムコール",
        Text::Canary => "Canary",
        Text::Nx => "NX",
        Text::Pie => "PIE",
        Text::Relro => "RELRO",
        Text::Fortify => "Fortify",
        Text::Severity => "深刻度",
        Text::Objects => "オブジェクト",
        Text::Scan => "スキャン",
        Text::OverallScore => "総合スコア",
        Text::Default => "既定",
        Text::CveVulnerabilities => "CVE 脆弱性",
        Text::MalwareDetections => "マルウェア検出",
        Text::PasswordIssues => "パスワード問題",
        Text::HardeningIssues => "ハードニング問題",
        Text::Capabilities => "機能",
        Text::Crypto => "暗号",
        Text::SoftwareBom => "ソフトウェア BOM",
        Text::Kernel => "カーネル",
        Text::Symbols => "シンボル",
        Text::Tasks => "タスク",
        Text::StackOverflow => "スタックオーバーフロー",
        Text::KernelConfig => "カーネル設定",
        Text::Bind => "バインド",
        Text::SuccessStatus => "成功",
        Text::PendingStatus => "保留",
        Text::InProgressStatus => "進行中",
        Text::CanceledStatus => "キャンセル済み",
        Text::ErrorStatus => "エラー",
        Text::Running => "実行中",
        Text::Queued => "待機中",
        Text::Done => "完了",
    }
}
