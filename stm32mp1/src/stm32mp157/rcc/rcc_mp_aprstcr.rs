#[doc = "Register `RCC_MP_APRSTCR` reader"]
pub type R = crate::R<RCC_MP_APRSTCRrs>;
#[doc = "Register `RCC_MP_APRSTCR` writer"]
pub type W = crate::W<RCC_MP_APRSTCRrs>;
#[doc = "Field `RDCTLEN` reader - RDCTLEN"]
pub type RDCTLEN_R = crate::BitReader;
#[doc = "Field `RDCTLEN` writer - RDCTLEN"]
pub type RDCTLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTTO` reader - RSTTO"]
pub type RSTTO_R = crate::FieldReader;
#[doc = "Field `RSTTO` writer - RSTTO"]
pub type RSTTO_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - RDCTLEN"]
    #[inline(always)]
    pub fn rdctlen(&self) -> RDCTLEN_R {
        RDCTLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:14 - RSTTO"]
    #[inline(always)]
    pub fn rstto(&self) -> RSTTO_R {
        RSTTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RDCTLEN"]
    #[inline(always)]
    #[must_use]
    pub fn rdctlen(&mut self) -> RDCTLEN_W<RCC_MP_APRSTCRrs> {
        RDCTLEN_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - RSTTO"]
    #[inline(always)]
    #[must_use]
    pub fn rstto(&mut self) -> RSTTO_W<RCC_MP_APRSTCRrs> {
        RSTTO_W::new(self, 8)
    }
}
#[doc = "This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_aprstcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_aprstcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_APRSTCRrs;
impl crate::RegisterSpec for RCC_MP_APRSTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_aprstcr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_APRSTCRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_aprstcr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_APRSTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_APRSTCR to value 0x7f00"]
impl crate::Resettable for RCC_MP_APRSTCRrs {
    const RESET_VALUE: u32 = 0x7f00;
}
