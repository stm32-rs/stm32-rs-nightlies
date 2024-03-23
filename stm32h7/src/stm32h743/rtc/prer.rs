#[doc = "Register `PRER` reader"]
pub type R = crate::R<PRERrs>;
#[doc = "Register `PRER` writer"]
pub type W = crate::W<PRERrs>;
#[doc = "Field `PREDIV_S` reader - Synchronous prescaler factor This is the synchronous division factor: ck_spre frequency = ck_apre frequency/(PREDIV_S+1)"]
pub type PREDIV_S_R = crate::FieldReader<u16>;
#[doc = "Field `PREDIV_S` writer - Synchronous prescaler factor This is the synchronous division factor: ck_spre frequency = ck_apre frequency/(PREDIV_S+1)"]
pub type PREDIV_S_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 15, u16>;
#[doc = "Field `PREDIV_A` reader - Asynchronous prescaler factor This is the asynchronous division factor: ck_apre frequency = RTCCLK frequency/(PREDIV_A+1)"]
pub type PREDIV_A_R = crate::FieldReader;
#[doc = "Field `PREDIV_A` writer - Asynchronous prescaler factor This is the asynchronous division factor: ck_apre frequency = RTCCLK frequency/(PREDIV_A+1)"]
pub type PREDIV_A_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:14 - Synchronous prescaler factor This is the synchronous division factor: ck_spre frequency = ck_apre frequency/(PREDIV_S+1)"]
    #[inline(always)]
    pub fn prediv_s(&self) -> PREDIV_S_R {
        PREDIV_S_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:22 - Asynchronous prescaler factor This is the asynchronous division factor: ck_apre frequency = RTCCLK frequency/(PREDIV_A+1)"]
    #[inline(always)]
    pub fn prediv_a(&self) -> PREDIV_A_R {
        PREDIV_A_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Synchronous prescaler factor This is the synchronous division factor: ck_spre frequency = ck_apre frequency/(PREDIV_S+1)"]
    #[inline(always)]
    #[must_use]
    pub fn prediv_s(&mut self) -> PREDIV_S_W<PRERrs> {
        PREDIV_S_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - Asynchronous prescaler factor This is the asynchronous division factor: ck_apre frequency = RTCCLK frequency/(PREDIV_A+1)"]
    #[inline(always)]
    #[must_use]
    pub fn prediv_a(&mut self) -> PREDIV_A_W<PRERrs> {
        PREDIV_A_W::new(self, 16)
    }
}
#[doc = "This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRERrs;
impl crate::RegisterSpec for PRERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prer::R`](R) reader structure"]
impl crate::Readable for PRERrs {}
#[doc = "`write(|w| ..)` method takes [`prer::W`](W) writer structure"]
impl crate::Writable for PRERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRER to value 0x007f_00ff"]
impl crate::Resettable for PRERrs {
    const RESET_VALUE: u32 = 0x007f_00ff;
}
