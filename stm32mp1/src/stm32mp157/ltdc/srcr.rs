///Register `SRCR` reader
pub type R = crate::R<SRCRrs>;
///Register `SRCR` writer
pub type W = crate::W<SRCRrs>;
///Field `IMR` reader - IMR
pub type IMR_R = crate::BitReader;
///Field `IMR` writer - IMR
pub type IMR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBR` reader - VBR
pub type VBR_R = crate::BitReader;
///Field `VBR` writer - VBR
pub type VBR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IMR
    #[inline(always)]
    pub fn imr(&self) -> IMR_R {
        IMR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VBR
    #[inline(always)]
    pub fn vbr(&self) -> VBR_R {
        VBR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRCR")
            .field("imr", &self.imr())
            .field("vbr", &self.vbr())
            .finish()
    }
}
impl W {
    ///Bit 0 - IMR
    #[inline(always)]
    pub fn imr(&mut self) -> IMR_W<'_, SRCRrs> {
        IMR_W::new(self, 0)
    }
    ///Bit 1 - VBR
    #[inline(always)]
    pub fn vbr(&mut self) -> VBR_W<'_, SRCRrs> {
        VBR_W::new(self, 1)
    }
}
/**This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR.

You can [`read`](crate::Reg::read) this register and get [`srcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LTDC:SRCR)*/
pub struct SRCRrs;
impl crate::RegisterSpec for SRCRrs {
    type Ux = u32;
}
///`read()` method returns [`srcr::R`](R) reader structure
impl crate::Readable for SRCRrs {}
///`write(|w| ..)` method takes [`srcr::W`](W) writer structure
impl crate::Writable for SRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRCR to value 0
impl crate::Resettable for SRCRrs {}
