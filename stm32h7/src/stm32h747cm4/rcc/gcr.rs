#[doc = "Register `GCR` reader"]
pub type R = crate::R<GCRrs>;
#[doc = "Register `GCR` writer"]
pub type W = crate::W<GCRrs>;
#[doc = "WWDG1 reset scope control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WW1RSC {
    #[doc = "0: Clear WWDG1 scope control"]
    Clear = 0,
    #[doc = "1: Set WWDG1 scope control"]
    Set = 1,
}
impl From<WW1RSC> for bool {
    #[inline(always)]
    fn from(variant: WW1RSC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WW1RSC` reader - WWDG1 reset scope control"]
pub type WW1RSC_R = crate::BitReader<WW1RSC>;
impl WW1RSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WW1RSC {
        match self.bits {
            false => WW1RSC::Clear,
            true => WW1RSC::Set,
        }
    }
    #[doc = "Clear WWDG1 scope control"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WW1RSC::Clear
    }
    #[doc = "Set WWDG1 scope control"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == WW1RSC::Set
    }
}
#[doc = "Field `WW1RSC` writer - WWDG1 reset scope control"]
pub type WW1RSC_W<'a, REG> = crate::BitWriter<'a, REG, WW1RSC>;
impl<'a, REG> WW1RSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear WWDG1 scope control"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WW1RSC::Clear)
    }
    #[doc = "Set WWDG1 scope control"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(WW1RSC::Set)
    }
}
#[doc = "Field `WW2RSC` reader - WWDG2 reset scope control"]
pub type WW2RSC_R = crate::BitReader;
#[doc = "Field `WW2RSC` writer - WWDG2 reset scope control"]
pub type WW2RSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_C1` reader - Force allow CPU1 to boot"]
pub type BOOT_C1_R = crate::BitReader;
#[doc = "Field `BOOT_C1` writer - Force allow CPU1 to boot"]
pub type BOOT_C1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_C2` reader - Force allow CPU2 to boot"]
pub type BOOT_C2_R = crate::BitReader;
#[doc = "Field `BOOT_C2` writer - Force allow CPU2 to boot"]
pub type BOOT_C2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WWDG1 reset scope control"]
    #[inline(always)]
    pub fn ww1rsc(&self) -> WW1RSC_R {
        WW1RSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WWDG2 reset scope control"]
    #[inline(always)]
    pub fn ww2rsc(&self) -> WW2RSC_R {
        WW2RSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force allow CPU1 to boot"]
    #[inline(always)]
    pub fn boot_c1(&self) -> BOOT_C1_R {
        BOOT_C1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force allow CPU2 to boot"]
    #[inline(always)]
    pub fn boot_c2(&self) -> BOOT_C2_R {
        BOOT_C2_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WWDG1 reset scope control"]
    #[inline(always)]
    #[must_use]
    pub fn ww1rsc(&mut self) -> WW1RSC_W<GCRrs> {
        WW1RSC_W::new(self, 0)
    }
    #[doc = "Bit 1 - WWDG2 reset scope control"]
    #[inline(always)]
    #[must_use]
    pub fn ww2rsc(&mut self) -> WW2RSC_W<GCRrs> {
        WW2RSC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Force allow CPU1 to boot"]
    #[inline(always)]
    #[must_use]
    pub fn boot_c1(&mut self) -> BOOT_C1_W<GCRrs> {
        BOOT_C1_W::new(self, 2)
    }
    #[doc = "Bit 3 - Force allow CPU2 to boot"]
    #[inline(always)]
    #[must_use]
    pub fn boot_c2(&mut self) -> BOOT_C2_W<GCRrs> {
        BOOT_C2_W::new(self, 3)
    }
}
#[doc = "RCC Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCRrs;
impl crate::RegisterSpec for GCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GCRrs {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCR to value 0"]
impl crate::Resettable for GCRrs {
    const RESET_VALUE: u32 = 0;
}
