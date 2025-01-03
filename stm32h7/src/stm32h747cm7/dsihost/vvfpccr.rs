///Register `VVFPCCR` reader
pub type R = crate::R<VVFPCCRrs>;
///Register `VVFPCCR` writer
pub type W = crate::W<VVFPCCRrs>;
///Field `VFP` reader - Vertical front
pub type VFP_R = crate::FieldReader<u16>;
///Field `VFP` writer - Vertical front
pub type VFP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Vertical front
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VVFPCCR").field("vfp", &self.vfp()).finish()
    }
}
impl W {
    ///Bits 0:9 - Vertical front
    #[inline(always)]
    pub fn vfp(&mut self) -> VFP_W<VVFPCCRrs> {
        VFP_W::new(self, 0)
    }
}
/**DSI Host video VFP current configuration register

You can [`read`](crate::Reg::read) this register and get [`vvfpccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvfpccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#DSIHOST:VVFPCCR)*/
pub struct VVFPCCRrs;
impl crate::RegisterSpec for VVFPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vvfpccr::R`](R) reader structure
impl crate::Readable for VVFPCCRrs {}
///`write(|w| ..)` method takes [`vvfpccr::W`](W) writer structure
impl crate::Writable for VVFPCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VVFPCCR to value 0
impl crate::Resettable for VVFPCCRrs {
    const RESET_VALUE: u32 = 0;
}
