#[doc = "Register `WRP2AR` reader"]
pub type R = crate::R<WRP2ARrs>;
#[doc = "Register `WRP2AR` writer"]
pub type W = crate::W<WRP2ARrs>;
#[doc = "Field `WRP2A_STRT` reader - Bank 2 WRP first area A start offset"]
pub type WRP2A_STRT_R = crate::FieldReader;
#[doc = "Field `WRP2A_STRT` writer - Bank 2 WRP first area A start offset"]
pub type WRP2A_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRP2A_END` reader - Bank 2 WRP first area A end offset"]
pub type WRP2A_END_R = crate::FieldReader;
#[doc = "Field `WRP2A_END` writer - Bank 2 WRP first area A end offset"]
pub type WRP2A_END_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bank 2 WRP first area A start offset"]
    #[inline(always)]
    pub fn wrp2a_strt(&self) -> WRP2A_STRT_R {
        WRP2A_STRT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Bank 2 WRP first area A end offset"]
    #[inline(always)]
    pub fn wrp2a_end(&self) -> WRP2A_END_R {
        WRP2A_END_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank 2 WRP first area A start offset"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2a_strt(&mut self) -> WRP2A_STRT_W<WRP2ARrs> {
        WRP2A_STRT_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Bank 2 WRP first area A end offset"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2a_end(&mut self) -> WRP2A_END_W<WRP2ARrs> {
        WRP2A_END_W::new(self, 16)
    }
}
#[doc = "Flash Bank 2 WRP area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp2ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp2ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
