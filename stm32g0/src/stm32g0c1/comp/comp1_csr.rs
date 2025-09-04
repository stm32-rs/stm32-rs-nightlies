///Register `COMP1_CSR` reader
pub type R = crate::R<COMP1_CSRrs>;
///Register `COMP1_CSR` writer
pub type W = crate::W<COMP1_CSRrs>;
///Field `EN` reader - Comparator 1 enable bit This bit is controlled by software (if not locked). It enables the comparator 1:
pub type EN_R = crate::BitReader;
///Field `EN` writer - Comparator 1 enable bit This bit is controlled by software (if not locked). It enables the comparator 1:
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INMSEL` reader - Comparator 1 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP1_INM of the comparator 1: > 1000: 1/4 VREFINT
pub type INMSEL_R = crate::FieldReader;
///Field `INMSEL` writer - Comparator 1 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP1_INM of the comparator 1: > 1000: 1/4 VREFINT
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `INPSEL` reader - Comparator 1 signal selector for non-inverting input This bitfield is controlled by software (if not locked). It selects the signal for the non-inverting input COMP1_INP of the comparator 1 (also see the WINMODE bit):
pub type INPSEL_R = crate::FieldReader;
///Field `INPSEL` writer - Comparator 1 signal selector for non-inverting input This bitfield is controlled by software (if not locked). It selects the signal for the non-inverting input COMP1_INP of the comparator 1 (also see the WINMODE bit):
pub type INPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WINMODE` reader - Comparator 1 non-inverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP1_INP input of the comparator 1:
pub type WINMODE_R = crate::BitReader;
///Field `WINMODE` writer - Comparator 1 non-inverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP1_INP input of the comparator 1:
pub type WINMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WINOUT` reader - Comparator 1 output selector This bit is controlled by software (if not locked). It selects the comparator 1 output:
pub type WINOUT_R = crate::BitReader;
///Field `WINOUT` writer - Comparator 1 output selector This bit is controlled by software (if not locked). It selects the comparator 1 output:
pub type WINOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POLARITY` reader - Comparator 1 polarity selector This bit is controlled by software (if not locked). It selects the comparator 1 output polarity:
pub type POLARITY_R = crate::BitReader;
///Field `POLARITY` writer - Comparator 1 polarity selector This bit is controlled by software (if not locked). It selects the comparator 1 output polarity:
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HYST` reader - Comparator 1 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 1:
pub type HYST_R = crate::FieldReader;
///Field `HYST` writer - Comparator 1 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 1:
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PWRMODE` reader - Comparator 1 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 1: others: Reserved
pub type PWRMODE_R = crate::FieldReader;
///Field `PWRMODE` writer - Comparator 1 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 1: others: Reserved
pub type PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BLANKSEL` reader - Comparator 1 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: xxxx1: TIM1 OC4 xxx1x: TIM1 OC5 xx1xx: TIM2 OC3 x1xxx: TIM3 OC3 1xxxx: TIM15 OC2
pub type BLANKSEL_R = crate::FieldReader;
///Field `BLANKSEL` writer - Comparator 1 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: xxxx1: TIM1 OC4 xxx1x: TIM1 OC5 xx1xx: TIM2 OC3 x1xxx: TIM3 OC3 1xxxx: TIM15 OC2
pub type BLANKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `VALUE` reader - Comparator 1 output status This bit is read-only. It reflects the level of the comparator 1 output after the polarity selector and blanking, as indicated in .
pub type VALUE_R = crate::BitReader;
///Field `LOCK` reader - COMP1_CSR register lock This bit is set by software and cleared by a system reset. It locks the whole content of the comparator 1 control register COMP1_CSR\[31:0\]:
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - COMP1_CSR register lock This bit is set by software and cleared by a system reset. It locks the whole content of the comparator 1 control register COMP1_CSR\[31:0\]:
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Comparator 1 enable bit This bit is controlled by software (if not locked). It enables the comparator 1:
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:7 - Comparator 1 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP1_INM of the comparator 1: > 1000: 1/4 VREFINT
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Comparator 1 signal selector for non-inverting input This bitfield is controlled by software (if not locked). It selects the signal for the non-inverting input COMP1_INP of the comparator 1 (also see the WINMODE bit):
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - Comparator 1 non-inverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP1_INP input of the comparator 1:
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - Comparator 1 output selector This bit is controlled by software (if not locked). It selects the comparator 1 output:
    #[inline(always)]
    pub fn winout(&self) -> WINOUT_R {
        WINOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Comparator 1 polarity selector This bit is controlled by software (if not locked). It selects the comparator 1 output polarity:
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 1 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 1:
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Comparator 1 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 1: others: Reserved
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:24 - Comparator 1 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: xxxx1: TIM1 OC4 xxx1x: TIM1 OC5 xx1xx: TIM2 OC3 x1xxx: TIM3 OC3 1xxxx: TIM15 OC2
    #[inline(always)]
    pub fn blanksel(&self) -> BLANKSEL_R {
        BLANKSEL_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bit 30 - Comparator 1 output status This bit is read-only. It reflects the level of the comparator 1 output after the polarity selector and blanking, as indicated in .
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - COMP1_CSR register lock This bit is set by software and cleared by a system reset. It locks the whole content of the comparator 1 control register COMP1_CSR\[31:0\]:
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP1_CSR")
            .field("en", &self.en())
            .field("inmsel", &self.inmsel())
            .field("inpsel", &self.inpsel())
            .field("winmode", &self.winmode())
            .field("winout", &self.winout())
            .field("polarity", &self.polarity())
            .field("hyst", &self.hyst())
            .field("pwrmode", &self.pwrmode())
            .field("blanksel", &self.blanksel())
            .field("value", &self.value())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator 1 enable bit This bit is controlled by software (if not locked). It enables the comparator 1:
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<COMP1_CSRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 4:7 - Comparator 1 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP1_INM of the comparator 1: > 1000: 1/4 VREFINT
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W<COMP1_CSRrs> {
        INMSEL_W::new(self, 4)
    }
    ///Bits 8:9 - Comparator 1 signal selector for non-inverting input This bitfield is controlled by software (if not locked). It selects the signal for the non-inverting input COMP1_INP of the comparator 1 (also see the WINMODE bit):
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W<COMP1_CSRrs> {
        INPSEL_W::new(self, 8)
    }
    ///Bit 11 - Comparator 1 non-inverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP1_INP input of the comparator 1:
    #[inline(always)]
    pub fn winmode(&mut self) -> WINMODE_W<COMP1_CSRrs> {
        WINMODE_W::new(self, 11)
    }
    ///Bit 14 - Comparator 1 output selector This bit is controlled by software (if not locked). It selects the comparator 1 output:
    #[inline(always)]
    pub fn winout(&mut self) -> WINOUT_W<COMP1_CSRrs> {
        WINOUT_W::new(self, 14)
    }
    ///Bit 15 - Comparator 1 polarity selector This bit is controlled by software (if not locked). It selects the comparator 1 output polarity:
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W<COMP1_CSRrs> {
        POLARITY_W::new(self, 15)
    }
    ///Bits 16:17 - Comparator 1 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 1:
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W<COMP1_CSRrs> {
        HYST_W::new(self, 16)
    }
    ///Bits 18:19 - Comparator 1 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 1: others: Reserved
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PWRMODE_W<COMP1_CSRrs> {
        PWRMODE_W::new(self, 18)
    }
    ///Bits 20:24 - Comparator 1 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: xxxx1: TIM1 OC4 xxx1x: TIM1 OC5 xx1xx: TIM2 OC3 x1xxx: TIM3 OC3 1xxxx: TIM15 OC2
    #[inline(always)]
    pub fn blanksel(&mut self) -> BLANKSEL_W<COMP1_CSRrs> {
        BLANKSEL_W::new(self, 20)
    }
    ///Bit 31 - COMP1_CSR register lock This bit is set by software and cleared by a system reset. It locks the whole content of the comparator 1 control register COMP1_CSR\[31:0\]:
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<COMP1_CSRrs> {
        LOCK_W::new(self, 31)
    }
}
/**Comparator 1 control and status register

You can [`read`](crate::Reg::read) this register and get [`comp1_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#COMP:COMP1_CSR)*/
pub struct COMP1_CSRrs;
impl crate::RegisterSpec for COMP1_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`comp1_csr::R`](R) reader structure
impl crate::Readable for COMP1_CSRrs {}
///`write(|w| ..)` method takes [`comp1_csr::W`](W) writer structure
impl crate::Writable for COMP1_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP1_CSR to value 0
impl crate::Resettable for COMP1_CSRrs {}
