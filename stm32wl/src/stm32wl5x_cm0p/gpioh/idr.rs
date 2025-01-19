///Register `IDR` reader
pub type R = crate::R<IDRrs>;
/**Port input data (y = 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDR3 {
    ///0: Input is logic low
    Low = 0,
    ///1: Input is logic high
    High = 1,
}
impl From<IDR3> for bool {
    #[inline(always)]
    fn from(variant: IDR3) -> Self {
        variant as u8 != 0
    }
}
///Field `IDR3` reader - Port input data (y = 0..15)
pub type IDR3_R = crate::BitReader<IDR3>;
impl IDR3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDR3 {
        match self.bits {
            false => IDR3::Low,
            true => IDR3::High,
        }
    }
    ///Input is logic low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDR3::Low
    }
    ///Input is logic high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDR3::High
    }
}
impl R {
    ///Bit 3 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr3(&self) -> IDR3_R {
        IDR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR").field("idr3", &self.idr3()).finish()
    }
}
/**GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#GPIOH:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {
    const RESET_VALUE: u32 = 0;
}
