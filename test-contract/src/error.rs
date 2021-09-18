use minicbor::Encode;

#[derive(Clone, Debug, Encode)]
pub enum TestError {
    #[n(0)]
    DivByZero,
}
