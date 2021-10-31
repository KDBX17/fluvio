pub use fluvio_controlplane_metadata::table::*;

mod convert {

    use crate::objects::{
        CreateRequest, DeleteRequest, ListResponse, ObjectFrom, ObjectTryFrom, WatchRequest,
    };
    use crate::{
        AdminSpec, NameFilter,
        objects::{ListRequest, Metadata, WatchResponse},
    };
    use super::TableSpec;

    impl AdminSpec for TableSpec {
        type ListFilter = NameFilter;
        type ListType = Metadata<Self>;
        type WatchResponseType = Self;
        const CREATE_TYPE: u8 = 5;
        type DeleteKey = String;
    }

    ObjectFrom!(CreateRequest, Table);
    ObjectFrom!(WatchRequest, Table);
    ObjectFrom!(WatchResponse, Table);
    ObjectFrom!(ListRequest, Table);
    ObjectFrom!(ListResponse, Table);
    ObjectFrom!(DeleteRequest, Table);

    ObjectTryFrom!(WatchResponse, Table);
    ObjectTryFrom!(ListResponse, Table);
}
