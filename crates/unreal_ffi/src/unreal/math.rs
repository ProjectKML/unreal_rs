use std::fmt::Debug;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct TVector<T: Clone + Copy + Debug + Default> {
    pub X: T,
    pub Y: T,
    pub Z: T,
}

pub type FVector = TVector<f64>;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct TRotator<T: Clone + Copy + Debug + Default> {
    pub Pitch: T,
    pub Yaw: T,
    pub Roll: T,
}

pub type FRotator = TRotator<f64>;
