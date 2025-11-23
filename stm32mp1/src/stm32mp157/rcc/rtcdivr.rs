///Register `RTCDIVR` reader
pub type R = crate::R<RTCDIVRrs>;
///Register `RTCDIVR` writer
pub type W = crate::W<RTCDIVRrs>;
///Field `RTCDIV` reader - RTCDIV
pub type RTCDIV_R = crate::FieldReader;
///Field `RTCDIV` writer - RTCDIV
pub type RTCDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - RTCDIV
    #[inline(always)]
    pub fn rtcdiv(&self) -> RTCDIV_R {
        RTCDIV_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCDIVR")
            .field("rtcdiv", &self.rtcdiv())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - RTCDIV
    #[inline(always)]
    pub fn rtcdiv(&mut self) -> RTCDIV_W<'_, RTCDIVRrs> {
        RTCDIV_W::new(self, 0)
    }
}
/**This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`rtcdivr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcdivr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:RTCDIVR)*/
pub struct RTCDIVRrs;
impl crate::RegisterSpec for RTCDIVRrs {
    type Ux = u32;
}
///`read()` method returns [`rtcdivr::R`](R) reader structure
impl crate::Readable for RTCDIVRrs {}
///`write(|w| ..)` method takes [`rtcdivr::W`](W) writer structure
impl crate::Writable for RTCDIVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTCDIVR to value 0
impl crate::Resettable for RTCDIVRrs {}
