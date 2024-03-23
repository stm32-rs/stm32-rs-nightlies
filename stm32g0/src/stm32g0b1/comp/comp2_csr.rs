#[doc = "Register `COMP2_CSR` reader"]
pub type R = crate::R<COMP2_CSRrs>;
#[doc = "Register `COMP2_CSR` writer"]
pub type W = crate::W<COMP2_CSRrs>;
#[doc = "Field `EN` reader - Comparator 2 enable bit This bit is controlled by software (if not locked). It enables the comparator 2:"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Comparator 2 enable bit This bit is controlled by software (if not locked). It enables the comparator 2:"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INMSEL` reader - Comparator 2 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP2_INM of the comparator 2: > 1000: 1/4 VREFINT"]
pub type INMSEL_R = crate::FieldReader;
#[doc = "Field `INMSEL` writer - Comparator 2 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP2_INM of the comparator 2: > 1000: 1/4 VREFINT"]
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INPSEL` reader - Comparator 2 signal selector for non-inverting input This bitfield is controlled by software (if not locked). It selects the signal for the non-inverting input COMP2_INP of the comparator 2 (also see the WINMODE bit):"]
pub type INPSEL_R = crate::FieldReader;
#[doc = "Field `INPSEL` writer - Comparator 2 signal selector for non-inverting input This bitfield is controlled by software (if not locked). It selects the signal for the non-inverting input COMP2_INP of the comparator 2 (also see the WINMODE bit):"]
pub type INPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WINMODE` reader - Comparator 2 non-inverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP2_INP input of the comparator 2:"]
pub type WINMODE_R = crate::BitReader;
#[doc = "Field `WINMODE` writer - Comparator 2 non-inverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP2_INP input of the comparator 2:"]
pub type WINMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINOUT` reader - Comparator 2 output selector This bit is controlled by software (if not locked). It selects the comparator 2 output:"]
pub type WINOUT_R = crate::BitReader;
#[doc = "Field `WINOUT` writer - Comparator 2 output selector This bit is controlled by software (if not locked). It selects the comparator 2 output:"]
pub type WINOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLARITY` reader - Comparator 2 polarity selector This bit is controlled by software (if not locked). It selects the comparator 2 output polarity:"]
pub type POLARITY_R = crate::BitReader;
#[doc = "Field `POLARITY` writer - Comparator 2 polarity selector This bit is controlled by software (if not locked). It selects the comparator 2 output polarity:"]
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Comparator 2 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 2:"]
pub type HYST_R = crate::FieldReader;
#[doc = "Field `HYST` writer - Comparator 2 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 2:"]
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWRMODE` reader - Comparator 2 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 2: others: Reserved"]
pub type PWRMODE_R = crate::FieldReader;
#[doc = "Field `PWRMODE` writer - Comparator 2 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 2: others: Reserved"]
pub type PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLANKSEL` reader - Comparator 2 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: xxxx1: TIM1 OC4 xxx1x: TIM1 OC5 xx1xx: TIM2 OC3 x1xxx: TIM3 OC3 1xxxx: TIM15 OC2"]
pub type BLANKSEL_R = crate::FieldReader;
#[doc = "Field `BLANKSEL` writer - Comparator 2 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: xxxx1: TIM1 OC4 xxx1x: TIM1 OC5 xx1xx: TIM2 OC3 x1xxx: TIM3 OC3 1xxxx: TIM15 OC2"]
pub type BLANKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VALUE` reader - Comparator 2 output status This bit is read-only. It reflects the level of the comparator 2 output after the polarity selector and blanking, as indicated in ."]
pub type VALUE_R = crate::BitReader;
#[doc = "Field `LOCK` reader - COMP2_CSR register lock This bit is set by software and cleared by a system reset. It locks the whole content of the comparator 2 control register COMP2_CSR\\[31:0\\]:"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - COMP2_CSR register lock This bit is set by software and cleared by a system reset. It locks the whole content of the comparator 2 control register COMP2_CSR\\[31:0\\]:"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 2 enable bit This bit is controlled by software (if not locked). It enables the comparator 2:"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Comparator 2 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP2_INM of the comparator 2: > 1000: 1/4 VREFINT"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Comparator 2 signal selector for non-inverting input This bitfield is controlled by software (if not locked). It selects the signal for the non-inverting input COMP2_INP of the comparator 2 (also see the WINMODE bit):"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Comparator 2 non-inverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP2_INP input of the comparator 2:"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Comparator 2 output selector This bit is controlled by software (if not locked). It selects the comparator 2 output:"]
    #[inline(always)]
    pub fn winout(&self) -> WINOUT_R {
        WINOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selector This bit is controlled by software (if not locked). It selects the comparator 2 output polarity:"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 2:"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Comparator 2 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 2: others: Reserved"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:24 - Comparator 2 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: xxxx1: TIM1 OC4 xxx1x: TIM1 OC5 xx1xx: TIM2 OC3 x1xxx: TIM3 OC3 1xxxx: TIM15 OC2"]
    #[inline(always)]
    pub fn blanksel(&self) -> BLANKSEL_R {
        BLANKSEL_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Comparator 2 output status This bit is read-only. It reflects the level of the comparator 2 output after the polarity selector and blanking, as indicated in ."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP2_CSR register lock This bit is set by software and cleared by a system reset. It locks the whole content of the comparator 2 control register COMP2_CSR\\[31:0\\]:"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable bit This bit is controlled by software (if not locked). It enables the comparator 2:"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<COMP2_CSRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Comparator 2 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP2_INM of the comparator 2: > 1000: 1/4 VREFINT"]
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<COMP2_CSRrs> {
        INMSEL_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Comparator 2 signal selector for non-inverting input This bitfield is controlled by software (if not locked). It selects the signal for the non-inverting input COMP2_INP of the comparator 2 (also see the WINMODE bit):"]
    #[inline(always)]
    #[must_use]
    pub fn inpsel(&mut self) -> INPSEL_W<COMP2_CSRrs> {
        INPSEL_W::new(self, 8)
    }
    #[doc = "Bit 11 - Comparator 2 non-inverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP2_INP input of the comparator 2:"]
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WINMODE_W<COMP2_CSRrs> {
        WINMODE_W::new(self, 11)
    }
    #[doc = "Bit 14 - Comparator 2 output selector This bit is controlled by software (if not locked). It selects the comparator 2 output:"]
    #[inline(always)]
    #[must_use]
    pub fn winout(&mut self) -> WINOUT_W<COMP2_CSRrs> {
        WINOUT_W::new(self, 14)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selector This bit is controlled by software (if not locked). It selects the comparator 2 output polarity:"]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<COMP2_CSRrs> {
        POLARITY_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 2:"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<COMP2_CSRrs> {
        HYST_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Comparator 2 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 2: others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pwrmode(&mut self) -> PWRMODE_W<COMP2_CSRrs> {
        PWRMODE_W::new(self, 18)
    }
    #[doc = "Bits 20:24 - Comparator 2 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: xxxx1: TIM1 OC4 xxx1x: TIM1 OC5 xx1xx: TIM2 OC3 x1xxx: TIM3 OC3 1xxxx: TIM15 OC2"]
    #[inline(always)]
    #[must_use]
    pub fn blanksel(&mut self) -> BLANKSEL_W<COMP2_CSRrs> {
        BLANKSEL_W::new(self, 20)
    }
    #[doc = "Bit 31 - COMP2_CSR register lock This bit is set by software and cleared by a system reset. It locks the whole content of the comparator 2 control register COMP2_CSR\\[31:0\\]:"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<COMP2_CSRrs> {
        LOCK_W::new(self, 31)
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
