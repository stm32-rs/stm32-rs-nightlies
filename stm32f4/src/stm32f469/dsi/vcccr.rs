///Register `VCCCR` reader
pub type R = crate::R<VCCCRrs>;
///Register `VCCCR` writer
pub type W = crate::W<VCCCRrs>;
///Field `NUMC` reader - Number of Chunks
pub type NUMC_R = crate::FieldReader<u16>;
///Field `NUMC` writer - Number of Chunks
pub type NUMC_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - Number of Chunks
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VCCCR").field("numc", &self.numc()).finish()
    }
}
impl W {
    ///Bits 0:13 - Number of Chunks
    #[inline(always)]
    pub fn numc(&mut self) -> NUMC_W<VCCCRrs> {
        NUMC_W::new(self, 0)
    }
}
/**DSI Host Video Chunks Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vcccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vcccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:VCCCR)*/
pub struct VCCCRrs;
impl crate::RegisterSpec for VCCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vcccr::R`](R) reader structure
impl crate::Readable for VCCCRrs {}
///`write(|w| ..)` method takes [`vcccr::W`](W) writer structure
impl crate::Writable for VCCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VCCCR to value 0
impl crate::Resettable for VCCCRrs {
    const RESET_VALUE: u32 = 0;
}
