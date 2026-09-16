///Register `COMP2_CSR` reader
pub type R = crate::R<COMP2_CSRrs>;
///Register `COMP2_CSR` writer
pub type W = crate::W<COMP2_CSRrs>;
///Field `COMP2_EN` reader - Comparator 2 enable bit
pub type COMP2_EN_R = crate::BitReader;
///Field `COMP2_EN` writer - Comparator 2 enable bit
pub type COMP2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP2_PWRMODE` reader - Power Mode of the comparator 2
pub type COMP2_PWRMODE_R = crate::FieldReader;
///Field `COMP2_PWRMODE` writer - Power Mode of the comparator 2
pub type COMP2_PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP2_INMSEL` reader - Comparator 2 input minus selection bits
pub type COMP2_INMSEL_R = crate::FieldReader;
///Field `COMP2_INMSEL` writer - Comparator 2 input minus selection bits
pub type COMP2_INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP2_INPSEL` reader - Comparator 1 input plus selection bit
pub type COMP2_INPSEL_R = crate::FieldReader;
///Field `COMP2_INPSEL` writer - Comparator 1 input plus selection bit
pub type COMP2_INPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP2_WINMODE` reader - Windows mode selection bit
pub type COMP2_WINMODE_R = crate::BitReader;
///Field `COMP2_WINMODE` writer - Windows mode selection bit
pub type COMP2_WINMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP2_POLARITY` reader - Comparator 2 polarity selection bit
pub type COMP2_POLARITY_R = crate::BitReader;
///Field `COMP2_POLARITY` writer - Comparator 2 polarity selection bit
pub type COMP2_POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP2_HYST` reader - Comparator 2 hysteresis selection bits
pub type COMP2_HYST_R = crate::FieldReader;
///Field `COMP2_HYST` writer - Comparator 2 hysteresis selection bits
pub type COMP2_HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP2_BLANKING` reader - Comparator 2 blanking source selection bits
pub type COMP2_BLANKING_R = crate::FieldReader;
///Field `COMP2_BLANKING` writer - Comparator 2 blanking source selection bits
pub type COMP2_BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMP2_BRGEN` reader - Scaler bridge enable
pub type COMP2_BRGEN_R = crate::BitReader;
///Field `COMP2_BRGEN` writer - Scaler bridge enable
pub type COMP2_BRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP2_SCALEN` reader - Voltage scaler enable bit
pub type COMP2_SCALEN_R = crate::BitReader;
///Field `COMP2_SCALEN` writer - Voltage scaler enable bit
pub type COMP2_SCALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP2_INMESEL` reader - comparator 2 input minus extended selection bits.
pub type COMP2_INMESEL_R = crate::FieldReader;
///Field `COMP2_INMESEL` writer - comparator 2 input minus extended selection bits.
pub type COMP2_INMESEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP2_VALUE` reader - Comparator 2 output status bit
pub type COMP2_VALUE_R = crate::BitReader;
///Field `COMP2_LOCK` reader - CSR register lock bit
pub type COMP2_LOCK_R = crate::BitReader;
///Field `COMP2_LOCK` writer - CSR register lock bit
pub type COMP2_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Comparator 2 enable bit
    #[inline(always)]
    pub fn comp2_en(&self) -> COMP2_EN_R {
        COMP2_EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Power Mode of the comparator 2
    #[inline(always)]
    pub fn comp2_pwrmode(&self) -> COMP2_PWRMODE_R {
        COMP2_PWRMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Comparator 2 input minus selection bits
    #[inline(always)]
    pub fn comp2_inmsel(&self) -> COMP2_INMSEL_R {
        COMP2_INMSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 7:8 - Comparator 1 input plus selection bit
    #[inline(always)]
    pub fn comp2_inpsel(&self) -> COMP2_INPSEL_R {
        COMP2_INPSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - Windows mode selection bit
    #[inline(always)]
    pub fn comp2_winmode(&self) -> COMP2_WINMODE_R {
        COMP2_WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - Comparator 2 polarity selection bit
    #[inline(always)]
    pub fn comp2_polarity(&self) -> COMP2_POLARITY_R {
        COMP2_POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 2 hysteresis selection bits
    #[inline(always)]
    pub fn comp2_hyst(&self) -> COMP2_HYST_R {
        COMP2_HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - Comparator 2 blanking source selection bits
    #[inline(always)]
    pub fn comp2_blanking(&self) -> COMP2_BLANKING_R {
        COMP2_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 22 - Scaler bridge enable
    #[inline(always)]
    pub fn comp2_brgen(&self) -> COMP2_BRGEN_R {
        COMP2_BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Voltage scaler enable bit
    #[inline(always)]
    pub fn comp2_scalen(&self) -> COMP2_SCALEN_R {
        COMP2_SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 25:26 - comparator 2 input minus extended selection bits.
    #[inline(always)]
    pub fn comp2_inmesel(&self) -> COMP2_INMESEL_R {
        COMP2_INMESEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 30 - Comparator 2 output status bit
    #[inline(always)]
    pub fn comp2_value(&self) -> COMP2_VALUE_R {
        COMP2_VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CSR register lock bit
    #[inline(always)]
    pub fn comp2_lock(&self) -> COMP2_LOCK_R {
        COMP2_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP2_CSR")
            .field("comp2_en", &self.comp2_en())
            .field("comp2_pwrmode", &self.comp2_pwrmode())
            .field("comp2_inmsel", &self.comp2_inmsel())
            .field("comp2_inpsel", &self.comp2_inpsel())
            .field("comp2_winmode", &self.comp2_winmode())
            .field("comp2_polarity", &self.comp2_polarity())
            .field("comp2_hyst", &self.comp2_hyst())
            .field("comp2_blanking", &self.comp2_blanking())
            .field("comp2_brgen", &self.comp2_brgen())
            .field("comp2_scalen", &self.comp2_scalen())
            .field("comp2_inmesel", &self.comp2_inmesel())
            .field("comp2_value", &self.comp2_value())
            .field("comp2_lock", &self.comp2_lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator 2 enable bit
    #[inline(always)]
    pub fn comp2_en(&mut self) -> COMP2_EN_W<'_, COMP2_CSRrs> {
        COMP2_EN_W::new(self, 0)
    }
    ///Bits 2:3 - Power Mode of the comparator 2
    #[inline(always)]
    pub fn comp2_pwrmode(&mut self) -> COMP2_PWRMODE_W<'_, COMP2_CSRrs> {
        COMP2_PWRMODE_W::new(self, 2)
    }
    ///Bits 4:5 - Comparator 2 input minus selection bits
    #[inline(always)]
    pub fn comp2_inmsel(&mut self) -> COMP2_INMSEL_W<'_, COMP2_CSRrs> {
        COMP2_INMSEL_W::new(self, 4)
    }
    ///Bits 7:8 - Comparator 1 input plus selection bit
    #[inline(always)]
    pub fn comp2_inpsel(&mut self) -> COMP2_INPSEL_W<'_, COMP2_CSRrs> {
        COMP2_INPSEL_W::new(self, 7)
    }
    ///Bit 9 - Windows mode selection bit
    #[inline(always)]
    pub fn comp2_winmode(&mut self) -> COMP2_WINMODE_W<'_, COMP2_CSRrs> {
        COMP2_WINMODE_W::new(self, 9)
    }
    ///Bit 15 - Comparator 2 polarity selection bit
    #[inline(always)]
    pub fn comp2_polarity(&mut self) -> COMP2_POLARITY_W<'_, COMP2_CSRrs> {
        COMP2_POLARITY_W::new(self, 15)
    }
    ///Bits 16:17 - Comparator 2 hysteresis selection bits
    #[inline(always)]
    pub fn comp2_hyst(&mut self) -> COMP2_HYST_W<'_, COMP2_CSRrs> {
        COMP2_HYST_W::new(self, 16)
    }
    ///Bits 18:20 - Comparator 2 blanking source selection bits
    #[inline(always)]
    pub fn comp2_blanking(&mut self) -> COMP2_BLANKING_W<'_, COMP2_CSRrs> {
        COMP2_BLANKING_W::new(self, 18)
    }
    ///Bit 22 - Scaler bridge enable
    #[inline(always)]
    pub fn comp2_brgen(&mut self) -> COMP2_BRGEN_W<'_, COMP2_CSRrs> {
        COMP2_BRGEN_W::new(self, 22)
    }
    ///Bit 23 - Voltage scaler enable bit
    #[inline(always)]
    pub fn comp2_scalen(&mut self) -> COMP2_SCALEN_W<'_, COMP2_CSRrs> {
        COMP2_SCALEN_W::new(self, 23)
    }
    ///Bits 25:26 - comparator 2 input minus extended selection bits.
    #[inline(always)]
    pub fn comp2_inmesel(&mut self) -> COMP2_INMESEL_W<'_, COMP2_CSRrs> {
        COMP2_INMESEL_W::new(self, 25)
    }
    ///Bit 31 - CSR register lock bit
    #[inline(always)]
    pub fn comp2_lock(&mut self) -> COMP2_LOCK_W<'_, COMP2_CSRrs> {
        COMP2_LOCK_W::new(self, 31)
    }
}
/**Comparator 2 control and status register

You can [`read`](crate::Reg::read) this register and get [`comp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#COMP:COMP2_CSR)*/
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
