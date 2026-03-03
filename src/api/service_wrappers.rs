use crate::Client;
use crate::client::OperationBuilder;
use crate::generated::ops;

pub struct ImV1FileApi<'a> {
    client: &'a Client,
}

impl<'a> ImV1FileApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::im::v1::file::CREATE)
    }

    pub fn get(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::im::v1::file::GET)
    }
}

pub struct ImV1ImageApi<'a> {
    client: &'a Client,
}

impl<'a> ImV1ImageApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::im::v1::image::CREATE)
    }

    pub fn get(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::im::v1::image::GET)
    }
}

pub struct ImV1PinApi<'a> {
    client: &'a Client,
}

impl<'a> ImV1PinApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::im::v1::pin::CREATE)
    }

    pub fn list(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::im::v1::pin::LIST)
    }

    pub fn delete(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::im::v1::pin::DELETE)
    }
}

pub struct ImV1ReactionApi<'a> {
    client: &'a Client,
}

impl<'a> ImV1ReactionApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::im::v1::message_reaction::CREATE)
    }

    pub fn list(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::im::v1::message_reaction::LIST)
    }

    pub fn delete(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::im::v1::message_reaction::DELETE)
    }
}

pub struct ImV1ThreadApi<'a> {
    client: &'a Client,
}

impl<'a> ImV1ThreadApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn forward(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::im::v1::thread::FORWARD)
    }
}

pub struct ContactV3DepartmentApi<'a> {
    client: &'a Client,
}

impl<'a> ContactV3DepartmentApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::department::CREATE)
    }

    pub fn get(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::department::GET)
    }

    pub fn list(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::department::LIST)
    }

    pub fn update(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::department::UPDATE)
    }

    pub fn delete(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::department::DELETE)
    }
}

pub struct ContactV3GroupApi<'a> {
    client: &'a Client,
}

impl<'a> ContactV3GroupApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::group::CREATE)
    }

    pub fn get(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::group::GET)
    }

    pub fn simplelist(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::group::SIMPLELIST)
    }

    pub fn patch(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::group::PATCH)
    }

    pub fn delete(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::group::DELETE)
    }
}

pub struct ContactV3UnitApi<'a> {
    client: &'a Client,
}

impl<'a> ContactV3UnitApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::unit::CREATE)
    }

    pub fn get(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::unit::GET)
    }

    pub fn list(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::unit::LIST)
    }

    pub fn patch(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::unit::PATCH)
    }

    pub fn delete(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::contact::v3::unit::DELETE)
    }
}

pub struct DriveV1FolderApi<'a> {
    client: &'a Client,
}

impl<'a> DriveV1FolderApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::drive::v1::file::CREATE_FOLDER)
    }

    pub fn list(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::drive::v1::file::LIST)
    }
}

pub struct DriveV1PermissionApi<'a> {
    client: &'a Client,
}

impl<'a> DriveV1PermissionApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn member_create(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::drive::v1::permission_member::CREATE)
    }

    pub fn member_list(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::drive::v1::permission_member::LIST)
    }

    pub fn member_update(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::drive::v1::permission_member::UPDATE)
    }

    pub fn member_delete(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::drive::v1::permission_member::DELETE)
    }

    pub fn public_get(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::drive::v1::permission_public::GET)
    }

    pub fn public_patch(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::drive::v1::permission_public::PATCH)
    }
}

pub struct CalendarV4CalendarEventApi<'a> {
    client: &'a Client,
}

impl<'a> CalendarV4CalendarEventApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::calendar::v4::calendar_event::CREATE)
    }

    pub fn get(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::calendar::v4::calendar_event::GET)
    }

    pub fn list(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::calendar::v4::calendar_event::LIST)
    }

    pub fn patch(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::calendar::v4::calendar_event::PATCH)
    }

    pub fn delete(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::calendar::v4::calendar_event::DELETE)
    }
}

pub struct DocxV1DocumentApi<'a> {
    client: &'a Client,
}

impl<'a> DocxV1DocumentApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::docx::v1::document::CREATE)
    }

    pub fn get(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::docx::v1::document::GET)
    }

    pub fn raw_content(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::docx::v1::document::RAW_CONTENT)
    }
}

pub struct DocxV1BlockApi<'a> {
    client: &'a Client,
}

impl<'a> DocxV1BlockApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn get(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::docx::v1::document_block::GET)
    }

    pub fn list(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::docx::v1::document_block::LIST)
    }

    pub fn patch(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::docx::v1::document_block::PATCH)
    }

    pub fn batch_update(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::docx::v1::document_block::BATCH_UPDATE)
    }
}

pub struct SheetsV3SpreadsheetApi<'a> {
    client: &'a Client,
}

impl<'a> SheetsV3SpreadsheetApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::sheets::v3::spreadsheet::CREATE)
    }

    pub fn get(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::sheets::v3::spreadsheet::GET)
    }

    pub fn patch(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::sheets::v3::spreadsheet::PATCH)
    }
}

pub struct SheetsV3SheetApi<'a> {
    client: &'a Client,
}

impl<'a> SheetsV3SheetApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn find(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::sheets::v3::spreadsheet_sheet::FIND)
    }

    pub fn get(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::sheets::v3::spreadsheet_sheet::GET)
    }

    pub fn query(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::sheets::v3::spreadsheet_sheet::QUERY)
    }

    pub fn replace(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::sheets::v3::spreadsheet_sheet::REPLACE)
    }
}

pub struct BitableV1AppApi<'a> {
    client: &'a Client,
}

impl<'a> BitableV1AppApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::bitable::v1::app::CREATE)
    }

    pub fn get(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::bitable::v1::app::GET)
    }

    pub fn update(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::bitable::v1::app::UPDATE)
    }
}

pub struct BitableV1TableApi<'a> {
    client: &'a Client,
}

impl<'a> BitableV1TableApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::bitable::v1::app_table::CREATE)
    }

    pub fn list(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::bitable::v1::app_table::LIST)
    }

    pub fn patch(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::bitable::v1::app_table::PATCH)
    }

    pub fn delete(&self) -> OperationBuilder<'a> {
        self.client.operation(ops::bitable::v1::app_table::DELETE)
    }
}

pub struct BitableV1RecordApi<'a> {
    client: &'a Client,
}

impl<'a> BitableV1RecordApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::bitable::v1::app_table_record::CREATE)
    }

    pub fn get(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::bitable::v1::app_table_record::GET)
    }

    pub fn list(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::bitable::v1::app_table_record::LIST)
    }

    pub fn update(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::bitable::v1::app_table_record::UPDATE)
    }

    pub fn delete(&self) -> OperationBuilder<'a> {
        self.client
            .operation(ops::bitable::v1::app_table_record::DELETE)
    }
}
