#[doc = "Register `PLLSAICFGR` reader"]
pub type R = crate::R<PLLSAICFGRrs>;
#[doc = "Register `PLLSAICFGR` writer"]
pub type W = crate::W<PLLSAICFGRrs>;
#[doc = "Field `PLLSAIN` reader - PLLSAI division factor for VCO"]
pub type PLLSAIN_R = crate::FieldReader<u16>;
#[doc = "Field `PLLSAIN` writer - PLLSAI division factor for VCO"]
pub type PLLSAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PLLSAIQ` reader - PLLSAI division factor for SAI1 clock"]
pub type PLLSAIQ_R = crate::FieldReader;
#[doc = "Field `PLLSAIQ` writer - PLLSAI division factor for SAI1 clock"]
pub type PLLSAIQ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLLSAIR` reader - PLLSAI division factor for LCD clock"]
pub type PLLSAIR_R = crate::FieldReader;
#[doc = "Field `PLLSAIR` writer - PLLSAI division factor for LCD clock"]
pub type PLLSAIR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 6:14 - PLLSAI division factor for VCO"]
    #[inline(always)]
    pub fn pllsain(&self) -> PLLSAIN_R {
        PLLSAIN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:27 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    pub fn pllsaiq(&self) -> PLLSAIQ_R {
        PLLSAIQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - PLLSAI division factor for LCD clock"]
    #[inline(always)]
    pub fn pllsair(&self) -> PLLSAIR_R {
        PLLSAIR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 6:14 - PLLSAI division factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn pllsain(&mut self) -> PLLSAIN_W<PLLSAICFGRrs> {
        PLLSAIN_W::new(self, 6)
    }
    #[doc = "Bits 24:27 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsaiq(&mut self) -> PLLSAIQ_W<PLLSAICFGRrs> {
        PLLSAIQ_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - PLLSAI division factor for LCD clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsair(&mut self) -> PLLSAIR_W<PLLSAICFGRrs> {
        PLLSAIR_W::new(self, 28)
    }
}
#[doc = "RCC PLL configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllsaicfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllsaicfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLSAICFGRrs;
impl crate::RegisterSpec for PLLSAICFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllsaicfgr::R`](R) reader structure"]
impl crate::Readable for PLLSAICFGRrs {}
#[doc = "`write(|w| ..)` method takes [`pllsaicfgr::W`](W) writer structure"]
impl crate::Writable for PLLSAICFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLSAICFGR to value 0x2400_3000"]
impl crate::Resettable for PLLSAICFGRrs {
    const RESET_VALUE: u32 = 0x2400_3000;
}
