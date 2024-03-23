#[doc = "Register `WRP2BR` reader"]
pub type R = crate::R<WRP2BRrs>;
#[doc = "Register `WRP2BR` writer"]
pub type W = crate::W<WRP2BRrs>;
#[doc = "Field `WRP2B_STRT` reader - WRP area B start offset, Bank 2"]
pub type WRP2B_STRT_R = crate::FieldReader;
#[doc = "Field `WRP2B_STRT` writer - WRP area B start offset, Bank 2"]
pub type WRP2B_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRP2B_END` reader - WRP area B end offset, Bank 2"]
pub type WRP2B_END_R = crate::FieldReader;
#[doc = "Field `WRP2B_END` writer - WRP area B end offset, Bank 2"]
pub type WRP2B_END_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - WRP area B start offset, Bank 2"]
    #[inline(always)]
    pub fn wrp2b_strt(&self) -> WRP2B_STRT_R {
        WRP2B_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP area B end offset, Bank 2"]
    #[inline(always)]
    pub fn wrp2b_end(&self) -> WRP2B_END_R {
        WRP2B_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - WRP area B start offset, Bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2b_strt(&mut self) -> WRP2B_STRT_W<WRP2BRrs> {
        WRP2B_STRT_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - WRP area B end offset, Bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2b_end(&mut self) -> WRP2B_END_W<WRP2BRrs> {
        WRP2B_END_W::new(self, 16)
    }
}
#[doc = "Flash WRP2 area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp2br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp2br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets WRP2BR to value 0xff"]
impl crate::Resettable for WRP2BRrs {
    const RESET_VALUE: u32 = 0xff;
}
