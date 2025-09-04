///Register `COMP2_CSR` reader
pub type R = crate::R<COMP2_CSRrs>;
///Register `COMP2_CSR` writer
pub type W = crate::W<COMP2_CSRrs>;
///Field `EN` reader - COMP1 enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - COMP1 enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INMSEL` reader - COMP1 signal selector for inverting input INM Controlled by software (if not locked), selects the signal for the inverting input COMP1_INM
pub type INMSEL_R = crate::FieldReader;
///Field `INMSEL` writer - COMP1 signal selector for inverting input INM Controlled by software (if not locked), selects the signal for the inverting input COMP1_INM
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `INPSEL` reader - COMP1 signal selector for inverting input INM Controlled by software (if not locked), selects the signal for the inverting input COMP1_INM
pub type INPSEL_R = crate::FieldReader;
///Field `INPSEL` writer - COMP1 signal selector for inverting input INM Controlled by software (if not locked), selects the signal for the inverting input COMP1_INM
pub type INPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WINMODE` reader - COMP1 noninverting input selector for window mode Controlled by software (if not locked), selects the signal for the COMP1_INP input of the COMP1.
pub type WINMODE_R = crate::BitReader;
///Field `WINMODE` writer - COMP1 noninverting input selector for window mode Controlled by software (if not locked), selects the signal for the COMP1_INP input of the COMP1.
pub type WINMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WINOUT` reader - COMP1 output selector Controlled by software (if not locked), selects the COMP1 output.
pub type WINOUT_R = crate::BitReader;
///Field `WINOUT` writer - COMP1 output selector Controlled by software (if not locked), selects the COMP1 output.
pub type WINOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POLARITY` reader - COMP1 polarity selector Controlled by software (if not locked), selects the COMP1 output polarity
pub type POLARITY_R = crate::BitReader;
///Field `POLARITY` writer - COMP1 polarity selector Controlled by software (if not locked), selects the COMP1 output polarity
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HYST` reader - COMP1 hysteresis selector Controlled by software (if not locked), selects the COMP1 hysteresis.
pub type HYST_R = crate::FieldReader;
///Field `HYST` writer - COMP1 hysteresis selector Controlled by software (if not locked), selects the COMP1 hysteresis.
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PWRMODE` reader - COMP1 power mode selector Controlled by software (if not locked), selects the power consumption and, as a consequence, the speed of the COMP1.
pub type PWRMODE_R = crate::FieldReader;
///Field `PWRMODE` writer - COMP1 power mode selector Controlled by software (if not locked), selects the power consumption and, as a consequence, the speed of the COMP1.
pub type PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BLANKSEL` reader - COMP1 blanking source selector.
pub type BLANKSEL_R = crate::FieldReader;
///Field `BLANKSEL` writer - COMP1 blanking source selector.
pub type BLANKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `VALUE` reader - COMP1 output status.
pub type VALUE_R = crate::BitReader;
///Field `LOCK` reader - COMP1_CSR register lock.
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - COMP1_CSR register lock.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - COMP1 enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:7 - COMP1 signal selector for inverting input INM Controlled by software (if not locked), selects the signal for the inverting input COMP1_INM
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - COMP1 signal selector for inverting input INM Controlled by software (if not locked), selects the signal for the inverting input COMP1_INM
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - COMP1 noninverting input selector for window mode Controlled by software (if not locked), selects the signal for the COMP1_INP input of the COMP1.
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - COMP1 output selector Controlled by software (if not locked), selects the COMP1 output.
    #[inline(always)]
    pub fn winout(&self) -> WINOUT_R {
        WINOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - COMP1 polarity selector Controlled by software (if not locked), selects the COMP1 output polarity
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - COMP1 hysteresis selector Controlled by software (if not locked), selects the COMP1 hysteresis.
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - COMP1 power mode selector Controlled by software (if not locked), selects the power consumption and, as a consequence, the speed of the COMP1.
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:24 - COMP1 blanking source selector.
    #[inline(always)]
    pub fn blanksel(&self) -> BLANKSEL_R {
        BLANKSEL_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bit 30 - COMP1 output status.
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - COMP1_CSR register lock.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP2_CSR")
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
    ///Bit 0 - COMP1 enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<COMP2_CSRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 4:7 - COMP1 signal selector for inverting input INM Controlled by software (if not locked), selects the signal for the inverting input COMP1_INM
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W<COMP2_CSRrs> {
        INMSEL_W::new(self, 4)
    }
    ///Bits 8:9 - COMP1 signal selector for inverting input INM Controlled by software (if not locked), selects the signal for the inverting input COMP1_INM
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W<COMP2_CSRrs> {
        INPSEL_W::new(self, 8)
    }
    ///Bit 11 - COMP1 noninverting input selector for window mode Controlled by software (if not locked), selects the signal for the COMP1_INP input of the COMP1.
    #[inline(always)]
    pub fn winmode(&mut self) -> WINMODE_W<COMP2_CSRrs> {
        WINMODE_W::new(self, 11)
    }
    ///Bit 14 - COMP1 output selector Controlled by software (if not locked), selects the COMP1 output.
    #[inline(always)]
    pub fn winout(&mut self) -> WINOUT_W<COMP2_CSRrs> {
        WINOUT_W::new(self, 14)
    }
    ///Bit 15 - COMP1 polarity selector Controlled by software (if not locked), selects the COMP1 output polarity
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W<COMP2_CSRrs> {
        POLARITY_W::new(self, 15)
    }
    ///Bits 16:17 - COMP1 hysteresis selector Controlled by software (if not locked), selects the COMP1 hysteresis.
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W<COMP2_CSRrs> {
        HYST_W::new(self, 16)
    }
    ///Bits 18:19 - COMP1 power mode selector Controlled by software (if not locked), selects the power consumption and, as a consequence, the speed of the COMP1.
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PWRMODE_W<COMP2_CSRrs> {
        PWRMODE_W::new(self, 18)
    }
    ///Bits 20:24 - COMP1 blanking source selector.
    #[inline(always)]
    pub fn blanksel(&mut self) -> BLANKSEL_W<COMP2_CSRrs> {
        BLANKSEL_W::new(self, 20)
    }
    ///Bit 31 - COMP1_CSR register lock.
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<COMP2_CSRrs> {
        LOCK_W::new(self, 31)
    }
}
/**COMP2 control and status register

You can [`read`](crate::Reg::read) this register and get [`comp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#COMP:COMP2_CSR)*/
pub struct COMP2_CSRrs;
impl crate::RegisterSpec for COMP2_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`comp2_csr::R`](R) reader structure
impl crate::Readable for COMP2_CSRrs {}
///`write(|w| ..)` method takes [`comp2_csr::W`](W) writer structure
impl crate::Writable for COMP2_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP2_CSR to value 0
impl crate::Resettable for COMP2_CSRrs {}
