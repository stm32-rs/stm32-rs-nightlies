#[doc = "Register `CHPCR` reader"]
pub type R = crate::R<CHPCRrs>;
#[doc = "Register `CHPCR` writer"]
pub type W = crate::W<CHPCRrs>;
#[doc = "Field `CHPFRQ` reader - Timerx carrier frequency value"]
pub type CHPFRQ_R = crate::FieldReader;
#[doc = "Field `CHPFRQ` writer - Timerx carrier frequency value"]
pub type CHPFRQ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHPDTY` reader - Timerx chopper duty cycle value"]
pub type CHPDTY_R = crate::FieldReader;
#[doc = "Field `CHPDTY` writer - Timerx chopper duty cycle value"]
pub type CHPDTY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STRTPW` reader - STRTPW"]
pub type STRTPW_R = crate::FieldReader;
#[doc = "Field `STRTPW` writer - STRTPW"]
pub type STRTPW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn chpfrq(&self) -> CHPFRQ_R {
        CHPFRQ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    pub fn chpdty(&self) -> CHPDTY_R {
        CHPDTY_R::new(((self.bits >> 4) & 7) as u8)
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
    pub fn chpfrq(&mut self) -> CHPFRQ_W<CHPCRrs> {
        CHPFRQ_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    #[must_use]
    pub fn chpdty(&mut self) -> CHPDTY_W<CHPCRrs> {
        CHPDTY_W::new(self, 4)
    }
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    #[must_use]
    pub fn strtpw(&mut self) -> STRTPW_W<CHPCRrs> {
        STRTPW_W::new(self, 7)
    }
}
#[doc = "Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHPCRrs;
impl crate::RegisterSpec for CHPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chpcr::R`](R) reader structure"]
impl crate::Readable for CHPCRrs {}
#[doc = "`write(|w| ..)` method takes [`chpcr::W`](W) writer structure"]
impl crate::Writable for CHPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHPCR to value 0"]
impl crate::Resettable for CHPCRrs {
    const RESET_VALUE: u32 = 0;
}
