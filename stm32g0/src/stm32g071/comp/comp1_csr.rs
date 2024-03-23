#[doc = "Register `COMP1_CSR` reader"]
pub type R = crate::R<COMP1_CSRrs>;
#[doc = "Register `COMP1_CSR` writer"]
pub type W = crate::W<COMP1_CSRrs>;
#[doc = "Field `EN` reader - COMP channel 1 enable bit"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - COMP channel 1 enable bit"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INMSEL` reader - Comparator 2 signal selector for inverting input INM"]
pub type INMSEL_R = crate::FieldReader;
#[doc = "Field `INMSEL` writer - Comparator 2 signal selector for inverting input INM"]
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INPSEL` reader - Comparator 2 signal selector for non-inverting input"]
pub type INPSEL_R = crate::FieldReader;
#[doc = "Field `INPSEL` writer - Comparator 2 signal selector for non-inverting input"]
pub type INPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WINMODE` reader - Comparator 2 non-inverting input selector for window mode"]
pub type WINMODE_R = crate::BitReader;
#[doc = "Field `WINMODE` writer - Comparator 2 non-inverting input selector for window mode"]
pub type WINMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINOUT` reader - Comparator 2 output selector"]
pub type WINOUT_R = crate::BitReader;
#[doc = "Field `WINOUT` writer - Comparator 2 output selector"]
pub type WINOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLARITY` reader - Comparator 2 polarity selector"]
pub type POLARITY_R = crate::BitReader;
#[doc = "Field `POLARITY` writer - Comparator 2 polarity selector"]
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Comparator 2 hysteresis selector"]
pub type HYST_R = crate::FieldReader;
#[doc = "Field `HYST` writer - Comparator 2 hysteresis selector"]
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWRMODE` reader - Comparator 2 power mode selector"]
pub type PWRMODE_R = crate::FieldReader;
#[doc = "Field `PWRMODE` writer - Comparator 2 power mode selector"]
pub type PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLANKSEL` reader - Comparator 2 blanking source selector"]
pub type BLANKSEL_R = crate::FieldReader;
#[doc = "Field `BLANKSEL` writer - Comparator 2 blanking source selector"]
pub type BLANKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VALUE` reader - Comparator 2 output status"]
pub type VALUE_R = crate::BitReader;
#[doc = "Field `VALUE` writer - Comparator 2 output status"]
pub type VALUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - COMP2_CSR register lock"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - COMP2_CSR register lock"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - COMP channel 1 enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Comparator 2 signal selector for inverting input INM"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Comparator 2 signal selector for non-inverting input"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Comparator 2 non-inverting input selector for window mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Comparator 2 output selector"]
    #[inline(always)]
    pub fn winout(&self) -> WINOUT_R {
        WINOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selector"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selector"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Comparator 2 power mode selector"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:24 - Comparator 2 blanking source selector"]
    #[inline(always)]
    pub fn blanksel(&self) -> BLANKSEL_R {
        BLANKSEL_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Comparator 2 output status"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP2_CSR register lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COMP channel 1 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<COMP1_CSRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Comparator 2 signal selector for inverting input INM"]
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<COMP1_CSRrs> {
        INMSEL_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Comparator 2 signal selector for non-inverting input"]
    #[inline(always)]
    #[must_use]
    pub fn inpsel(&mut self) -> INPSEL_W<COMP1_CSRrs> {
        INPSEL_W::new(self, 8)
    }
    #[doc = "Bit 11 - Comparator 2 non-inverting input selector for window mode"]
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WINMODE_W<COMP1_CSRrs> {
        WINMODE_W::new(self, 11)
    }
    #[doc = "Bit 14 - Comparator 2 output selector"]
    #[inline(always)]
    #[must_use]
    pub fn winout(&mut self) -> WINOUT_W<COMP1_CSRrs> {
        WINOUT_W::new(self, 14)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selector"]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<COMP1_CSRrs> {
        POLARITY_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selector"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<COMP1_CSRrs> {
        HYST_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Comparator 2 power mode selector"]
    #[inline(always)]
    #[must_use]
    pub fn pwrmode(&mut self) -> PWRMODE_W<COMP1_CSRrs> {
        PWRMODE_W::new(self, 18)
    }
    #[doc = "Bits 20:24 - Comparator 2 blanking source selector"]
    #[inline(always)]
    #[must_use]
    pub fn blanksel(&mut self) -> BLANKSEL_W<COMP1_CSRrs> {
        BLANKSEL_W::new(self, 20)
    }
    #[doc = "Bit 30 - Comparator 2 output status"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<COMP1_CSRrs> {
        VALUE_W::new(self, 30)
    }
    #[doc = "Bit 31 - COMP2_CSR register lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<COMP1_CSRrs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "Comparator 1 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
