use crate::bootstrap::Bootstrap;


fn main() -> Result<()> {
    Bootstrap::new(std::env::args())
        .build()?
        .run()
}