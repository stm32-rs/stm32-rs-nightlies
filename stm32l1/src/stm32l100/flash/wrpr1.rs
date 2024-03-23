#[doc = "Register `WRPR1` reader"]
pub type R = crate::R<WRPR1rs>;
#[doc = "Register `WRPR1` writer"]
pub type W = crate::W<WRPR1rs>;
#[doc = "Field `WRP1` reader - Write protection"]
pub type WRP1_R = crate::FieldReader<u32>;
#[doc = "Field `WRP1` writer - Write protection"]
pub type WRP1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write protection"]
    #[inline(always)]
    pub fn wrp1(&self) -> WRP1_R {
        WRP1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1(&mut self) -> WRP1_W<WRPR1rs> {
        WRP1_W::new(self, 0)
    }
}
#[doc = "Write protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPR1rs;
impl crate::RegisterSpec for WRPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpr1::R`](R) reader structure"]
impl crate::Readable for WRPR1rs {}
#[doc = "`write(|w| ..)` method takes [`wrpr1::W`](W) writer structure"]
impl crate::Writable for WRPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRPR1 to value 0"]
impl crate::Resettable for WRPR1rs {
    const RESET_VALUE: u32 = 0;
}
