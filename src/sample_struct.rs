use std::fmt;

// 유저 구조체
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: Option<String>,
    active: bool,
}

impl User {
    /// 생성자 함수 (Self 반환)
    pub fn new(id: u32, username: &str, email: Option<&str>) -> Self {
        Self {
            id,
            username: username.to_string(),
            email: email.map(|s| s.to_string()),
            active: true,
        }
    }

    /// 읽기 메서드 (불변 참조)
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// 상태 변경 메서드 (가변 참조)
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// 이메일을 안전하게 가져오기
    pub fn email_or_default(&self) -> &str {
        self.email.as_deref().unwrap_or("no-email@example.com")
    }

    /// 이메일 유효성 검사 결과를 Result로 반환
    pub fn validate_email(&self) -> Result<(), String> {
        match &self.email {
            Some(e) if e.contains('@') => Ok(()),
            Some(_) => Err("이메일 형식이 올바르지 않습니다.".into()),
            None => Err("이메일이 없습니다.".into()),
        }
    }
}

// Display trait 구현 (사용자 정의 출력 방식)
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} - {} <{}>", self.id, self.username, self.email_or_default())
    }
}
