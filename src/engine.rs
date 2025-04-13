use log::info;
use ouroboros::self_referencing;
use rime_api::{Rime, Session, Traits};
use xkbcommon::xkb;

/// 使用單一會話。
#[self_referencing]
struct EngineInner {
    api: Rime,
    #[borrows(api)]
    #[covariant]
    session: Session<'this>,
}

pub struct Engine(EngineInner);

impl Engine {
    /// 新建輸入法引擎。
    pub fn new() -> Self {
        // api
        let api = Rime::new().expect("fail to create api");

        // traits
        let shared_data_dir = option_env!("RIME_SHARED_DATA_DIR").unwrap_or("/usr/share/rime-data");
        let config_dir = dirs::config_dir()
            .expect("fail to get config dir")
            .join("wlrime")
            .join("rime");
        let mut traits = Traits::builder()
            .shared_data_dir(shared_data_dir)
            .user_data_dir(&config_dir.to_string_lossy())
            .distribution_name("wlrime")
            .distribution_code_name("wlrime")
            .distribution_version("0.1.0")
            .app_name("rime.wlrime")
            .build()
            .expect("fail to build traits");

        // setup, initialize and maintain
        api.setup(&mut traits);
        api.set_notification_handler(|session_id, ty, value| {
            info!("Handle notification: {session_id} {ty} {value}")
        });
        api.initialize(&mut traits);
        api.start_maintenance(true);
        api.join_maintenance_thread();

        let inner = EngineInner::new(api, |api| api.create_session());

        Self(inner)
    }

    /// 獲取會話。
    pub fn session(&self) -> &Session {
        self.0.borrow_session()
    }

    pub fn preedit(&self) -> Preedit {
        let context = self.session().context();
        let composition = context.composition();
        let start = composition.sel_start();
        let end = composition.sel_end();
        let text = composition
            .preedit()
            .map(|result| result.unwrap().to_string());
        Preedit { start, end, text }
    }

    pub fn key(&mut self, key: xkb::Keysym, mods: xkb::ModMask) -> bool {
        let key = key.raw() as i32;
        let mods = mods as i32;
        self.session().process_key(key, mods)
    }

    pub fn candidate(&self) -> CandidateInfo {
        let context = self.session().context();
        let menu = context.menu();
        let highlighted_candidate_index = menu.highlighted_candidate_index();
        // let page_no = menu.page_no();
        let num_candidates = menu.num_candidates();
        CandidateInfo {
            // page_no,
            highlighted_candidate_index,
            num_candidates,
        }
    }

    pub fn candidate_get(&self, index: i32) -> Option<String> {
        let context = self.session().context();
        let menu = context.menu();
        let mut candidates = menu.candidates();
        let cand = candidates.nth(index as usize);
        let text = cand
            .flatten()
            .and_then(|c| c.text().map(|result| result.unwrap().to_string()));
        text
    }

    /// 获取
    pub fn get_commit(&self) -> Option<String> {
        let commit = self.session().commit();
        commit.text().map(|res| res.unwrap().to_string())
    }

    /// 切換 ASCII 模式。
    pub fn toggle(&mut self) {
        let session = self.session();
        let ascii_mode = session.get_option_c(c"ascii_mode");
        session.set_option_c(c"ascii_mode", !ascii_mode);
    }

    pub fn reset(&mut self) {
        let session = self.session();
        session.clear_composition();
        session.commit_composition();
    }

    pub fn is_bypass(&self) -> bool {
        let status = self.session().status();
        !status.is_composing() || status.is_ascii_mode()
    }
}

pub struct Preedit {
    pub start: i32,
    pub end: i32,
    pub text: Option<String>,
}

pub struct CandidateInfo {
    // pub page_no: i32,
    pub highlighted_candidate_index: i32,
    pub num_candidates: i32,
}
