#[allow(dead_code)]
pub struct Contact {
	name: String
}

#[tarpc::service]
pub trait Contacts {
	async fn test(i: u32) -> Result<u32, String>;
	// async fn all(user_id: String) -> Result<Vec<Contact>>;
	// async fn post(user_id: String, data: Contact) -> Result<Contact>;
	// async fn get(user_id: String, contact_id: String) -> Result<Contact>;
	// async fn delete(user_id: String, contact_id: String) -> Option<()>;
	// async fn update(user_id: String, contact_id: String, data: Contact) -> Result<Contact>;
}