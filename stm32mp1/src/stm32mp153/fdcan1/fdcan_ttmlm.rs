#[doc = "Register `FDCAN_TTMLM` reader"]
pub type R = crate::R<FDCAN_TTMLMrs>;
#[doc = "Register `FDCAN_TTMLM` writer"]
pub type W = crate::W<FDCAN_TTMLMrs>;
#[doc = "Field `CCM` reader - CCM"]
pub type CCM_R = crate::FieldReader;
#[doc = "Field `CCM` writer - CCM"]
pub type CCM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CSS` reader - CSS"]
pub type CSS_R = crate::FieldReader;
#[doc = "Field `CSS` writer - CSS"]
pub type CSS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXEW` reader - TXEW"]
pub type TXEW_R = crate::FieldReader;
#[doc = "Field `TXEW` writer - TXEW"]
pub type TXEW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ENTT` reader - ENTT"]
pub type ENTT_R = crate::FieldReader<u16>;
#[doc = "Field `ENTT` writer - ENTT"]
pub type ENTT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:5 - CCM"]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - CSS"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - TXEW"]
    #[inline(always)]
    pub fn txew(&self) -> TXEW_R {
        TXEW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - ENTT"]
    #[inline(always)]
    pub fn entt(&self) -> ENTT_R {
        ENTT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - CCM"]
    #[inline(always)]
    #[must_use]
    pub fn ccm(&mut self) -> CCM_W<FDCAN_TTMLMrs> {
        CCM_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - CSS"]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CSS_W<FDCAN_TTMLMrs> {
        CSS_W::new(self, 6)
    }
    #[doc = "Bits 8:11 - TXEW"]
    #[inline(always)]
    #[must_use]
    pub fn txew(&mut self) -> TXEW_W<FDCAN_TTMLMrs> {
        TXEW_W::new(self, 8)
    }
    #[doc = "Bits 16:27 - ENTT"]
    #[inline(always)]
    #[must_use]
    pub fn entt(&mut self) -> ENTT_W<FDCAN_TTMLMrs> {
        ENTT_W::new(self, 16)
    }
}
#[doc = "FDCAN TT matrix limits register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttmlm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttmlm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTMLMrs;
impl crate::RegisterSpec for FDCAN_TTMLMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttmlm::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTMLMrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttmlm::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTMLMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TTMLM to value 0"]
impl crate::Resettable for FDCAN_TTMLMrs {
    const RESET_VALUE: u32 = 0;
}
