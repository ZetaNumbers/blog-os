pub struct WildcardTry;

impl<T> core::ops::FromResidual<T> for WildcardTry {
    fn from_residual(_: T) -> Self {
        WildcardTry
    }
}

// impl<T> core::ops::TryV2<T> for WildcardTry {
//     type Output = ();

//     type Residual = WildcardTry;

//     fn from_output((): ()) -> Self {
//         WildcardTry
//     }

//     fn branch(self) -> core::ops::ControlFlow<Self::Residual, Self::Output> {
//         todo!()
//     }
// }
