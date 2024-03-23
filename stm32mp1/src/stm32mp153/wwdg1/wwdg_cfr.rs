#[doc = "Register `WWDG_CFR` reader"]
pub type R = crate::R<WWDG_CFRrs>;
#[doc = "Register `WWDG_CFR` writer"]
pub type W = crate::W<WWDG_CFRrs>;
#[doc = "Field `W` reader - W"]
pub type W_R = crate::FieldReader;
#[doc = "Field `W` writer - W"]
pub type W_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EWI` reader - EWI"]
pub type EWI_R = crate::BitReader;
#[doc = "Field `EWI` writer - EWI"]
pub type EWI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGTB` reader - WDGTB"]
pub type WDGTB_R = crate::FieldReader;
#[doc = "Field `WDGTB` writer - WDGTB"]
pub type WDGTB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - W"]
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 9 - EWI"]
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 11:13 - WDGTB"]
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 11) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - W"]
    #[inline(always)]
    #[must_use]
    pub fn w(&mut self) -> W_W<WWDG_CFRrs> {
        W_W::new(self, 0)
    }
    #[doc = "Bit 9 - EWI"]
    #[inline(always)]
    #[must_use]
    pub fn ewi(&mut self) -> EWI_W<WWDG_CFRrs> {
        EWI_W::new(self, 9)
    }
    #[doc = "Bits 11:13 - WDGTB"]
    #[inline(always)]
    #[must_use]
    pub fn wdgtb(&mut self) -> WDGTB_W<WWDG_CFRrs> {
        WDGTB_W::new(self, 11)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_cfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdg_cfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WWDG_CFRrs;
impl crate::RegisterSpec for WWDG_CFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdg_cfr::R`](R) reader structure"]
impl crate::Readable for WWDG_CFRrs {}
#[doc = "`write(|w| ..)` method takes [`wwdg_cfr::W`](W) writer structure"]
impl crate::Writable for WWDG_CFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WWDG_CFR to value 0x7f"]
impl crate::Resettable for WWDG_CFRrs {
    const RESET_VALUE: u32 = 0x7f;
}
