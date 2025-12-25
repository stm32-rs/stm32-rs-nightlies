///Register `IDR` reader
pub type R = crate::R<IDRrs>;
///Field `IDR0` reader - Port input data (y = 0..15)
pub type IDR0_R = crate::BitReader;
///Field `IDR1` reader - Port input data (y = 0..15)
pub type IDR1_R = crate::BitReader;
///Field `IDR2` reader - Port input data (y = 0..15)
pub type IDR2_R = crate::BitReader;
///Field `IDR3` reader - Port input data (y = 0..15)
pub type IDR3_R = crate::BitReader;
///Field `IDR4` reader - Port input data (y = 0..15)
pub type IDR4_R = crate::BitReader;
///Field `IDR5` reader - Port input data (y = 0..15)
pub type IDR5_R = crate::BitReader;
///Field `IDR6` reader - Port input data (y = 0..15)
pub type IDR6_R = crate::BitReader;
///Field `IDR7` reader - Port input data (y = 0..15)
pub type IDR7_R = crate::BitReader;
///Field `IDR8` reader - Port input data (y = 0..15)
pub type IDR8_R = crate::BitReader;
///Field `IDR9` reader - Port input data (y = 0..15)
pub type IDR9_R = crate::BitReader;
///Field `IDR10` reader - Port input data (y = 0..15)
pub type IDR10_R = crate::BitReader;
///Field `IDR11` reader - Port input data (y = 0..15)
pub type IDR11_R = crate::BitReader;
///Field `IDR12` reader - Port input data (y = 0..15)
pub type IDR12_R = crate::BitReader;
///Field `IDR13` reader - Port input data (y = 0..15)
pub type IDR13_R = crate::BitReader;
///Field `IDR14` reader - Port input data (y = 0..15)
pub type IDR14_R = crate::BitReader;
///Field `IDR15` reader - Port input data (y = 0..15)
pub type IDR15_R = crate::BitReader;
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
            .field("idr0", &self.idr0())
            .finish()
    }
}
/**GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#GPIOB:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {}
