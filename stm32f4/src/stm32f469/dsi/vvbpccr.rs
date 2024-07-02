///Register `VVBPCCR` reader
pub type R = crate::R<VVBPCCRrs>;
///Register `VVBPCCR` writer
pub type W = crate::W<VVBPCCRrs>;
///Field `VBP` reader - Vertical Back-Porch duration
pub type VBP_R = crate::FieldReader<u16>;
///Field `VBP` writer - Vertical Back-Porch duration
pub type VBP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Vertical Back-Porch duration
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VVBPCCR").field("vbp", &self.vbp()).finish()
    }
}
impl W {
    ///Bits 0:9 - Vertical Back-Porch duration
    #[inline(always)]
    #[must_use]
    pub fn vbp(&mut self) -> VBP_W<VVBPCCRrs> {
        VBP_W::new(self, 0)
    }
}
/**DSI Host Video VBP Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vvbpccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvbpccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:VVBPCCR)*/
pub struct VVBPCCRrs;
impl crate::RegisterSpec for VVBPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vvbpccr::R`](R) reader structure
impl crate::Readable for VVBPCCRrs {}
///`write(|w| ..)` method takes [`vvbpccr::W`](W) writer structure
impl crate::Writable for VVBPCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VVBPCCR to value 0
impl crate::Resettable for VVBPCCRrs {
    const RESET_VALUE: u32 = 0;
}
