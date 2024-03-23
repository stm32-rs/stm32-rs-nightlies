#[doc = "Register `WRP2AR` reader"]
pub type R = crate::R<WRP2ARrs>;
#[doc = "Register `WRP2AR` writer"]
pub type W = crate::W<WRP2ARrs>;
#[doc = "Field `WRP2A_PSTRT` reader - WRP2A_PSTRT"]
pub type WRP2A_PSTRT_R = crate::FieldReader;
#[doc = "Field `WRP2A_PSTRT` writer - WRP2A_PSTRT"]
pub type WRP2A_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRP2A_PEND` reader - WRP2A_PEND"]
pub type WRP2A_PEND_R = crate::FieldReader;
#[doc = "Field `WRP2A_PEND` writer - WRP2A_PEND"]
pub type WRP2A_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - WRP2A_PSTRT"]
    #[inline(always)]
    pub fn wrp2a_pstrt(&self) -> WRP2A_PSTRT_R {
        WRP2A_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP2A_PEND"]
    #[inline(always)]
    pub fn wrp2a_pend(&self) -> WRP2A_PEND_R {
        WRP2A_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - WRP2A_PSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2a_pstrt(&mut self) -> WRP2A_PSTRT_W<WRP2ARrs> {
        WRP2A_PSTRT_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - WRP2A_PEND"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2a_pend(&mut self) -> WRP2A_PEND_W<WRP2ARrs> {
        WRP2A_PEND_W::new(self, 16)
    }
}
#[doc = "Flash WPR2 area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp2ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp2ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRP2ARrs;
impl crate::RegisterSpec for WRP2ARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp2ar::R`](R) reader structure"]
impl crate::Readable for WRP2ARrs {}
#[doc = "`write(|w| ..)` method takes [`wrp2ar::W`](W) writer structure"]
impl crate::Writable for WRP2ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRP2AR to value 0xff00_ff00"]
impl crate::Resettable for WRP2ARrs {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
