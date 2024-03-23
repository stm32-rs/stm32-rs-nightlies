#[doc = "Register `COMP2_CSR` reader"]
pub type R = crate::R<COMP2_CSRrs>;
#[doc = "Register `COMP2_CSR` writer"]
pub type W = crate::W<COMP2_CSRrs>;
#[doc = "Field `COMP2_EN` reader - Comparator 2 enable bit"]
pub type COMP2_EN_R = crate::BitReader;
#[doc = "Field `COMP2_EN` writer - Comparator 2 enable bit"]
pub type COMP2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_PWRMODE` reader - Power Mode of the comparator 2"]
pub type COMP2_PWRMODE_R = crate::FieldReader;
#[doc = "Field `COMP2_PWRMODE` writer - Power Mode of the comparator 2"]
pub type COMP2_PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP2_INMSEL` reader - Comparator 2 Input Minus connection configuration bit"]
pub type COMP2_INMSEL_R = crate::FieldReader;
#[doc = "Field `COMP2_INMSEL` writer - Comparator 2 Input Minus connection configuration bit"]
pub type COMP2_INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP2_INPSEL` reader - Comparator 2 Input Plus connection configuration bit"]
pub type COMP2_INPSEL_R = crate::BitReader;
#[doc = "Field `COMP2_INPSEL` writer - Comparator 2 Input Plus connection configuration bit"]
pub type COMP2_INPSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_WINMODE` reader - Windows mode selection bit"]
pub type COMP2_WINMODE_R = crate::BitReader;
#[doc = "Field `COMP2_WINMODE` writer - Windows mode selection bit"]
pub type COMP2_WINMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_POLARITY` reader - Comparator 2 polarity selection bit"]
pub type COMP2_POLARITY_R = crate::BitReader;
#[doc = "Field `COMP2_POLARITY` writer - Comparator 2 polarity selection bit"]
pub type COMP2_POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_HYST` reader - Comparator 2 hysteresis selection bits"]
pub type COMP2_HYST_R = crate::FieldReader;
#[doc = "Field `COMP2_HYST` writer - Comparator 2 hysteresis selection bits"]
pub type COMP2_HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP2_BLANKING` reader - Comparator 2 blanking source selection bits"]
pub type COMP2_BLANKING_R = crate::FieldReader;
#[doc = "Field `COMP2_BLANKING` writer - Comparator 2 blanking source selection bits"]
pub type COMP2_BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP2_BRGEN` reader - Scaler bridge enable"]
pub type COMP2_BRGEN_R = crate::BitReader;
#[doc = "Field `COMP2_BRGEN` writer - Scaler bridge enable"]
pub type COMP2_BRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_SCALEN` reader - Voltage scaler enable bit"]
pub type COMP2_SCALEN_R = crate::BitReader;
#[doc = "Field `COMP2_SCALEN` writer - Voltage scaler enable bit"]
pub type COMP2_SCALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_VALUE` reader - Comparator 2 output status bit"]
pub type COMP2_VALUE_R = crate::BitReader;
#[doc = "Field `COMP2_LOCK` writer - COMP2_CSR register lock bit"]
pub type COMP2_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2_en(&self) -> COMP2_EN_R {
        COMP2_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 2"]
    #[inline(always)]
    pub fn comp2_pwrmode(&self) -> COMP2_PWRMODE_R {
        COMP2_PWRMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inmsel(&self) -> COMP2_INMSEL_R {
        COMP2_INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inpsel(&self) -> COMP2_INPSEL_R {
        COMP2_INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Windows mode selection bit"]
    #[inline(always)]
    pub fn comp2_winmode(&self) -> COMP2_WINMODE_R {
        COMP2_WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2_polarity(&self) -> COMP2_POLARITY_R {
        COMP2_POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selection bits"]
    #[inline(always)]
    pub fn comp2_hyst(&self) -> COMP2_HYST_R {
        COMP2_HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source selection bits"]
    #[inline(always)]
    pub fn comp2_blanking(&self) -> COMP2_BLANKING_R {
        COMP2_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn comp2_brgen(&self) -> COMP2_BRGEN_R {
        COMP2_BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn comp2_scalen(&self) -> COMP2_SCALEN_R {
        COMP2_SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator 2 output status bit"]
    #[inline(always)]
    pub fn comp2_value(&self) -> COMP2_VALUE_R {
        COMP2_VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_en(&mut self) -> COMP2_EN_W<COMP2_CSRrs> {
        COMP2_EN_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 2"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_pwrmode(&mut self) -> COMP2_PWRMODE_W<COMP2_CSRrs> {
        COMP2_PWRMODE_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_inmsel(&mut self) -> COMP2_INMSEL_W<COMP2_CSRrs> {
        COMP2_INMSEL_W::new(self, 4)
    }
    #[doc = "Bit 7 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_inpsel(&mut self) -> COMP2_INPSEL_W<COMP2_CSRrs> {
        COMP2_INPSEL_W::new(self, 7)
    }
    #[doc = "Bit 9 - Windows mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_winmode(&mut self) -> COMP2_WINMODE_W<COMP2_CSRrs> {
        COMP2_WINMODE_W::new(self, 9)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_polarity(&mut self) -> COMP2_POLARITY_W<COMP2_CSRrs> {
        COMP2_POLARITY_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_hyst(&mut self) -> COMP2_HYST_W<COMP2_CSRrs> {
        COMP2_HYST_W::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_blanking(&mut self) -> COMP2_BLANKING_W<COMP2_CSRrs> {
        COMP2_BLANKING_W::new(self, 18)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_brgen(&mut self) -> COMP2_BRGEN_W<COMP2_CSRrs> {
        COMP2_BRGEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_scalen(&mut self) -> COMP2_SCALEN_W<COMP2_CSRrs> {
        COMP2_SCALEN_W::new(self, 23)
    }
    #[doc = "Bit 31 - COMP2_CSR register lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_lock(&mut self) -> COMP2_LOCK_W<COMP2_CSRrs> {
        COMP2_LOCK_W::new(self, 31)
    }
}
#[doc = "Comparator 2 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP2_CSRrs;
impl crate::RegisterSpec for COMP2_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp2_csr::R`](R) reader structure"]
impl crate::Readable for COMP2_CSRrs {}
#[doc = "`write(|w| ..)` method takes [`comp2_csr::W`](W) writer structure"]
impl crate::Writable for COMP2_CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for COMP2_CSRrs {
    const RESET_VALUE: u32 = 0;
}
