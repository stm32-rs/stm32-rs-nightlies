///Register `COMP1_CSR` reader
pub type R = crate::R<COMP1_CSRrs>;
///Register `COMP1_CSR` writer
pub type W = crate::W<COMP1_CSRrs>;
///Field `COMP1_EN` reader - Comparator enable
pub type COMP1_EN_R = crate::BitReader;
///Field `COMP1_EN` writer - Comparator enable
pub type COMP1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP1_PWRMODE` reader - Comparator power mode
pub type COMP1_PWRMODE_R = crate::FieldReader;
///Field `COMP1_PWRMODE` writer - Comparator power mode
pub type COMP1_PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP1_INMSEL` reader - Comparator input minus selection
pub type COMP1_INMSEL_R = crate::FieldReader;
///Field `COMP1_INMSEL` writer - Comparator input minus selection
pub type COMP1_INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMP1_INPSEL` reader - Comparator input plus selection
pub type COMP1_INPSEL_R = crate::FieldReader;
///Field `COMP1_INPSEL` writer - Comparator input plus selection
pub type COMP1_INPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP1_POLARITY` reader - Comparator output polarity
pub type COMP1_POLARITY_R = crate::BitReader;
///Field `COMP1_POLARITY` writer - Comparator output polarity
pub type COMP1_POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP1_HYST` reader - Comparator hysteresis
pub type COMP1_HYST_R = crate::FieldReader;
///Field `COMP1_HYST` writer - Comparator hysteresis
pub type COMP1_HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP1_BLANKING` reader - Comparator blanking source
pub type COMP1_BLANKING_R = crate::FieldReader;
///Field `COMP1_BLANKING` writer - Comparator blanking source
pub type COMP1_BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMP1_BRGEN` reader - Comparator voltage scaler enable
pub type COMP1_BRGEN_R = crate::BitReader;
///Field `COMP1_BRGEN` writer - Comparator voltage scaler enable
pub type COMP1_BRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP1_SCALEN` reader - Comparator scaler bridge enable
pub type COMP1_SCALEN_R = crate::BitReader;
///Field `COMP1_SCALEN` writer - Comparator scaler bridge enable
pub type COMP1_SCALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP1_INMESEL` reader - Comparator input minus extended selection
pub type COMP1_INMESEL_R = crate::FieldReader;
///Field `COMP1_INMESEL` writer - Comparator input minus extended selection
pub type COMP1_INMESEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP1_VALUE` reader - Comparator output level
pub type COMP1_VALUE_R = crate::BitReader;
///Field `COMP1_LOCK` reader - Comparator lock
pub type COMP1_LOCK_R = crate::BitReader;
///Field `COMP1_LOCK` writer - Comparator lock
pub type COMP1_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Comparator enable
    #[inline(always)]
    pub fn comp1_en(&self) -> COMP1_EN_R {
        COMP1_EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Comparator power mode
    #[inline(always)]
    pub fn comp1_pwrmode(&self) -> COMP1_PWRMODE_R {
        COMP1_PWRMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Comparator input minus selection
    #[inline(always)]
    pub fn comp1_inmsel(&self) -> COMP1_INMSEL_R {
        COMP1_INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:8 - Comparator input plus selection
    #[inline(always)]
    pub fn comp1_inpsel(&self) -> COMP1_INPSEL_R {
        COMP1_INPSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 15 - Comparator output polarity
    #[inline(always)]
    pub fn comp1_polarity(&self) -> COMP1_POLARITY_R {
        COMP1_POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator hysteresis
    #[inline(always)]
    pub fn comp1_hyst(&self) -> COMP1_HYST_R {
        COMP1_HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - Comparator blanking source
    #[inline(always)]
    pub fn comp1_blanking(&self) -> COMP1_BLANKING_R {
        COMP1_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 22 - Comparator voltage scaler enable
    #[inline(always)]
    pub fn comp1_brgen(&self) -> COMP1_BRGEN_R {
        COMP1_BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Comparator scaler bridge enable
    #[inline(always)]
    pub fn comp1_scalen(&self) -> COMP1_SCALEN_R {
        COMP1_SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 25:26 - Comparator input minus extended selection
    #[inline(always)]
    pub fn comp1_inmesel(&self) -> COMP1_INMESEL_R {
        COMP1_INMESEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 30 - Comparator output level
    #[inline(always)]
    pub fn comp1_value(&self) -> COMP1_VALUE_R {
        COMP1_VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Comparator lock
    #[inline(always)]
    pub fn comp1_lock(&self) -> COMP1_LOCK_R {
        COMP1_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP1_CSR")
            .field("comp1_en", &self.comp1_en())
            .field("comp1_pwrmode", &self.comp1_pwrmode())
            .field("comp1_inmsel", &self.comp1_inmsel())
            .field("comp1_inpsel", &self.comp1_inpsel())
            .field("comp1_polarity", &self.comp1_polarity())
            .field("comp1_hyst", &self.comp1_hyst())
            .field("comp1_blanking", &self.comp1_blanking())
            .field("comp1_brgen", &self.comp1_brgen())
            .field("comp1_scalen", &self.comp1_scalen())
            .field("comp1_inmesel", &self.comp1_inmesel())
            .field("comp1_value", &self.comp1_value())
            .field("comp1_lock", &self.comp1_lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator enable
    #[inline(always)]
    pub fn comp1_en(&mut self) -> COMP1_EN_W<'_, COMP1_CSRrs> {
        COMP1_EN_W::new(self, 0)
    }
    ///Bits 2:3 - Comparator power mode
    #[inline(always)]
    pub fn comp1_pwrmode(&mut self) -> COMP1_PWRMODE_W<'_, COMP1_CSRrs> {
        COMP1_PWRMODE_W::new(self, 2)
    }
    ///Bits 4:6 - Comparator input minus selection
    #[inline(always)]
    pub fn comp1_inmsel(&mut self) -> COMP1_INMSEL_W<'_, COMP1_CSRrs> {
        COMP1_INMSEL_W::new(self, 4)
    }
    ///Bits 7:8 - Comparator input plus selection
    #[inline(always)]
    pub fn comp1_inpsel(&mut self) -> COMP1_INPSEL_W<'_, COMP1_CSRrs> {
        COMP1_INPSEL_W::new(self, 7)
    }
    ///Bit 15 - Comparator output polarity
    #[inline(always)]
    pub fn comp1_polarity(&mut self) -> COMP1_POLARITY_W<'_, COMP1_CSRrs> {
        COMP1_POLARITY_W::new(self, 15)
    }
    ///Bits 16:17 - Comparator hysteresis
    #[inline(always)]
    pub fn comp1_hyst(&mut self) -> COMP1_HYST_W<'_, COMP1_CSRrs> {
        COMP1_HYST_W::new(self, 16)
    }
    ///Bits 18:20 - Comparator blanking source
    #[inline(always)]
    pub fn comp1_blanking(&mut self) -> COMP1_BLANKING_W<'_, COMP1_CSRrs> {
        COMP1_BLANKING_W::new(self, 18)
    }
    ///Bit 22 - Comparator voltage scaler enable
    #[inline(always)]
    pub fn comp1_brgen(&mut self) -> COMP1_BRGEN_W<'_, COMP1_CSRrs> {
        COMP1_BRGEN_W::new(self, 22)
    }
    ///Bit 23 - Comparator scaler bridge enable
    #[inline(always)]
    pub fn comp1_scalen(&mut self) -> COMP1_SCALEN_W<'_, COMP1_CSRrs> {
        COMP1_SCALEN_W::new(self, 23)
    }
    ///Bits 25:26 - Comparator input minus extended selection
    #[inline(always)]
    pub fn comp1_inmesel(&mut self) -> COMP1_INMESEL_W<'_, COMP1_CSRrs> {
        COMP1_INMESEL_W::new(self, 25)
    }
    ///Bit 31 - Comparator lock
    #[inline(always)]
    pub fn comp1_lock(&mut self) -> COMP1_LOCK_W<'_, COMP1_CSRrs> {
        COMP1_LOCK_W::new(self, 31)
    }
}
/**Comparator control and status register

You can [`read`](crate::Reg::read) this register and get [`comp1_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#COMP:COMP1_CSR)*/
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
