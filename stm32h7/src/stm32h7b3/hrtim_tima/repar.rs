#[doc = "Register `REPAR` reader"]
pub type R = crate::R<REPARrs>;
#[doc = "Register `REPAR` writer"]
pub type W = crate::W<REPARrs>;
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
    pub fn repx(&mut self) -> REPX_W<REPARrs> {
        REPX_W::new(self, 0)
    }
}
#[doc = "Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REPARrs;
impl crate::RegisterSpec for REPARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`repar::R`](R) reader structure"]
impl crate::Readable for REPARrs {}
#[doc = "`write(|w| ..)` method takes [`repar::W`](W) writer structure"]
impl crate::Writable for REPARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REPAR to value 0"]
impl crate::Resettable for REPARrs {
    const RESET_VALUE: u32 = 0;
}
