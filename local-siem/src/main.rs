use usiem::events::schema::FieldSchema;
use usiem_kernel::SiemBasicKernel;
use usiem_basic_parser::BasicParserComponent;
use usiem_sqlite_store::{SqliteDatastore,  proxy::SqliteProxyOptions};

fn main() {
    let options = SqliteProxyOptions::default();
    let sqlite_store = SqliteDatastore::new(FieldSchema::new(), "./".to_string(), options);
    let mut kernel = SiemBasicKernel::new(1024, 4, 1000);
    let parser_component = BasicParserComponent::new();
    // Add parsers to the parser
    //parser.add_parser(parser);
    kernel.register_parser_component(Box::new(parser_component));
    kernel.register_output_component(Box::new(sqlite_store));
    kernel.run();

}
