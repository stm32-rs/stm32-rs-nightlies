#[doc = "Register `WRP2BR` reader"]
pub type R = crate::R<WRP2BRrs>;
#[doc = "Register `WRP2BR` writer"]
pub type W = crate::W<WRP2BRrs>;
#[doc = "Field `WRP2B_STRT` reader - Bank 2 WRP second area B start offset"]
pub type WRP2B_STRT_R = crate::FieldReader;
#[doc = "Field `WRP2B_STRT` writer - Bank 2 WRP second area B start offset"]
pub type WRP2B_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRP2B_END` reader - Bank 2 WRP second area B end offset"]
pub type WRP2B_END_R = crate::FieldReader;
#[doc = "Field `WRP2B_END` writer - Bank 2 WRP second area B end offset"]
pub type WRP2B_END_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bank 2 WRP second area B start offset"]
    #[inline(always)]
    pub fn wrp2b_strt(&self) -> WRP2B_STRT_R {
        WRP2B_STRT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Bank 2 WRP second area B end offset"]
    #[inline(always)]
    pub fn wrp2b_end(&self) -> WRP2B_END_R {
        WRP2B_END_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank 2 WRP second area B start offset"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2b_strt(&mut self) -> WRP2B_STRT_W<WRP2BRrs> {
        WRP2B_STRT_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Bank 2 WRP second area B end offset"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2b_end(&mut self) -> WRP2B_END_W<WRP2BRrs> {
        WRP2B_END_W::new(self, 16)
    }
}
#[doc = "Flash Bank 2 WRP area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp2br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp2br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
