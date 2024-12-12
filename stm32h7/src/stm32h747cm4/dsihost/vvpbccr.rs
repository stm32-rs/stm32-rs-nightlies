///Register `VVPBCCR` reader
pub type R = crate::R<VVPBCCRrs>;
///Register `VVPBCCR` writer
pub type W = crate::W<VVPBCCRrs>;
///Field `VBP` reader - Vertical back
pub type VBP_R = crate::FieldReader<u16>;
///Field `VBP` writer - Vertical back
pub type VBP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Vertical back
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VVPBCCR").field("vbp", &self.vbp()).finish()
    }
}
impl W {
    ///Bits 0:9 - Vertical back
    #[inline(always)]
    pub fn vbp(&mut self) -> VBP_W<VVPBCCRrs> {
        VBP_W::new(self, 0)
    }
}
/**DSI Host video VBP current configuration register

You can [`read`](crate::Reg::read) this register and get [`vvpbccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvpbccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VVPBCCR)*/
pub struct VVPBCCRrs;
impl crate::RegisterSpec for VVPBCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vvpbccr::R`](R) reader structure
impl crate::Readable for VVPBCCRrs {}
///`write(|w| ..)` method takes [`vvpbccr::W`](W) writer structure
impl crate::Writable for VVPBCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VVPBCCR to value 0
impl crate::Resettable for VVPBCCRrs {
    const RESET_VALUE: u32 = 0;
}
