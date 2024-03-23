#[doc = "Register `CHPAR` reader"]
pub type R = crate::R<CHPARrs>;
#[doc = "Register `CHPAR` writer"]
pub type W = crate::W<CHPARrs>;
#[doc = "Field `CARFRQ` reader - Timerx carrier frequency value"]
pub type CARFRQ_R = crate::FieldReader;
#[doc = "Field `CARFRQ` writer - Timerx carrier frequency value"]
pub type CARFRQ_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `CARDTY` reader - Timerx chopper duty cycle value"]
pub type CARDTY_R = crate::FieldReader;
#[doc = "Field `CARDTY` writer - Timerx chopper duty cycle value"]
pub type CARDTY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `STRTPW` reader - STRTPW"]
pub type STRTPW_R = crate::FieldReader;
#[doc = "Field `STRTPW` writer - STRTPW"]
pub type STRTPW_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn carfrq(&self) -> CARFRQ_R {
        CARFRQ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    pub fn cardty(&self) -> CARDTY_R {
        CARDTY_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    pub fn strtpw(&self) -> STRTPW_R {
        STRTPW_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    #[must_use]
    pub fn carfrq(&mut self) -> CARFRQ_W<CHPARrs> {
        CARFRQ_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    #[must_use]
    pub fn cardty(&mut self) -> CARDTY_W<CHPARrs> {
        CARDTY_W::new(self, 4)
    }
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    #[must_use]
    pub fn strtpw(&mut self) -> STRTPW_W<CHPARrs> {
        STRTPW_W::new(self, 7)
    }
}
#[doc = "Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHPARrs;
impl crate::RegisterSpec for CHPARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chpar::R`](R) reader structure"]
impl crate::Readable for CHPARrs {}
#[doc = "`write(|w| ..)` method takes [`chpar::W`](W) writer structure"]
impl crate::Writable for CHPARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHPAR to value 0"]
impl crate::Resettable for CHPARrs {
    const RESET_VALUE: u32 = 0;
}
