///Register `IDR` reader
pub type R = crate::R<IDRrs>;
/**Port input data (y = 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDR0 {
    ///0: Input is logic low
    Low = 0,
    ///1: Input is logic high
    High = 1,
}
impl From<IDR0> for bool {
    #[inline(always)]
    fn from(variant: IDR0) -> Self {
        variant as u8 != 0
    }
}
///Field `IDR0` reader - Port input data (y = 0..15)
pub type IDR0_R = crate::BitReader<IDR0>;
impl IDR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDR0 {
        match self.bits {
            false => IDR0::Low,
            true => IDR0::High,
        }
    }
    ///Input is logic low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDR0::Low
    }
    ///Input is logic high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDR0::High
    }
}
///Field `IDR1` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR1_R;
///Field `IDR2` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR2_R;
///Field `IDR3` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR3_R;
///Field `IDR4` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR4_R;
///Field `IDR5` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR5_R;
///Field `IDR6` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR6_R;
///Field `IDR7` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR7_R;
///Field `IDR8` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR8_R;
///Field `IDR9` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR9_R;
///Field `IDR10` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR10_R;
///Field `IDR11` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR11_R;
///Field `IDR12` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR12_R;
///Field `IDR13` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR13_R;
///Field `IDR14` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR14_R;
///Field `IDR15` reader - Port input data (y = 0..15)
pub use IDR0_R as IDR15_R;
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
    ///Bit 2 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr2(&self) -> IDR2_R {
        IDR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr3(&self) -> IDR3_R {
        IDR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr4(&self) -> IDR4_R {
        IDR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr5(&self) -> IDR5_R {
        IDR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr6(&self) -> IDR6_R {
        IDR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr7(&self) -> IDR7_R {
        IDR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr8(&self) -> IDR8_R {
        IDR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr9(&self) -> IDR9_R {
        IDR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr10(&self) -> IDR10_R {
        IDR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr11(&self) -> IDR11_R {
        IDR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr12(&self) -> IDR12_R {
        IDR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr13(&self) -> IDR13_R {
        IDR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr14(&self) -> IDR14_R {
        IDR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr15(&self) -> IDR15_R {
        IDR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR")
            .field("idr0", &self.idr0())
            .field("idr15", &self.idr15())
            .field("idr14", &self.idr14())
            .field("idr13", &self.idr13())
            .field("idr12", &self.idr12())
            .field("idr11", &self.idr11())
            .field("idr10", &self.idr10())
            .field("idr9", &self.idr9())
            .field("idr8", &self.idr8())
            .field("idr7", &self.idr7())
            .field("idr6", &self.idr6())
            .field("idr5", &self.idr5())
            .field("idr4", &self.idr4())
            .field("idr3", &self.idr3())
            .field("idr2", &self.idr2())
            .field("idr1", &self.idr1())
            .finish()
    }
}
/**GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GPIOA:IDR)*/
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
