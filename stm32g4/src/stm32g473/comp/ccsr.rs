#[doc = "Register `C%sCSR` reader"]
pub type R = crate::R<CCSRrs>;
#[doc = "Register `C%sCSR` writer"]
pub type W = crate::W<CCSRrs>;
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_DEGLITCH_EN` reader - COMP_DEGLITCH_EN"]
pub type COMP_DEGLITCH_EN_R = crate::BitReader;
#[doc = "Field `COMP_DEGLITCH_EN` writer - COMP_DEGLITCH_EN"]
pub type COMP_DEGLITCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INMSEL` reader - INMSEL"]
pub type INMSEL_R = crate::FieldReader;
#[doc = "Field `INMSEL` writer - INMSEL"]
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INPSEL` reader - INPSEL"]
pub type INPSEL_R = crate::BitReader;
#[doc = "Field `INPSEL` writer - INPSEL"]
pub type INPSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - POL"]
pub type POL_R = crate::BitReader;
#[doc = "Field `POL` writer - POL"]
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - HYST"]
pub type HYST_R = crate::FieldReader;
#[doc = "Field `HYST` writer - HYST"]
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BLANKSEL` reader - BLANKSEL"]
pub type BLANKSEL_R = crate::FieldReader;
#[doc = "Field `BLANKSEL` writer - BLANKSEL"]
pub type BLANKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BRGEN` reader - BRGEN"]
pub type BRGEN_R = crate::BitReader;
#[doc = "Field `BRGEN` writer - BRGEN"]
pub type BRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALEN` reader - SCALEN"]
pub type SCALEN_R = crate::BitReader;
#[doc = "Field `SCALEN` writer - SCALEN"]
pub type SCALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALUE` reader - VALUE"]
pub type VALUE_R = crate::BitReader;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COMP_DEGLITCH_EN"]
    #[inline(always)]
    pub fn comp_deglitch_en(&self) -> COMP_DEGLITCH_EN_R {
        COMP_DEGLITCH_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - INMSEL"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - INPSEL"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - POL"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - HYST"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - BLANKSEL"]
    #[inline(always)]
    pub fn blanksel(&self) -> BLANKSEL_R {
        BLANKSEL_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22 - BRGEN"]
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SCALEN"]
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CCSRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - COMP_DEGLITCH_EN"]
    #[inline(always)]
    #[must_use]
    pub fn comp_deglitch_en(&mut self) -> COMP_DEGLITCH_EN_W<CCSRrs> {
        COMP_DEGLITCH_EN_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - INMSEL"]
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<CCSRrs> {
        INMSEL_W::new(self, 4)
    }
    #[doc = "Bit 8 - INPSEL"]
    #[inline(always)]
    #[must_use]
    pub fn inpsel(&mut self) -> INPSEL_W<CCSRrs> {
        INPSEL_W::new(self, 8)
    }
    #[doc = "Bit 15 - POL"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<CCSRrs> {
        POL_W::new(self, 15)
    }
    #[doc = "Bits 16:18 - HYST"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<CCSRrs> {
        HYST_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - BLANKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn blanksel(&mut self) -> BLANKSEL_W<CCSRrs> {
        BLANKSEL_W::new(self, 19)
    }
    #[doc = "Bit 22 - BRGEN"]
    #[inline(always)]
    #[must_use]
    pub fn brgen(&mut self) -> BRGEN_W<CCSRrs> {
        BRGEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - SCALEN"]
    #[inline(always)]
    #[must_use]
    pub fn scalen(&mut self) -> SCALEN_W<CCSRrs> {
        SCALEN_W::new(self, 23)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<CCSRrs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCSRrs;
impl crate::RegisterSpec for CCSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccsr::R`](R) reader structure"]
impl crate::Readable for CCSRrs {}
#[doc = "`write(|w| ..)` method takes [`ccsr::W`](W) writer structure"]
impl crate::Writable for CCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C%sCSR to value 0"]
impl crate::Resettable for CCSRrs {
    const RESET_VALUE: u32 = 0;
}
