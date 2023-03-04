use esp_idf_sys as _;

fn main() {
    esp_idf_sys::link_patches();

    println!("rust_esp32_std_lepton");
}
