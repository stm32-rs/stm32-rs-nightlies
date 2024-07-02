///Register `LTDC_SRCR` reader
pub type R = crate::R<LTDC_SRCRrs>;
///Register `LTDC_SRCR` writer
pub type W = crate::W<LTDC_SRCRrs>;
///Field `IMR` reader - immediate reload This bit is set by software and cleared only by hardware after reload.
pub type IMR_R = crate::BitReader;
///Field `IMR` writer - immediate reload This bit is set by software and cleared only by hardware after reload.
pub type IMR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBR` reader - vertical blanking reload This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set).
pub type VBR_R = crate::BitReader;
///Field `VBR` writer - vertical blanking reload This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set).
pub type VBR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - immediate reload This bit is set by software and cleared only by hardware after reload.
    #[inline(always)]
    pub fn imr(&self) -> IMR_R {
        IMR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - vertical blanking reload This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set).
    #[inline(always)]
    pub fn vbr(&self) -> VBR_R {
        VBR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTDC_SRCR")
            .field("imr", &self.imr())
            .field("vbr", &self.vbr())
            .finish()
    }
}
impl W {
    ///Bit 0 - immediate reload This bit is set by software and cleared only by hardware after reload.
    #[inline(always)]
    #[must_use]
    pub fn imr(&mut self) -> IMR_W<LTDC_SRCRrs> {
        IMR_W::new(self, 0)
    }
    ///Bit 1 - vertical blanking reload This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set).
    #[inline(always)]
    #[must_use]
    pub fn vbr(&mut self) -> VBR_W<LTDC_SRCRrs> {
        VBR_W::new(self, 1)
    }
}
/**LTDC shadow reload configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_srcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_srcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_SRCR)*/
pub struct LTDC_SRCRrs;
impl crate::RegisterSpec for LTDC_SRCRrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_srcr::R`](R) reader structure
impl crate::Readable for LTDC_SRCRrs {}
///`write(|w| ..)` method takes [`ltdc_srcr::W`](W) writer structure
impl crate::Writable for LTDC_SRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_SRCR to value 0
impl crate::Resettable for LTDC_SRCRrs {
    const RESET_VALUE: u32 = 0;
}
