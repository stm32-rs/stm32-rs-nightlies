///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `EN` reader - EN: Comparator enable bit This bit is set and cleared by software (only if LOCK not set). It switches on Comparator. 0: Comparator switched OFF 1: Comparator switched ON
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN: Comparator enable bit This bit is set and cleared by software (only if LOCK not set). It switches on Comparator. 0: Comparator switched OFF 1: Comparator switched ON
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRMODE` reader - PWRMODE\[1:0\]: Power Mode of the comparator These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the Comparator. 00:High speed 01 or 10:Medium speed 11:Ultra low power
pub type PWRMODE_R = crate::FieldReader;
///Field `PWRMODE` writer - PWRMODE\[1:0\]: Power Mode of the comparator These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the Comparator. 00:High speed 01 or 10:Medium speed 11:Ultra low power
pub type PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INMSEL` reader - INMSEL: Comparator input minus selection bits These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of comparator. 000: 1/4 VREFINT 001: 1/2 VREFINT 010: 3/4VREFINT 011: VREFINT 100: DAC OUT 101: PA13 110: PB0 111: PB3
pub type INMSEL_R = crate::FieldReader;
///Field `INMSEL` writer - INMSEL: Comparator input minus selection bits These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of comparator. 000: 1/4 VREFINT 001: 1/2 VREFINT 010: 3/4VREFINT 011: VREFINT 100: DAC OUT 101: PA13 110: PB0 111: PB3
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `INPSEL` reader - INPSEL\[1:0\]: Comparator input plus selection bit This bit is set and cleared by software (only if LOCK not set). 00: PA14 01: PB1 1x: PB2
pub type INPSEL_R = crate::FieldReader;
///Field `INPSEL` writer - INPSEL\[1:0\]: Comparator input plus selection bit This bit is set and cleared by software (only if LOCK not set). 00: PA14 01: PB1 1x: PB2
pub type INPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `POLARITY` reader - POLARITY: Comparator polarity selection bit This bit is set and cleared by software (only if LOCK not set). It inverts Comparator polarity. 0: Comparator output value not inverted 1: Comparator output value inverted
pub type POLARITY_R = crate::BitReader;
///Field `POLARITY` writer - POLARITY: Comparator polarity selection bit This bit is set and cleared by software (only if LOCK not set). It inverts Comparator polarity. 0: Comparator output value not inverted 1: Comparator output value inverted
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HYST` reader - HYST\[1:0\]: Comparator hysteresis selection bits These bits are set and cleared by software (only if LOCK not set). They select the Hysteresis voltage of the comparator . 00: No hysteresis 01: Low hysteresis 10: Medium hysteresis 11: High hysteresis
pub type HYST_R = crate::FieldReader;
///Field `HYST` writer - HYST\[1:0\]: Comparator hysteresis selection bits These bits are set and cleared by software (only if LOCK not set). They select the Hysteresis voltage of the comparator . 00: No hysteresis 01: Low hysteresis 10: Medium hysteresis 11: High hysteresis
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BLANKING` reader - BLANKING\[2:0\]: Comparator blanking source selection bits These bits select which timer output controls the comparator output blanking. 000: No blanking 001: TIM2 OC4 selected as blanking source 010: TIM16 OC1 selected as blanking source All other values: reserved
pub type BLANKING_R = crate::FieldReader;
///Field `BLANKING` writer - BLANKING\[2:0\]: Comparator blanking source selection bits These bits select which timer output controls the comparator output blanking. 000: No blanking 001: TIM2 OC4 selected as blanking source 010: TIM16 OC1 selected as blanking source All other values: reserved
pub type BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BRGEN` reader - BRGEN: Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enable the bridge of the scaler. 0: Scaler resistor bridge disable 1: Scaler resistor bridge enable If SCALEN is set and BRGEN is reset, BG voltage reference is available but not 1/4BGAP, 1/2BGAP, 3/4 BGAP. BGAP value is sent instead of 1/4BGAP, 1/2BGAP, 3/4 BGAP. If SCALEN and BRGEN are set, 1/4 BGAP 1/2BGAP 3/4BGAP and BGAP voltage references are available.
pub type BRGEN_R = crate::BitReader;
///Field `BRGEN` writer - BRGEN: Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enable the bridge of the scaler. 0: Scaler resistor bridge disable 1: Scaler resistor bridge enable If SCALEN is set and BRGEN is reset, BG voltage reference is available but not 1/4BGAP, 1/2BGAP, 3/4 BGAP. BGAP value is sent instead of 1/4BGAP, 1/2BGAP, 3/4 BGAP. If SCALEN and BRGEN are set, 1/4 BGAP 1/2BGAP 3/4BGAP and BGAP voltage references are available.
pub type BRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCALEN` reader - SCALEN: Voltage scaler enable bit This bit is set and cleared by software. This bit enable the outputs of the VREFINT divider available on the minus input of the Comparator 0: scaler disable 1: scaler enable
pub type SCALEN_R = crate::BitReader;
///Field `SCALEN` writer - SCALEN: Voltage scaler enable bit This bit is set and cleared by software. This bit enable the outputs of the VREFINT divider available on the minus input of the Comparator 0: scaler disable 1: scaler enable
pub type SCALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VALUE` reader - VALUE: Comparator output status bit This bit is read-only. It reflects the current comparator output taking into account POLARITY bit effect.
pub type VALUE_R = crate::BitReader;
///Field `LOCK` reader - LOCK: COMP_CSR register lock bit This bit is set by software and cleared by a hardware system reset. It locks the whole content of the comparator control register, COMP1_CSR\[31:0\]. 0: COMP1_CSR\[31:0\] are read/write 1: COMP1_CSR\[31:0\] are read-only
pub type LOCK_R = crate::BitReader;
impl R {
    ///Bit 0 - EN: Comparator enable bit This bit is set and cleared by software (only if LOCK not set). It switches on Comparator. 0: Comparator switched OFF 1: Comparator switched ON
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - PWRMODE\[1:0\]: Power Mode of the comparator These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the Comparator. 00:High speed 01 or 10:Medium speed 11:Ultra low power
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - INMSEL: Comparator input minus selection bits These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of comparator. 000: 1/4 VREFINT 001: 1/2 VREFINT 010: 3/4VREFINT 011: VREFINT 100: DAC OUT 101: PA13 110: PB0 111: PB3
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:8 - INPSEL\[1:0\]: Comparator input plus selection bit This bit is set and cleared by software (only if LOCK not set). 00: PA14 01: PB1 1x: PB2
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 15 - POLARITY: Comparator polarity selection bit This bit is set and cleared by software (only if LOCK not set). It inverts Comparator polarity. 0: Comparator output value not inverted 1: Comparator output value inverted
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - HYST\[1:0\]: Comparator hysteresis selection bits These bits are set and cleared by software (only if LOCK not set). They select the Hysteresis voltage of the comparator . 00: No hysteresis 01: Low hysteresis 10: Medium hysteresis 11: High hysteresis
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - BLANKING\[2:0\]: Comparator blanking source selection bits These bits select which timer output controls the comparator output blanking. 000: No blanking 001: TIM2 OC4 selected as blanking source 010: TIM16 OC1 selected as blanking source All other values: reserved
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 22 - BRGEN: Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enable the bridge of the scaler. 0: Scaler resistor bridge disable 1: Scaler resistor bridge enable If SCALEN is set and BRGEN is reset, BG voltage reference is available but not 1/4BGAP, 1/2BGAP, 3/4 BGAP. BGAP value is sent instead of 1/4BGAP, 1/2BGAP, 3/4 BGAP. If SCALEN and BRGEN are set, 1/4 BGAP 1/2BGAP 3/4BGAP and BGAP voltage references are available.
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SCALEN: Voltage scaler enable bit This bit is set and cleared by software. This bit enable the outputs of the VREFINT divider available on the minus input of the Comparator 0: scaler disable 1: scaler enable
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 30 - VALUE: Comparator output status bit This bit is read-only. It reflects the current comparator output taking into account POLARITY bit effect.
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - LOCK: COMP_CSR register lock bit This bit is set by software and cleared by a hardware system reset. It locks the whole content of the comparator control register, COMP1_CSR\[31:0\]. 0: COMP1_CSR\[31:0\] are read/write 1: COMP1_CSR\[31:0\] are read-only
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("en", &self.en())
            .field("pwrmode", &self.pwrmode())
            .field("inmsel", &self.inmsel())
            .field("inpsel", &self.inpsel())
            .field("polarity", &self.polarity())
            .field("hyst", &self.hyst())
            .field("blanking", &self.blanking())
            .field("brgen", &self.brgen())
            .field("scalen", &self.scalen())
            .field("value", &self.value())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN: Comparator enable bit This bit is set and cleared by software (only if LOCK not set). It switches on Comparator. 0: Comparator switched OFF 1: Comparator switched ON
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CSRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 2:3 - PWRMODE\[1:0\]: Power Mode of the comparator These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the Comparator. 00:High speed 01 or 10:Medium speed 11:Ultra low power
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PWRMODE_W<'_, CSRrs> {
        PWRMODE_W::new(self, 2)
    }
    ///Bits 4:6 - INMSEL: Comparator input minus selection bits These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of comparator. 000: 1/4 VREFINT 001: 1/2 VREFINT 010: 3/4VREFINT 011: VREFINT 100: DAC OUT 101: PA13 110: PB0 111: PB3
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W<'_, CSRrs> {
        INMSEL_W::new(self, 4)
    }
    ///Bits 7:8 - INPSEL\[1:0\]: Comparator input plus selection bit This bit is set and cleared by software (only if LOCK not set). 00: PA14 01: PB1 1x: PB2
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W<'_, CSRrs> {
        INPSEL_W::new(self, 7)
    }
    ///Bit 15 - POLARITY: Comparator polarity selection bit This bit is set and cleared by software (only if LOCK not set). It inverts Comparator polarity. 0: Comparator output value not inverted 1: Comparator output value inverted
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W<'_, CSRrs> {
        POLARITY_W::new(self, 15)
    }
    ///Bits 16:17 - HYST\[1:0\]: Comparator hysteresis selection bits These bits are set and cleared by software (only if LOCK not set). They select the Hysteresis voltage of the comparator . 00: No hysteresis 01: Low hysteresis 10: Medium hysteresis 11: High hysteresis
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W<'_, CSRrs> {
        HYST_W::new(self, 16)
    }
    ///Bits 18:20 - BLANKING\[2:0\]: Comparator blanking source selection bits These bits select which timer output controls the comparator output blanking. 000: No blanking 001: TIM2 OC4 selected as blanking source 010: TIM16 OC1 selected as blanking source All other values: reserved
    #[inline(always)]
    pub fn blanking(&mut self) -> BLANKING_W<'_, CSRrs> {
        BLANKING_W::new(self, 18)
    }
    ///Bit 22 - BRGEN: Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enable the bridge of the scaler. 0: Scaler resistor bridge disable 1: Scaler resistor bridge enable If SCALEN is set and BRGEN is reset, BG voltage reference is available but not 1/4BGAP, 1/2BGAP, 3/4 BGAP. BGAP value is sent instead of 1/4BGAP, 1/2BGAP, 3/4 BGAP. If SCALEN and BRGEN are set, 1/4 BGAP 1/2BGAP 3/4BGAP and BGAP voltage references are available.
    #[inline(always)]
    pub fn brgen(&mut self) -> BRGEN_W<'_, CSRrs> {
        BRGEN_W::new(self, 22)
    }
    ///Bit 23 - SCALEN: Voltage scaler enable bit This bit is set and cleared by software. This bit enable the outputs of the VREFINT divider available on the minus input of the Comparator 0: scaler disable 1: scaler enable
    #[inline(always)]
    pub fn scalen(&mut self) -> SCALEN_W<'_, CSRrs> {
        SCALEN_W::new(self, 23)
    }
}
/**CSR register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#COMP:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
