#[doc = "Register `REPER` reader"]
pub type R = crate::R<REPERrs>;
#[doc = "Register `REPER` writer"]
pub type W = crate::W<REPERrs>;
#[doc = "Field `REPx` reader - Timerx Repetition counter value"]
pub type REPX_R = crate::FieldReader;
#[doc = "Field `REPx` writer - Timerx Repetition counter value"]
pub type REPX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timerx Repetition counter value"]
    #[inline(always)]
    pub fn repx(&self) -> REPX_R {
        REPX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timerx Repetition counter value"]
    #[inline(always)]
    #[must_use]
    pub fn repx(&mut self) -> REPX_W<REPERrs> {
        REPX_W::new(self, 0)
    }
}
#[doc = "Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REPERrs;
impl crate::RegisterSpec for REPERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reper::R`](R) reader structure"]
impl crate::Readable for REPERrs {}
#[doc = "`write(|w| ..)` method takes [`reper::W`](W) writer structure"]
impl crate::Writable for REPERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REPER to value 0"]
impl crate::Resettable for REPERrs {
    const RESET_VALUE: u32 = 0;
}
