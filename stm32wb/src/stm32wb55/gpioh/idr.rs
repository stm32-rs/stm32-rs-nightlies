///Register `IDR` reader
pub type R = crate::R<IDRrs>;
/**Port input data (y = 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPUT_DATA {
    ///0: Input is logic low
    Low = 0,
    ///1: Input is logic high
    High = 1,
}
impl From<INPUT_DATA> for bool {
    #[inline(always)]
    fn from(variant: INPUT_DATA) -> Self {
        variant as u8 != 0
    }
}
///Field `IDR0` reader - Port input data (y = 0..15)
pub type IDR0_R = crate::BitReader<INPUT_DATA>;
impl IDR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INPUT_DATA {
        match self.bits {
            false => INPUT_DATA::Low,
            true => INPUT_DATA::High,
        }
    }
    ///Input is logic low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == INPUT_DATA::Low
    }
    ///Input is logic high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == INPUT_DATA::High
    }
}
///Field `IDR1` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR1_R;
///Field `IDR3` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR3_R;
impl R {
    ///Bit 0 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr0(&self) -> IDR0_R {
        IDR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr1(&self) -> IDR1_R {
        IDR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr3(&self) -> IDR3_R {
        IDR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR")
            .field("idr0", &self.idr0())
            .field("idr3", &self.idr3())
            .field("idr1", &self.idr1())
            .finish()
    }
}
/**GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOH:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {}
