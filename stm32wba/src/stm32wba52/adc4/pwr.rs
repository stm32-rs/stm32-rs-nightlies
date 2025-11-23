///Register `PWR` reader
pub type R = crate::R<PWRrs>;
///Register `PWR` writer
pub type W = crate::W<PWRrs>;
///Field `AUTOFF` reader - Auto-off mode bit This bit is set and cleared by software. it is used to enable/disable the Auto-off mode. Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no conversion is ongoing).
pub type AUTOFF_R = crate::BitReader;
///Field `AUTOFF` writer - Auto-off mode bit This bit is set and cleared by software. it is used to enable/disable the Auto-off mode. Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no conversion is ongoing).
pub type AUTOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPD` reader - Deep-power-down mode bit This bit is set and cleared by software. It is used to enable/disable Deep-power-down mode in Autonomous mode when the ADC is not used. Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no conversion is ongoing). Note: Setting DPD in Auto-off mode automatically disables the LDO.
pub type DPD_R = crate::BitReader;
///Field `DPD` writer - Deep-power-down mode bit This bit is set and cleared by software. It is used to enable/disable Deep-power-down mode in Autonomous mode when the ADC is not used. Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no conversion is ongoing). Note: Setting DPD in Auto-off mode automatically disables the LDO.
pub type DPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Auto-off mode bit This bit is set and cleared by software. it is used to enable/disable the Auto-off mode. Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Deep-power-down mode bit This bit is set and cleared by software. It is used to enable/disable Deep-power-down mode in Autonomous mode when the ADC is not used. Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no conversion is ongoing). Note: Setting DPD in Auto-off mode automatically disables the LDO.
    #[inline(always)]
    pub fn dpd(&self) -> DPD_R {
        DPD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR")
            .field("autoff", &self.autoff())
            .field("dpd", &self.dpd())
            .finish()
    }
}
impl W {
    ///Bit 0 - Auto-off mode bit This bit is set and cleared by software. it is used to enable/disable the Auto-off mode. Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn autoff(&mut self) -> AUTOFF_W<'_, PWRrs> {
        AUTOFF_W::new(self, 0)
    }
    ///Bit 1 - Deep-power-down mode bit This bit is set and cleared by software. It is used to enable/disable Deep-power-down mode in Autonomous mode when the ADC is not used. Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no conversion is ongoing). Note: Setting DPD in Auto-off mode automatically disables the LDO.
    #[inline(always)]
    pub fn dpd(&mut self) -> DPD_W<'_, PWRrs> {
        DPD_W::new(self, 1)
    }
}
/**ADC data register

You can [`read`](crate::Reg::read) this register and get [`pwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#ADC4:PWR)*/
pub struct PWRrs;
impl crate::RegisterSpec for PWRrs {
    type Ux = u32;
}
///`read()` method returns [`pwr::R`](R) reader structure
impl crate::Readable for PWRrs {}
///`write(|w| ..)` method takes [`pwr::W`](W) writer structure
impl crate::Writable for PWRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWR to value 0
impl crate::Resettable for PWRrs {}
