///Register `VPCCR` reader
pub type R = crate::R<VPCCRrs>;
///Register `VPCCR` writer
pub type W = crate::W<VPCCRrs>;
///Field `VPSIZE` reader - Video packet size
pub type VPSIZE_R = crate::FieldReader<u16>;
///Field `VPSIZE` writer - Video packet size
pub type VPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - Video packet size
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VPCCR")
            .field("vpsize", &self.vpsize())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Video packet size
    #[inline(always)]
    #[must_use]
    pub fn vpsize(&mut self) -> VPSIZE_W<VPCCRrs> {
        VPSIZE_W::new(self, 0)
    }
}
/**DSI Host video packet current configuration register

You can [`read`](crate::Reg::read) this register and get [`vpccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vpccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VPCCR)*/
pub struct VPCCRrs;
impl crate::RegisterSpec for VPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vpccr::R`](R) reader structure
impl crate::Readable for VPCCRrs {}
///`write(|w| ..)` method takes [`vpccr::W`](W) writer structure
impl crate::Writable for VPCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VPCCR to value 0
impl crate::Resettable for VPCCRrs {
    const RESET_VALUE: u32 = 0;
}
