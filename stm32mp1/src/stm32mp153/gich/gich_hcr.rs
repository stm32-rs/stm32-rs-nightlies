#[doc = "Register `GICH_HCR` reader"]
pub type R = crate::R<GICH_HCRrs>;
#[doc = "Register `GICH_HCR` writer"]
pub type W = crate::W<GICH_HCRrs>;
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UIE` reader - UIE"]
pub type UIE_R = crate::BitReader;
#[doc = "Field `UIE` writer - UIE"]
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LRENPIE` reader - LRENPIE"]
pub type LRENPIE_R = crate::BitReader;
#[doc = "Field `LRENPIE` writer - LRENPIE"]
pub type LRENPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPIE` reader - NPIE"]
pub type NPIE_R = crate::BitReader;
#[doc = "Field `NPIE` writer - NPIE"]
pub type NPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VGRP0EIE` reader - VGRP0EIE"]
pub type VGRP0EIE_R = crate::BitReader;
#[doc = "Field `VGRP0EIE` writer - VGRP0EIE"]
pub type VGRP0EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VGRP0DIE` reader - VGRP0DIE"]
pub type VGRP0DIE_R = crate::BitReader;
#[doc = "Field `VGRP0DIE` writer - VGRP0DIE"]
pub type VGRP0DIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VGRP1EIE` reader - VGRP1EIE"]
pub type VGRP1EIE_R = crate::BitReader;
#[doc = "Field `VGRP1EIE` writer - VGRP1EIE"]
pub type VGRP1EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VGRP1DIE` reader - VGRP1DIE"]
pub type VGRP1DIE_R = crate::BitReader;
#[doc = "Field `VGRP1DIE` writer - VGRP1DIE"]
pub type VGRP1DIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOICOUNT` reader - EOICOUNT"]
pub type EOICOUNT_R = crate::FieldReader;
#[doc = "Field `EOICOUNT` writer - EOICOUNT"]
pub type EOICOUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UIE"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LRENPIE"]
    #[inline(always)]
    pub fn lrenpie(&self) -> LRENPIE_R {
        LRENPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NPIE"]
    #[inline(always)]
    pub fn npie(&self) -> NPIE_R {
        NPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VGRP0EIE"]
    #[inline(always)]
    pub fn vgrp0eie(&self) -> VGRP0EIE_R {
        VGRP0EIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VGRP0DIE"]
    #[inline(always)]
    pub fn vgrp0die(&self) -> VGRP0DIE_R {
        VGRP0DIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VGRP1EIE"]
    #[inline(always)]
    pub fn vgrp1eie(&self) -> VGRP1EIE_R {
        VGRP1EIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VGRP1DIE"]
    #[inline(always)]
    pub fn vgrp1die(&self) -> VGRP1DIE_R {
        VGRP1DIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 27:31 - EOICOUNT"]
    #[inline(always)]
    pub fn eoicount(&self) -> EOICOUNT_R {
        EOICOUNT_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<GICH_HCRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - UIE"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<GICH_HCRrs> {
        UIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - LRENPIE"]
    #[inline(always)]
    #[must_use]
    pub fn lrenpie(&mut self) -> LRENPIE_W<GICH_HCRrs> {
        LRENPIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - NPIE"]
    #[inline(always)]
    #[must_use]
    pub fn npie(&mut self) -> NPIE_W<GICH_HCRrs> {
        NPIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - VGRP0EIE"]
    #[inline(always)]
    #[must_use]
    pub fn vgrp0eie(&mut self) -> VGRP0EIE_W<GICH_HCRrs> {
        VGRP0EIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - VGRP0DIE"]
    #[inline(always)]
    #[must_use]
    pub fn vgrp0die(&mut self) -> VGRP0DIE_W<GICH_HCRrs> {
        VGRP0DIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - VGRP1EIE"]
    #[inline(always)]
    #[must_use]
    pub fn vgrp1eie(&mut self) -> VGRP1EIE_W<GICH_HCRrs> {
        VGRP1EIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - VGRP1DIE"]
    #[inline(always)]
    #[must_use]
    pub fn vgrp1die(&mut self) -> VGRP1DIE_W<GICH_HCRrs> {
        VGRP1DIE_W::new(self, 7)
    }
    #[doc = "Bits 27:31 - EOICOUNT"]
    #[inline(always)]
    #[must_use]
    pub fn eoicount(&mut self) -> EOICOUNT_W<GICH_HCRrs> {
        EOICOUNT_W::new(self, 27)
    }
}
#[doc = "GICH hypervisor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_hcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gich_hcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICH_HCRrs;
impl crate::RegisterSpec for GICH_HCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gich_hcr::R`](R) reader structure"]
impl crate::Readable for GICH_HCRrs {}
#[doc = "`write(|w| ..)` method takes [`gich_hcr::W`](W) writer structure"]
impl crate::Writable for GICH_HCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICH_HCR to value 0"]
impl crate::Resettable for GICH_HCRrs {
    const RESET_VALUE: u32 = 0;
}
