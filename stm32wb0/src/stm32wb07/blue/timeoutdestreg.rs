///Register `TIMEOUTDESTREG` reader
pub type R = crate::R<TIMEOUTDESTREGrs>;
///Register `TIMEOUTDESTREG` writer
pub type W = crate::W<TIMEOUTDESTREGrs>;
///Field `DESTINATION` reader - Timeout timer Destination
pub type DESTINATION_R = crate::FieldReader;
///Field `DESTINATION` writer - Timeout timer Destination
pub type DESTINATION_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Timeout timer Destination
    #[inline(always)]
    pub fn destination(&self) -> DESTINATION_R {
        DESTINATION_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUTDESTREG")
            .field("destination", &self.destination())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Timeout timer Destination
    #[inline(always)]
    pub fn destination(&mut self) -> DESTINATION_W<'_, TIMEOUTDESTREGrs> {
        DESTINATION_W::new(self, 0)
    }
}
/**TIMEOUTDESTREG register

You can [`read`](crate::Reg::read) this register and get [`timeoutdestreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeoutdestreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:TIMEOUTDESTREG)*/
pub struct TIMEOUTDESTREGrs;
impl crate::RegisterSpec for TIMEOUTDESTREGrs {
    type Ux = u32;
}
///`read()` method returns [`timeoutdestreg::R`](R) reader structure
impl crate::Readable for TIMEOUTDESTREGrs {}
///`write(|w| ..)` method takes [`timeoutdestreg::W`](W) writer structure
impl crate::Writable for TIMEOUTDESTREGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIMEOUTDESTREG to value 0
impl crate::Resettable for TIMEOUTDESTREGrs {}
