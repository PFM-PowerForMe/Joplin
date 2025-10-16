use std::rc::Rc;

use crate::one::property_set::page_series_node;
use crate::onenote::page::{Page, parse_page};
use crate::onestore::OneStore;
use crate::onestore::object_space::ObjectSpaceRef;
use crate::shared::exguid::ExGuid;
use parser_utils::errors::{ErrorKind, Result};

/// A series of page.
///
/// See [\[MS-ONE\] 1.3.2] and [\[MS-ONE\] 2.2.18].
///
/// [\[MS-ONE\] 1.3.2]: https://docs.microsoft.com/en-us/openspecs/office_file_formats/ms-one/2dd687ac-f36b-4723-b959-4d60c8a90ca9
/// [\[MS-ONE\] 2.2.18]: https://docs.microsoft.com/en-us/openspecs/office_file_formats/ms-one/e2957d3b-a2a8-4756-8662-4e67fefa9f4e
#[derive(Clone, Debug)]
pub struct PageSeries {
    pages: Vec<Page>,
}

impl PageSeries {
    /// The pages contained in this page series.
    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
}

pub(crate) fn parse_page_series(id: ExGuid, store: Rc<dyn OneStore>) -> Result<PageSeries> {
    let object = store
        .data_root()
        .get_object(id)
        .ok_or_else(|| ErrorKind::MalformedOneNoteData("page series object is missing".into()))?;
    let data = page_series_node::parse(object.as_ref())?;

    let pages = data
        .page_spaces
        .into_iter()
        .map(|page_space_id| {
            let space = store
                .object_space(page_space_id)
                .ok_or_else(|| ErrorKind::MalformedOneNoteData("page space is missing".into()))?;
            Ok(space)
        })
        .map(|page_space: Result<ObjectSpaceRef>| parse_page(page_space?))
        .collect::<Result<_>>()?;

    Ok(PageSeries { pages })
}
