// mod record;
mod parquetter;

fn main() {
    parquetter::read_parquet();
    // let v = "osdu:wks:work-product-component--WellLog:1.2.0".splitn(3,':').last().unwrap();
    // println!("{}", v);
    // record::print_message();
}
