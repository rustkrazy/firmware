use std::fs::File;

const FIRMWARE_VERSION: &str = "cc64c94b50fa2120ef6f2825fb3105f6691197a6";

fn main() -> anyhow::Result<()> {
    let fwcopy = [
        "bootcode.bin",
        "fixup.dat",
        "fixup4.dat",
        "fixup4cd.dat",
        "fixup4db.dat",
        "fixup4x.dat",
        "fixup_cd.dat",
        "fixup_db.dat",
        "fixup_x.dat",
        "start.elf",
        "start4.elf",
        "start4cd.elf",
        "start4db.elf",
        "start4x.elf",
        "start_cd.elf",
        "start_db.elf",
        "start_x.elf",
    ];

    for fw in fwcopy {
        println!("Downloading RPi firmware: {}", fw);

        let mut file = File::create(fw)?;

        let url = format!(
            "https://github.com/raspberrypi/firmware/blob/{}/boot/{}",
            FIRMWARE_VERSION, fw
        );
        let mut resp = reqwest::blocking::get(url)?.error_for_status()?;

        resp.copy_to(&mut file)?;
    }

    println!("Firmware updated successfully");
    Ok(())
}
