#[doc = "Register `COMP1_CSR` reader"]
pub type R = crate::R<COMP1_CSRrs>;
#[doc = "Register `COMP1_CSR` writer"]
pub type W = crate::W<COMP1_CSRrs>;
#[doc = "Field `COMP1_EN` reader - Comparator enable"]
pub type COMP1_EN_R = crate::BitReader;
#[doc = "Field `COMP1_EN` writer - Comparator enable"]
pub type COMP1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_PWRMODE` reader - Comparator power mode"]
pub type COMP1_PWRMODE_R = crate::FieldReader;
#[doc = "Field `COMP1_PWRMODE` writer - Comparator power mode"]
pub type COMP1_PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP1_INMSEL` reader - Comparator input minus selection"]
pub type COMP1_INMSEL_R = crate::FieldReader;
#[doc = "Field `COMP1_INMSEL` writer - Comparator input minus selection"]
pub type COMP1_INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP1_INPSEL` reader - Comparator input plus selection"]
pub type COMP1_INPSEL_R = crate::FieldReader;
#[doc = "Field `COMP1_INPSEL` writer - Comparator input plus selection"]
pub type COMP1_INPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP1_POLARITY` reader - Comparator output polarity"]
pub type COMP1_POLARITY_R = crate::BitReader;
#[doc = "Field `COMP1_POLARITY` writer - Comparator output polarity"]
pub type COMP1_POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_HYST` reader - Comparator hysteresis"]
pub type COMP1_HYST_R = crate::FieldReader;
#[doc = "Field `COMP1_HYST` writer - Comparator hysteresis"]
pub type COMP1_HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP1_BLANKING` reader - Comparator blanking source"]
pub type COMP1_BLANKING_R = crate::FieldReader;
#[doc = "Field `COMP1_BLANKING` writer - Comparator blanking source"]
pub type COMP1_BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP1_BRGEN` reader - Comparator voltage scaler enable"]
pub type COMP1_BRGEN_R = crate::BitReader;
#[doc = "Field `COMP1_BRGEN` writer - Comparator voltage scaler enable"]
pub type COMP1_BRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_SCALEN` reader - Comparator scaler bridge enable"]
pub type COMP1_SCALEN_R = crate::BitReader;
#[doc = "Field `COMP1_SCALEN` writer - Comparator scaler bridge enable"]
pub type COMP1_SCALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_INMESEL` reader - Comparator input minus extended selection"]
pub type COMP1_INMESEL_R = crate::FieldReader;
#[doc = "Field `COMP1_INMESEL` writer - Comparator input minus extended selection"]
pub type COMP1_INMESEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP1_VALUE` reader - Comparator output level"]
pub type COMP1_VALUE_R = crate::BitReader;
#[doc = "Field `COMP1_LOCK` reader - Comparator lock"]
pub type COMP1_LOCK_R = crate::BitReader;
#[doc = "Field `COMP1_LOCK` writer - Comparator lock"]
pub type COMP1_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator enable"]
    #[inline(always)]
    pub fn comp1_en(&self) -> COMP1_EN_R {
        COMP1_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator power mode"]
    #[inline(always)]
    pub fn comp1_pwrmode(&self) -> COMP1_PWRMODE_R {
        COMP1_PWRMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator input minus selection"]
    #[inline(always)]
    pub fn comp1_inmsel(&self) -> COMP1_INMSEL_R {
        COMP1_INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Comparator input plus selection"]
    #[inline(always)]
    pub fn comp1_inpsel(&self) -> COMP1_INPSEL_R {
        COMP1_INPSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 15 - Comparator output polarity"]
    #[inline(always)]
    pub fn comp1_polarity(&self) -> COMP1_POLARITY_R {
        COMP1_POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator hysteresis"]
    #[inline(always)]
    pub fn comp1_hyst(&self) -> COMP1_HYST_R {
        COMP1_HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator blanking source"]
    #[inline(always)]
    pub fn comp1_blanking(&self) -> COMP1_BLANKING_R {
        COMP1_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Comparator voltage scaler enable"]
    #[inline(always)]
    pub fn comp1_brgen(&self) -> COMP1_BRGEN_R {
        COMP1_BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Comparator scaler bridge enable"]
    #[inline(always)]
    pub fn comp1_scalen(&self) -> COMP1_SCALEN_R {
        COMP1_SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Comparator input minus extended selection"]
    #[inline(always)]
    pub fn comp1_inmesel(&self) -> COMP1_INMESEL_R {
        COMP1_INMESEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 30 - Comparator output level"]
    #[inline(always)]
    pub fn comp1_value(&self) -> COMP1_VALUE_R {
        COMP1_VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator lock"]
    #[inline(always)]
    pub fn comp1_lock(&self) -> COMP1_LOCK_R {
        COMP1_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_en(&mut self) -> COMP1_EN_W<COMP1_CSRrs> {
        COMP1_EN_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Comparator power mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_pwrmode(&mut self) -> COMP1_PWRMODE_W<COMP1_CSRrs> {
        COMP1_PWRMODE_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator input minus selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_inmsel(&mut self) -> COMP1_INMSEL_W<COMP1_CSRrs> {
        COMP1_INMSEL_W::new(self, 4)
    }
    #[doc = "Bits 7:8 - Comparator input plus selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_inpsel(&mut self) -> COMP1_INPSEL_W<COMP1_CSRrs> {
        COMP1_INPSEL_W::new(self, 7)
    }
    #[doc = "Bit 15 - Comparator output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_polarity(&mut self) -> COMP1_POLARITY_W<COMP1_CSRrs> {
        COMP1_POLARITY_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_hyst(&mut self) -> COMP1_HYST_W<COMP1_CSRrs> {
        COMP1_HYST_W::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator blanking source"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_blanking(&mut self) -> COMP1_BLANKING_W<COMP1_CSRrs> {
        COMP1_BLANKING_W::new(self, 18)
    }
    #[doc = "Bit 22 - Comparator voltage scaler enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_brgen(&mut self) -> COMP1_BRGEN_W<COMP1_CSRrs> {
        COMP1_BRGEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Comparator scaler bridge enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_scalen(&mut self) -> COMP1_SCALEN_W<COMP1_CSRrs> {
        COMP1_SCALEN_W::new(self, 23)
    }
    #[doc = "Bits 25:26 - Comparator input minus extended selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_inmesel(&mut self) -> COMP1_INMESEL_W<COMP1_CSRrs> {
        COMP1_INMESEL_W::new(self, 25)
    }
    #[doc = "Bit 31 - Comparator lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_lock(&mut self) -> COMP1_LOCK_W<COMP1_CSRrs> {
        COMP1_LOCK_W::new(self, 31)
    }
}
#[doc = "Comparator control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP1_CSRrs;
impl crate::RegisterSpec for COMP1_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp1_csr::R`](R) reader structure"]
impl crate::Readable for COMP1_CSRrs {}
#[doc = "`write(|w| ..)` method takes [`comp1_csr::W`](W) writer structure"]
impl crate::Writable for COMP1_CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP1_CSR to value 0"]
impl crate::Resettable for COMP1_CSRrs {
    const RESET_VALUE: u32 = 0;
}
