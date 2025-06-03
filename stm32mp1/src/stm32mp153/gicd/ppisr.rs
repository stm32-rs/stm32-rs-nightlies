///Register `PPISR` reader
pub type R = crate::R<PPISRrs>;
///Field `PPI6` reader - PPI6
pub type PPI6_R = crate::BitReader;
///Field `PPI5` reader - PPI5
pub type PPI5_R = crate::BitReader;
///Field `PPI4` reader - PPI4
pub type PPI4_R = crate::BitReader;
///Field `PPI0` reader - PPI0
pub type PPI0_R = crate::BitReader;
///Field `PPI1` reader - PPI1
pub type PPI1_R = crate::BitReader;
///Field `PPI2` reader - PPI2
pub type PPI2_R = crate::BitReader;
///Field `PPI3` reader - PPI3
pub type PPI3_R = crate::BitReader;
impl R {
    ///Bit 9 - PPI6
    #[inline(always)]
    pub fn ppi6(&self) -> PPI6_R {
        PPI6_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PPI5
    #[inline(always)]
    pub fn ppi5(&self) -> PPI5_R {
        PPI5_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PPI4
    #[inline(always)]
    pub fn ppi4(&self) -> PPI4_R {
        PPI4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PPI0
    #[inline(always)]
    pub fn ppi0(&self) -> PPI0_R {
        PPI0_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - PPI1
    #[inline(always)]
    pub fn ppi1(&self) -> PPI1_R {
        PPI1_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PPI2
    #[inline(always)]
    pub fn ppi2(&self) -> PPI2_R {
        PPI2_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - PPI3
    #[inline(always)]
    pub fn ppi3(&self) -> PPI3_R {
        PPI3_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPISR")
            .field("ppi6", &self.ppi6())
            .field("ppi5", &self.ppi5())
            .field("ppi4", &self.ppi4())
            .field("ppi0", &self.ppi0())
            .field("ppi1", &self.ppi1())
            .field("ppi2", &self.ppi2())
            .field("ppi3", &self.ppi3())
            .finish()
    }
}
/**GICD private peripheral interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ppisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:PPISR)*/
pub struct PPISRrs;
impl crate::RegisterSpec for PPISRrs {
    type Ux = u32;
}
///`read()` method returns [`ppisr::R`](R) reader structure
impl crate::Readable for PPISRrs {}
///`reset()` method sets PPISR to value 0
impl crate::Resettable for PPISRrs {}
