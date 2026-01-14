///Register `GPIOZ_IDR` reader
pub type R = crate::R<GPIOZ_IDRrs>;
///Field `IDR0` reader - IDR0
pub type IDR0_R = crate::BitReader;
///Field `IDR1` reader - IDR1
pub type IDR1_R = crate::BitReader;
///Field `IDR2` reader - IDR2
pub type IDR2_R = crate::BitReader;
///Field `IDR3` reader - IDR3
pub type IDR3_R = crate::BitReader;
///Field `IDR4` reader - IDR4
pub type IDR4_R = crate::BitReader;
///Field `IDR5` reader - IDR5
pub type IDR5_R = crate::BitReader;
///Field `IDR6` reader - IDR6
pub type IDR6_R = crate::BitReader;
///Field `IDR7` reader - IDR7
pub type IDR7_R = crate::BitReader;
///Field `IDR8` reader - IDR8
pub type IDR8_R = crate::BitReader;
///Field `IDR9` reader - IDR9
pub type IDR9_R = crate::BitReader;
///Field `IDR10` reader - IDR10
pub type IDR10_R = crate::BitReader;
///Field `IDR11` reader - IDR11
pub type IDR11_R = crate::BitReader;
///Field `IDR12` reader - IDR12
pub type IDR12_R = crate::BitReader;
///Field `IDR13` reader - IDR13
pub type IDR13_R = crate::BitReader;
///Field `IDR14` reader - IDR14
pub type IDR14_R = crate::BitReader;
///Field `IDR15` reader - IDR15
pub type IDR15_R = crate::BitReader;
impl R {
    ///Bit 0 - IDR0
    #[inline(always)]
    pub fn idr0(&self) -> IDR0_R {
        IDR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IDR1
    #[inline(always)]
    pub fn idr1(&self) -> IDR1_R {
        IDR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IDR2
    #[inline(always)]
    pub fn idr2(&self) -> IDR2_R {
        IDR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IDR3
    #[inline(always)]
    pub fn idr3(&self) -> IDR3_R {
        IDR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDR4
    #[inline(always)]
    pub fn idr4(&self) -> IDR4_R {
        IDR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IDR5
    #[inline(always)]
    pub fn idr5(&self) -> IDR5_R {
        IDR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IDR6
    #[inline(always)]
    pub fn idr6(&self) -> IDR6_R {
        IDR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IDR7
    #[inline(always)]
    pub fn idr7(&self) -> IDR7_R {
        IDR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IDR8
    #[inline(always)]
    pub fn idr8(&self) -> IDR8_R {
        IDR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - IDR9
    #[inline(always)]
    pub fn idr9(&self) -> IDR9_R {
        IDR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - IDR10
    #[inline(always)]
    pub fn idr10(&self) -> IDR10_R {
        IDR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - IDR11
    #[inline(always)]
    pub fn idr11(&self) -> IDR11_R {
        IDR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - IDR12
    #[inline(always)]
    pub fn idr12(&self) -> IDR12_R {
        IDR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - IDR13
    #[inline(always)]
    pub fn idr13(&self) -> IDR13_R {
        IDR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - IDR14
    #[inline(always)]
    pub fn idr14(&self) -> IDR14_R {
        IDR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - IDR15
    #[inline(always)]
    pub fn idr15(&self) -> IDR15_R {
        IDR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOZ_IDR")
            .field("idr0", &self.idr0())
            .field("idr1", &self.idr1())
            .field("idr2", &self.idr2())
            .field("idr3", &self.idr3())
            .field("idr4", &self.idr4())
            .field("idr5", &self.idr5())
            .field("idr6", &self.idr6())
            .field("idr7", &self.idr7())
            .field("idr8", &self.idr8())
            .field("idr9", &self.idr9())
            .field("idr10", &self.idr10())
            .field("idr11", &self.idr11())
            .field("idr12", &self.idr12())
            .field("idr13", &self.idr13())
            .field("idr14", &self.idr14())
            .field("idr15", &self.idr15())
            .finish()
    }
}
/**GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpioz_idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOZ:GPIOZ_IDR)*/
pub struct GPIOZ_IDRrs;
impl crate::RegisterSpec for GPIOZ_IDRrs {
    type Ux = u32;
}
///`read()` method returns [`gpioz_idr::R`](R) reader structure
impl crate::Readable for GPIOZ_IDRrs {}
///`reset()` method sets GPIOZ_IDR to value 0
impl crate::Resettable for GPIOZ_IDRrs {}
