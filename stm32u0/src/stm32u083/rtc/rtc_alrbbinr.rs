///Register `RTC_ALRBBINR` reader
pub type R = crate::R<RTC_ALRBBINRrs>;
///Register `RTC_ALRBBINR` writer
pub type W = crate::W<RTC_ALRBBINRrs>;
/**Field `SS` reader - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\[14:0\]
is the mirror of SS\[14:0\]
in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR.*/
pub type SS_R = crate::FieldReader<u32>;
/**Field `SS` writer - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\[14:0\]
is the mirror of SS\[14:0\]
in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR.*/
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    /**Bits 0:31 - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\[14:0\]
    is the mirror of SS\[14:0\]
    in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR.*/
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_ALRBBINR")
            .field("ss", &self.ss())
            .finish()
    }
}
impl W {
    /**Bits 0:31 - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\[14:0\]
    is the mirror of SS\[14:0\]
    in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR.*/
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W<RTC_ALRBBINRrs> {
        SS_W::new(self, 0)
    }
}
/**RTC alarm B binary mode register

You can [`read`](crate::Reg::read) this register and get [`rtc_alrbbinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrbbinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RTC:RTC_ALRBBINR)*/
pub struct RTC_ALRBBINRrs;
impl crate::RegisterSpec for RTC_ALRBBINRrs {
    type Ux = u32;
}
///`read()` method returns [`rtc_alrbbinr::R`](R) reader structure
impl crate::Readable for RTC_ALRBBINRrs {}
///`write(|w| ..)` method takes [`rtc_alrbbinr::W`](W) writer structure
impl crate::Writable for RTC_ALRBBINRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RTC_ALRBBINR to value 0
impl crate::Resettable for RTC_ALRBBINRrs {
    const RESET_VALUE: u32 = 0;
}
