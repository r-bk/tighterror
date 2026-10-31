fn main() -> Result<(), tighterror_build::errors::TbError> {
    tighterror_build::BuildOptions::new().test(true).build()
}
