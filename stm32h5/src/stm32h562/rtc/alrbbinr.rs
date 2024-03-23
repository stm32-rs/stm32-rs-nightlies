#[doc = "Register `ALRBBINR` reader"]
pub type R = crate::R<ALRBBINRrs>;
#[doc = "Register `ALRBBINR` writer"]
pub type W = crate::W<ALRBBINRrs>;
#[doc = "Field `SS` reader - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\]
is the mirror of SS\\[14:0\\]
in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR."]
pub type SS_R = crate::FieldReader<u32>;
#[doc = "Field `SS` writer - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\]
is the mirror of SS\\[14:0\\]
in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR."]
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\]
is the mirror of SS\\[14:0\\]
in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR."]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\]
is the mirror of SS\\[14:0\\]
in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR."]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<ALRBBINRrs> {
        SS_W::new(self, 0)
    }
}
#[doc = "RTC alarm B binary mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrbbinr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrbbinr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRBBINRrs;
impl crate::RegisterSpec for ALRBBINRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrbbinr::R`](R) reader structure"]
impl crate::Readable for ALRBBINRrs {}
#[doc = "`write(|w| ..)` method takes [`alrbbinr::W`](W) writer structure"]
impl crate::Writable for ALRBBINRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALRBBINR to value 0"]
impl crate::Resettable for ALRBBINRrs {
    const RESET_VALUE: u32 = 0;
}
