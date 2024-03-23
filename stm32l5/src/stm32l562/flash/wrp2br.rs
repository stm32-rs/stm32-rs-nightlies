#[doc = "Register `WRP2BR` reader"]
pub type R = crate::R<WRP2BRrs>;
#[doc = "Register `WRP2BR` writer"]
pub type W = crate::W<WRP2BRrs>;
#[doc = "Field `WRP2B_PSTRT` reader - WRP2B_PSTRT"]
pub type WRP2B_PSTRT_R = crate::FieldReader;
#[doc = "Field `WRP2B_PSTRT` writer - WRP2B_PSTRT"]
pub type WRP2B_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRP2B_PEND` reader - WRP2B_PEND"]
pub type WRP2B_PEND_R = crate::FieldReader;
#[doc = "Field `WRP2B_PEND` writer - WRP2B_PEND"]
pub type WRP2B_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - WRP2B_PSTRT"]
    #[inline(always)]
    pub fn wrp2b_pstrt(&self) -> WRP2B_PSTRT_R {
        WRP2B_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP2B_PEND"]
    #[inline(always)]
    pub fn wrp2b_pend(&self) -> WRP2B_PEND_R {
        WRP2B_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - WRP2B_PSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2b_pstrt(&mut self) -> WRP2B_PSTRT_W<WRP2BRrs> {
        WRP2B_PSTRT_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - WRP2B_PEND"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2b_pend(&mut self) -> WRP2B_PEND_W<WRP2BRrs> {
        WRP2B_PEND_W::new(self, 16)
    }
}
#[doc = "Flash WPR2 area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp2br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp2br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRP2BRrs;
impl crate::RegisterSpec for WRP2BRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp2br::R`](R) reader structure"]
impl crate::Readable for WRP2BRrs {}
#[doc = "`write(|w| ..)` method takes [`wrp2br::W`](W) writer structure"]
impl crate::Writable for WRP2BRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRP2BR to value 0xff00_ff00"]
impl crate::Resettable for WRP2BRrs {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
