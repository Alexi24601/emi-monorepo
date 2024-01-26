use sirius::prelude::*;
use std::path::Path;

#[test]
fn test_run_sirius() -> Result<(), String> {
    let sirius = SiriusBuilder::default()
        .maximal_mz(1000.0)?
        .isotope_settings_filter(false)?
        .build();

    let input_file_path = Path::new("tests/data/input_sirius.mgf");
    let output_file_path = Path::new("tests/data/output_sirius");

    sirius.run(input_file_path, output_file_path).unwrap();

    Ok(())
}
