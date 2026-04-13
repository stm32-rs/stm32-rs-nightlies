///Register `PFCR` reader
pub type R = crate::R<PFCRrs>;
///Register `PFCR` writer
pub type W = crate::W<PFCRrs>;
///Field `CCFR` reader - Configuration clock frequency range selection
pub type CCFR_R = crate::FieldReader;
///Field `CCFR` writer - Configuration clock frequency range selection
pub type CCFR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `HSFR` reader - PHY high-speed frequency range selection
pub type HSFR_R = crate::FieldReader;
///Field `HSFR` writer - PHY high-speed frequency range selection
pub type HSFR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DLD` reader - Data lane direction of lane 0
pub type DLD_R = crate::BitReader;
///Field `DLD` writer - Data lane direction of lane 0
pub type DLD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Configuration clock frequency range selection
    #[inline(always)]
    pub fn ccfr(&self) -> CCFR_R {
        CCFR_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:14 - PHY high-speed frequency range selection
    #[inline(always)]
    pub fn hsfr(&self) -> HSFR_R {
        HSFR_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - Data lane direction of lane 0
    #[inline(always)]
    pub fn dld(&self) -> DLD_R {
        DLD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PFCR")
            .field("ccfr", &self.ccfr())
            .field("hsfr", &self.hsfr())
            .field("dld", &self.dld())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Configuration clock frequency range selection
    #[inline(always)]
    pub fn ccfr(&mut self) -> CCFR_W<'_, PFCRrs> {
        CCFR_W::new(self, 0)
    }
    ///Bits 8:14 - PHY high-speed frequency range selection
    #[inline(always)]
    pub fn hsfr(&mut self) -> HSFR_W<'_, PFCRrs> {
        HSFR_W::new(self, 8)
    }
    ///Bit 16 - Data lane direction of lane 0
    #[inline(always)]
    pub fn dld(&mut self) -> DLD_W<'_, PFCRrs> {
        DLD_W::new(self, 16)
    }
}
/**CSI PHY frequency control register

You can [`read`](crate::Reg::read) this register and get [`pfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#CSI:PFCR)*/
pub struct PFCRrs;
impl crate::RegisterSpec for PFCRrs {
    type Ux = u32;
}
///`read()` method returns [`pfcr::R`](R) reader structure
impl crate::Readable for PFCRrs {}
///`write(|w| ..)` method takes [`pfcr::W`](W) writer structure
impl crate::Writable for PFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PFCR to value 0x17
impl crate::Resettable for PFCRrs {
    const RESET_VALUE: u32 = 0x17;
}
