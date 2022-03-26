use std::borrow::Cow;

use usiem::events::schema::FieldSchema;
use usiem_kernel::SiemBasicKernel;
use usiem_basic_parser::BasicParserComponent;
use usiem_sqlite_store::{SqliteDatastore,  proxy::SqliteProxyOptions};
use usiem_dm_sqlite::SqliteDatasetManager;
use usiem_syslog::input::SyslogInput;
fn main() {

    let mut kernel = SiemBasicKernel::new(1024, 4, 1000);

    let dataset_manager = SqliteDatasetManager::new("algo.db".to_string()).expect("Should start the dataset manager");
    let options = SqliteProxyOptions::default();
    let sqlite_store = SqliteDatastore::new(FieldSchema::new(), ".".to_string(), options);

    let parser_component = BasicParserComponent::new();
    // Add parsers to the parser
    //parser.add_parser(parser);


    let input = SyslogInput::new(Cow::Borrowed("localhost:22002"));

    kernel.register_input_component(Box::new(input));
    kernel.register_dataset_manager(Box::new(dataset_manager));
    kernel.register_parser_component(Box::new(parser_component));
    kernel.register_output_component(Box::new(sqlite_store));

    //Next lines are required to run
    //kernel.register_enricher_component(Box::new(enricher));
    //kernel.register_rule_engine_component(Box::new(ruler));
    //kernel.register_alert_component(Box::new(alerter));
    //kernel.register_state_storage(Box::new(state_storage));

    kernel.run();

}
