#[doc = "Register `REPDR` reader"]
pub type R = crate::R<REPDRrs>;
#[doc = "Register `REPDR` writer"]
pub type W = crate::W<REPDRrs>;
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
    pub fn repx(&mut self) -> REPX_W<REPDRrs> {
        REPX_W::new(self, 0)
    }
}
#[doc = "Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REPDRrs;
impl crate::RegisterSpec for REPDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`repdr::R`](R) reader structure"]
impl crate::Readable for REPDRrs {}
#[doc = "`write(|w| ..)` method takes [`repdr::W`](W) writer structure"]
impl crate::Writable for REPDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REPDR to value 0"]
impl crate::Resettable for REPDRrs {
    const RESET_VALUE: u32 = 0;
}
