///Register `VCCR` reader
pub type R = crate::R<VCCRrs>;
///Register `VCCR` writer
pub type W = crate::W<VCCRrs>;
///Field `NUMC` reader - Number of Chunks
pub type NUMC_R = crate::FieldReader<u16>;
///Field `NUMC` writer - Number of Chunks
pub type NUMC_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - Number of Chunks
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VCCR").field("numc", &self.numc()).finish()
    }
}
impl W {
    ///Bits 0:12 - Number of Chunks
    #[inline(always)]
    pub fn numc(&mut self) -> NUMC_W<'_, VCCRrs> {
        NUMC_W::new(self, 0)
    }
}
/**DSI Host Video Chunks Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#DSI:VCCR)*/
pub struct VCCRrs;
impl crate::RegisterSpec for VCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vccr::R`](R) reader structure
impl crate::Readable for VCCRrs {}
///`write(|w| ..)` method takes [`vccr::W`](W) writer structure
impl crate::Writable for VCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VCCR to value 0
impl crate::Resettable for VCCRrs {}
