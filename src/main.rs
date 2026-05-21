fn main() {
    //create report (heap allocated string)
    let mut report = String::from("This is a report.");
    
    // Borrow mutably to modify the report
    let report_mut_ref = &mut report;
    report_mut_ref.push_str(" It has been updated.");

    //Borrow the report as an immutable reference
    let report_ref = &report;

    //Ownership still valid here, we can use report_ref
    println!("{}", report_ref);
}

//function borrows mutable reference to modify the report
// it doen not take ownership of the report, so the caller retains ownership
fn update_report(report: &mut String) {
    report.push_str(" This is an update from the function.");
}

//function borrows immutable reference to read the report
fn print_report(report: &String) {
    println!("{}", report);
}
