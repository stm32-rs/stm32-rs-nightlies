///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `PRESC` reader - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL1=10, ADSTART1=10, ADSTP1=10, ADDIS1=10 and ADEN1=10).
pub type PRESC_R = crate::FieldReader;
///Field `PRESC` writer - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL1=10, ADSTART1=10, ADSTP1=10, ADDIS1=10 and ADEN1=10).
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `VREFEN` reader - V<sub>REFINT</sub> enable This bit is set and cleared by software to enable/disable the V<sub>REFINT</sub>. Note: Software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type VREFEN_R = crate::BitReader;
///Field `VREFEN` writer - V<sub>REFINT</sub> enable This bit is set and cleared by software to enable/disable the V<sub>REFINT</sub>. Note: Software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEN` reader - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type TSEN_R = crate::BitReader;
///Field `TSEN` writer - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBATEN` reader - V<sub>BAT</sub> enable This bit is set and cleared by software to enable/disable the V<sub>BAT</sub> channel. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing)
pub type VBATEN_R = crate::BitReader;
///Field `VBATEN` writer - V<sub>BAT</sub> enable This bit is set and cleared by software to enable/disable the V<sub>BAT</sub> channel. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing)
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 18:21 - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL1=10, ADSTART1=10, ADSTP1=10, ADDIS1=10 and ADEN1=10).
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 22 - V<sub>REFINT</sub> enable This bit is set and cleared by software to enable/disable the V<sub>REFINT</sub>. Note: Software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - V<sub>BAT</sub> enable This bit is set and cleared by software to enable/disable the V<sub>BAT</sub> channel. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing)
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("presc", &self.presc())
            .field("vrefen", &self.vrefen())
            .field("tsen", &self.tsen())
            .field("vbaten", &self.vbaten())
            .finish()
    }
}
impl W {
    ///Bits 18:21 - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL1=10, ADSTART1=10, ADSTP1=10, ADDIS1=10 and ADEN1=10).
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<'_, CCRrs> {
        PRESC_W::new(self, 18)
    }
    ///Bit 22 - V<sub>REFINT</sub> enable This bit is set and cleared by software to enable/disable the V<sub>REFINT</sub>. Note: Software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<'_, CCRrs> {
        VREFEN_W::new(self, 22)
    }
    ///Bit 23 - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W<'_, CCRrs> {
        TSEN_W::new(self, 23)
    }
    ///Bit 24 - V<sub>BAT</sub> enable This bit is set and cleared by software to enable/disable the V<sub>BAT</sub> channel. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing)
    #[inline(always)]
    pub fn vbaten(&mut self) -> VBATEN_W<'_, CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
/**ADC common configuration register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#ADC:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
