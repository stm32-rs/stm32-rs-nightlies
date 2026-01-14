///Register `HISR` reader
pub type R = crate::R<HISRrs>;
///Field `FEIF4` reader - FEIF4
pub type FEIF4_R = crate::BitReader;
///Field `DMEIF4` reader - DMEIF4
pub type DMEIF4_R = crate::BitReader;
///Field `TEIF4` reader - TEIF4
pub type TEIF4_R = crate::BitReader;
///Field `HTIF4` reader - HTIF4
pub type HTIF4_R = crate::BitReader;
///Field `TCIF4` reader - TCIF4
pub type TCIF4_R = crate::BitReader;
///Field `FEIF5` reader - FEIF5
pub type FEIF5_R = crate::BitReader;
///Field `DMEIF5` reader - DMEIF5
pub type DMEIF5_R = crate::BitReader;
///Field `TEIF5` reader - TEIF5
pub type TEIF5_R = crate::BitReader;
///Field `HTIF5` reader - HTIF5
pub type HTIF5_R = crate::BitReader;
///Field `TCIF5` reader - TCIF5
pub type TCIF5_R = crate::BitReader;
///Field `FEIF6` reader - FEIF6
pub type FEIF6_R = crate::BitReader;
///Field `DMEIF6` reader - DMEIF6
pub type DMEIF6_R = crate::BitReader;
///Field `TEIF6` reader - TEIF6
pub type TEIF6_R = crate::BitReader;
///Field `HTIF6` reader - HTIF6
pub type HTIF6_R = crate::BitReader;
///Field `TCIF6` reader - TCIF6
pub type TCIF6_R = crate::BitReader;
///Field `FEIF7` reader - FEIF7
pub type FEIF7_R = crate::BitReader;
///Field `DMEIF7` reader - DMEIF7
pub type DMEIF7_R = crate::BitReader;
///Field `TEIF7` reader - TEIF7
pub type TEIF7_R = crate::BitReader;
///Field `HTIF7` reader - HTIF7
pub type HTIF7_R = crate::BitReader;
///Field `TCIF7` reader - TCIF7
pub type TCIF7_R = crate::BitReader;
impl R {
    ///Bit 0 - FEIF4
    #[inline(always)]
    pub fn feif4(&self) -> FEIF4_R {
        FEIF4_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - DMEIF4
    #[inline(always)]
    pub fn dmeif4(&self) -> DMEIF4_R {
        DMEIF4_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TEIF4
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HTIF4
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TCIF4
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FEIF5
    #[inline(always)]
    pub fn feif5(&self) -> FEIF5_R {
        FEIF5_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - DMEIF5
    #[inline(always)]
    pub fn dmeif5(&self) -> DMEIF5_R {
        DMEIF5_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TEIF5
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HTIF5
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TCIF5
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - FEIF6
    #[inline(always)]
    pub fn feif6(&self) -> FEIF6_R {
        FEIF6_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - DMEIF6
    #[inline(always)]
    pub fn dmeif6(&self) -> DMEIF6_R {
        DMEIF6_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TEIF6
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - HTIF6
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TCIF6
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - FEIF7
    #[inline(always)]
    pub fn feif7(&self) -> FEIF7_R {
        FEIF7_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - DMEIF7
    #[inline(always)]
    pub fn dmeif7(&self) -> DMEIF7_R {
        DMEIF7_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TEIF7
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - HTIF7
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TCIF7
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HISR")
            .field("feif4", &self.feif4())
            .field("dmeif4", &self.dmeif4())
            .field("teif4", &self.teif4())
            .field("htif4", &self.htif4())
            .field("tcif4", &self.tcif4())
            .field("feif5", &self.feif5())
            .field("dmeif5", &self.dmeif5())
            .field("teif5", &self.teif5())
            .field("htif5", &self.htif5())
            .field("tcif5", &self.tcif5())
            .field("feif6", &self.feif6())
            .field("dmeif6", &self.dmeif6())
            .field("teif6", &self.teif6())
            .field("htif6", &self.htif6())
            .field("tcif6", &self.tcif6())
            .field("feif7", &self.feif7())
            .field("dmeif7", &self.dmeif7())
            .field("teif7", &self.teif7())
            .field("htif7", &self.htif7())
            .field("tcif7", &self.tcif7())
            .finish()
    }
}
/**DMA high interrupt status register

You can [`read`](crate::Reg::read) this register and get [`hisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMA1:HISR)*/
pub struct HISRrs;
impl crate::RegisterSpec for HISRrs {
    type Ux = u32;
}
///`read()` method returns [`hisr::R`](R) reader structure
impl crate::Readable for HISRrs {}
///`reset()` method sets HISR to value 0
impl crate::Resettable for HISRrs {}
