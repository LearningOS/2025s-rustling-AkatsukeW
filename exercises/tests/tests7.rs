// tests7.rs
//
// When building packages, some dependencies can neither be imported in
// `Cargo.toml` nor be directly linked; some preprocesses varies from code
// generation to set-up package-specific configurations.
//
// Cargo does not aim to replace other build tools, but it does integrate
// with them with custom build scripts called `build.rs`. This file is
// usually placed in the root of the project, while in this case the same
// directory of this exercise.
//
// It can be used to:
//
// - Building a bundled C library.
// - Finding a C library on the host system.
// - Generating a Rust module from a specification.
// - Performing any platform-specific configuration needed for the crate.
//
// When setting up configurations, we can `println!` in the build script
// to tell Cargo to follow some instructions. The generic format is:
//
//     println!("cargo:{}", your_command_in_string);
//
// Please see the official Cargo book about build scripts for more
// information:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// In this exercise, we look for an environment variable and expect it to
// fall in a range. You can look into the testcase to find out the details.
//
// You should NOT modify this file. Modify `build.rs` in the same directory
// to pass this exercise.
//
// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let test_foo = env::var("TEST_FOO")
        .unwrap_or_else(|_| panic!("must set TEST_FOO "));

    let timestamp: u64 = test_foo.parse()
        .unwrap_or_else(|_| panic!("TEST_FOO must is valid unix time: {}", test_foo));

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("test_config.rs");

    let mut f = File::create(&dest_path).unwrap();
    writeln!(
        f,
        "pub const EXPECTED_START: u64 = {};\n\
         pub const VALID_RANGE: std::ops::RangeInclusive<u64> = {}..={};",
        timestamp, timestamp, timestamp + 10
    ).unwrap();

    println!("cargo:rerun-if-env-changed=TEST_FOO");
}

#[cfg(test)]
mod tests {
    use super::*;

    // 包含构建脚本生成的配置（网页3）
    include!(concat!(env!("OUT_DIR"), "/test_config.rs"));

    #[test]
    fn test_success() {
        // 1. 获取当前时间戳（网页1）
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("系统时间早于UNIX纪元时间")
            .as_secs();

        // 2. 使用预生成的配置（网页5）
        assert!(
            VALID_RANGE.contains(&now),
            "当前时间戳 {} 超出允许范围 [{}, {}]",
            now, VALID_RANGE.start(), VALID_RANGE.end()
        );
    }
}
