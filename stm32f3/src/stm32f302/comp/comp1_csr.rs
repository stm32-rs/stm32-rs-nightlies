///Register `COMP1_CSR` reader
pub type R = crate::R<COMP1_CSRrs>;
///Register `COMP1_CSR` writer
pub type W = crate::W<COMP1_CSRrs>;
///Field `COMP1EN` reader - Comparator 1 enable
pub type COMP1EN_R = crate::BitReader;
///Field `COMP1EN` writer - Comparator 1 enable
pub type COMP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP1_INP_DAC` reader - Comparator 1 non inverting input connection to DAC output
pub type COMP1_INP_DAC_R = crate::BitReader;
///Field `COMP1_INP_DAC` writer - Comparator 1 non inverting input connection to DAC output
pub type COMP1_INP_DAC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP1MODE` reader - Comparator 1 mode
pub type COMP1MODE_R = crate::FieldReader;
///Field `COMP1MODE` writer - Comparator 1 mode
pub type COMP1MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP1INMSEL` reader - Comparator 1 inverting input selection
pub type COMP1INMSEL_R = crate::FieldReader;
///Field `COMP1INMSEL` writer - Comparator 1 inverting input selection
pub type COMP1INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMP1OUTSEL` reader - Comparator 1 output selection
pub type COMP1OUTSEL_R = crate::FieldReader;
///Field `COMP1OUTSEL` writer - Comparator 1 output selection
pub type COMP1OUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `COMP1POL` reader - Comparator 1 output polarity
pub type COMP1POL_R = crate::BitReader;
///Field `COMP1POL` writer - Comparator 1 output polarity
pub type COMP1POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP1HYST` reader - Comparator 1 hysteresis
pub type COMP1HYST_R = crate::FieldReader;
///Field `COMP1HYST` writer - Comparator 1 hysteresis
pub type COMP1HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP1_BLANKING` reader - Comparator 1 blanking source
pub type COMP1_BLANKING_R = crate::FieldReader;
///Field `COMP1_BLANKING` writer - Comparator 1 blanking source
pub type COMP1_BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMP1OUT` reader - Comparator 1 output
pub type COMP1OUT_R = crate::BitReader;
///Field `COMP1LOCK` reader - Comparator 1 lock
pub type COMP1LOCK_R = crate::BitReader;
///Field `COMP1LOCK` writer - Comparator 1 lock
pub type COMP1LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Comparator 1 enable
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Comparator 1 non inverting input connection to DAC output
    #[inline(always)]
    pub fn comp1_inp_dac(&self) -> COMP1_INP_DAC_R {
        COMP1_INP_DAC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Comparator 1 mode
    #[inline(always)]
    pub fn comp1mode(&self) -> COMP1MODE_R {
        COMP1MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Comparator 1 inverting input selection
    #[inline(always)]
    pub fn comp1inmsel(&self) -> COMP1INMSEL_R {
        COMP1INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 10:13 - Comparator 1 output selection
    #[inline(always)]
    pub fn comp1outsel(&self) -> COMP1OUTSEL_R {
        COMP1OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 15 - Comparator 1 output polarity
    #[inline(always)]
    pub fn comp1pol(&self) -> COMP1POL_R {
        COMP1POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 1 hysteresis
    #[inline(always)]
    pub fn comp1hyst(&self) -> COMP1HYST_R {
        COMP1HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - Comparator 1 blanking source
    #[inline(always)]
    pub fn comp1_blanking(&self) -> COMP1_BLANKING_R {
        COMP1_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 30 - Comparator 1 output
    #[inline(always)]
    pub fn comp1out(&self) -> COMP1OUT_R {
        COMP1OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Comparator 1 lock
    #[inline(always)]
    pub fn comp1lock(&self) -> COMP1LOCK_R {
        COMP1LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP1_CSR")
            .field("comp1en", &self.comp1en())
            .field("comp1_inp_dac", &self.comp1_inp_dac())
            .field("comp1mode", &self.comp1mode())
            .field("comp1inmsel", &self.comp1inmsel())
            .field("comp1outsel", &self.comp1outsel())
            .field("comp1pol", &self.comp1pol())
            .field("comp1hyst", &self.comp1hyst())
            .field("comp1_blanking", &self.comp1_blanking())
            .field("comp1out", &self.comp1out())
            .field("comp1lock", &self.comp1lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator 1 enable
    #[inline(always)]
    pub fn comp1en(&mut self) -> COMP1EN_W<'_, COMP1_CSRrs> {
        COMP1EN_W::new(self, 0)
    }
    ///Bit 1 - Comparator 1 non inverting input connection to DAC output
    #[inline(always)]
    pub fn comp1_inp_dac(&mut self) -> COMP1_INP_DAC_W<'_, COMP1_CSRrs> {
        COMP1_INP_DAC_W::new(self, 1)
    }
    ///Bits 2:3 - Comparator 1 mode
    #[inline(always)]
    pub fn comp1mode(&mut self) -> COMP1MODE_W<'_, COMP1_CSRrs> {
        COMP1MODE_W::new(self, 2)
    }
    ///Bits 4:6 - Comparator 1 inverting input selection
    #[inline(always)]
    pub fn comp1inmsel(&mut self) -> COMP1INMSEL_W<'_, COMP1_CSRrs> {
        COMP1INMSEL_W::new(self, 4)
    }
    ///Bits 10:13 - Comparator 1 output selection
    #[inline(always)]
    pub fn comp1outsel(&mut self) -> COMP1OUTSEL_W<'_, COMP1_CSRrs> {
        COMP1OUTSEL_W::new(self, 10)
    }
    ///Bit 15 - Comparator 1 output polarity
    #[inline(always)]
    pub fn comp1pol(&mut self) -> COMP1POL_W<'_, COMP1_CSRrs> {
        COMP1POL_W::new(self, 15)
    }
    ///Bits 16:17 - Comparator 1 hysteresis
    #[inline(always)]
    pub fn comp1hyst(&mut self) -> COMP1HYST_W<'_, COMP1_CSRrs> {
        COMP1HYST_W::new(self, 16)
    }
    ///Bits 18:20 - Comparator 1 blanking source
    #[inline(always)]
    pub fn comp1_blanking(&mut self) -> COMP1_BLANKING_W<'_, COMP1_CSRrs> {
        COMP1_BLANKING_W::new(self, 18)
    }
    ///Bit 31 - Comparator 1 lock
    #[inline(always)]
    pub fn comp1lock(&mut self) -> COMP1LOCK_W<'_, COMP1_CSRrs> {
        COMP1LOCK_W::new(self, 31)
    }
}
/**control and status register

You can [`read`](crate::Reg::read) this register and get [`comp1_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#COMP:COMP1_CSR)*/
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
