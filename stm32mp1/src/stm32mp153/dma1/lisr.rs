///Register `LISR` reader
pub type R = crate::R<LISRrs>;
///Field `FEIF0` reader - FEIF0
pub type FEIF0_R = crate::BitReader;
///Field `DMEIF0` reader - DMEIF0
pub type DMEIF0_R = crate::BitReader;
///Field `TEIF0` reader - TEIF0
pub type TEIF0_R = crate::BitReader;
///Field `HTIF0` reader - HTIF0
pub type HTIF0_R = crate::BitReader;
///Field `TCIF0` reader - TCIF0
pub type TCIF0_R = crate::BitReader;
///Field `FEIF1` reader - FEIF1
pub type FEIF1_R = crate::BitReader;
///Field `DMEIF1` reader - DMEIF1
pub type DMEIF1_R = crate::BitReader;
///Field `TEIF1` reader - TEIF1
pub type TEIF1_R = crate::BitReader;
///Field `HTIF1` reader - HTIF1
pub type HTIF1_R = crate::BitReader;
///Field `TCIF1` reader - TCIF1
pub type TCIF1_R = crate::BitReader;
///Field `FEIF2` reader - FEIF2
pub type FEIF2_R = crate::BitReader;
///Field `DMEIF2` reader - DMEIF2
pub type DMEIF2_R = crate::BitReader;
///Field `TEIF2` reader - TEIF2
pub type TEIF2_R = crate::BitReader;
///Field `HTIF2` reader - HTIF2
pub type HTIF2_R = crate::BitReader;
///Field `TCIF2` reader - TCIF2
pub type TCIF2_R = crate::BitReader;
///Field `FEIF3` reader - FEIF3
pub type FEIF3_R = crate::BitReader;
///Field `DMEIF3` reader - DMEIF3
pub type DMEIF3_R = crate::BitReader;
///Field `TEIF3` reader - TEIF3
pub type TEIF3_R = crate::BitReader;
///Field `HTIF3` reader - HTIF3
pub type HTIF3_R = crate::BitReader;
///Field `TCIF3` reader - TCIF3
pub type TCIF3_R = crate::BitReader;
impl R {
    ///Bit 0 - FEIF0
    #[inline(always)]
    pub fn feif0(&self) -> FEIF0_R {
        FEIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - DMEIF0
    #[inline(always)]
    pub fn dmeif0(&self) -> DMEIF0_R {
        DMEIF0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TEIF0
    #[inline(always)]
    pub fn teif0(&self) -> TEIF0_R {
        TEIF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HTIF0
    #[inline(always)]
    pub fn htif0(&self) -> HTIF0_R {
        HTIF0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TCIF0
    #[inline(always)]
    pub fn tcif0(&self) -> TCIF0_R {
        TCIF0_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FEIF1
    #[inline(always)]
    pub fn feif1(&self) -> FEIF1_R {
        FEIF1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - DMEIF1
    #[inline(always)]
    pub fn dmeif1(&self) -> DMEIF1_R {
        DMEIF1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TEIF1
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HTIF1
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TCIF1
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - FEIF2
    #[inline(always)]
    pub fn feif2(&self) -> FEIF2_R {
        FEIF2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - DMEIF2
    #[inline(always)]
    pub fn dmeif2(&self) -> DMEIF2_R {
        DMEIF2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TEIF2
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - HTIF2
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TCIF2
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - FEIF3
    #[inline(always)]
    pub fn feif3(&self) -> FEIF3_R {
        FEIF3_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - DMEIF3
    #[inline(always)]
    pub fn dmeif3(&self) -> DMEIF3_R {
        DMEIF3_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TEIF3
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - HTIF3
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TCIF3
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LISR")
            .field("feif0", &self.feif0())
            .field("dmeif0", &self.dmeif0())
            .field("teif0", &self.teif0())
            .field("htif0", &self.htif0())
            .field("tcif0", &self.tcif0())
            .field("feif1", &self.feif1())
            .field("dmeif1", &self.dmeif1())
            .field("teif1", &self.teif1())
            .field("htif1", &self.htif1())
            .field("tcif1", &self.tcif1())
            .field("feif2", &self.feif2())
            .field("dmeif2", &self.dmeif2())
            .field("teif2", &self.teif2())
            .field("htif2", &self.htif2())
            .field("tcif2", &self.tcif2())
            .field("feif3", &self.feif3())
            .field("dmeif3", &self.dmeif3())
            .field("teif3", &self.teif3())
            .field("htif3", &self.htif3())
            .field("tcif3", &self.tcif3())
            .finish()
    }
}
/**DMA low interrupt status register

You can [`read`](crate::Reg::read) this register and get [`lisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMA1:LISR)*/
pub struct LISRrs;
impl crate::RegisterSpec for LISRrs {
    type Ux = u32;
}
///`read()` method returns [`lisr::R`](R) reader structure
impl crate::Readable for LISRrs {}
///`reset()` method sets LISR to value 0
impl crate::Resettable for LISRrs {}
