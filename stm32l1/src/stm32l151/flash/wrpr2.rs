#[doc = "Register `WRPR2` reader"]
pub type R = crate::R<WRPR2rs>;
#[doc = "Register `WRPR2` writer"]
pub type W = crate::W<WRPR2rs>;
#[doc = "Field `WRP2` reader - WRP2"]
pub type WRP2_R = crate::FieldReader<u32>;
#[doc = "Field `WRP2` writer - WRP2"]
pub type WRP2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WRP2"]
    #[inline(always)]
    pub fn wrp2(&self) -> WRP2_R {
        WRP2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WRP2"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2(&mut self) -> WRP2_W<WRPR2rs> {
        WRP2_W::new(self, 0)
    }
}
#[doc = "Write protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPR2rs;
impl crate::RegisterSpec for WRPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpr2::R`](R) reader structure"]
impl crate::Readable for WRPR2rs {}
#[doc = "`write(|w| ..)` method takes [`wrpr2::W`](W) writer structure"]
impl crate::Writable for WRPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRPR2 to value 0"]
impl crate::Resettable for WRPR2rs {
    const RESET_VALUE: u32 = 0;
}
