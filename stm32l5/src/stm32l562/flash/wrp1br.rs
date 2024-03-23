#[doc = "Register `WRP1BR` reader"]
pub type R = crate::R<WRP1BRrs>;
#[doc = "Register `WRP1BR` writer"]
pub type W = crate::W<WRP1BRrs>;
#[doc = "Field `WRP1B_PSTRT` reader - WRP1B_PSTRT"]
pub type WRP1B_PSTRT_R = crate::FieldReader;
#[doc = "Field `WRP1B_PSTRT` writer - WRP1B_PSTRT"]
pub type WRP1B_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRP1B_PEND` reader - WRP1B_PEND"]
pub type WRP1B_PEND_R = crate::FieldReader;
#[doc = "Field `WRP1B_PEND` writer - WRP1B_PEND"]
pub type WRP1B_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - WRP1B_PSTRT"]
    #[inline(always)]
    pub fn wrp1b_pstrt(&self) -> WRP1B_PSTRT_R {
        WRP1B_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP1B_PEND"]
    #[inline(always)]
    pub fn wrp1b_pend(&self) -> WRP1B_PEND_R {
        WRP1B_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - WRP1B_PSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1b_pstrt(&mut self) -> WRP1B_PSTRT_W<WRP1BRrs> {
        WRP1B_PSTRT_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - WRP1B_PEND"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1b_pend(&mut self) -> WRP1B_PEND_W<WRP1BRrs> {
        WRP1B_PEND_W::new(self, 16)
    }
}
#[doc = "Flash Bank 1 WRP area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp1br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRP1BRrs;
impl crate::RegisterSpec for WRP1BRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp1br::R`](R) reader structure"]
impl crate::Readable for WRP1BRrs {}
#[doc = "`write(|w| ..)` method takes [`wrp1br::W`](W) writer structure"]
impl crate::Writable for WRP1BRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRP1BR to value 0xff00_ff00"]
impl crate::Resettable for WRP1BRrs {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
