///Register `ADC_PWR` reader
pub type R = crate::R<ADC_PWRrs>;
///Register `ADC_PWR` writer
pub type W = crate::W<ADC_PWRrs>;
///Field `AUTOFF` reader - AUTOFF
pub type AUTOFF_R = crate::BitReader;
///Field `AUTOFF` writer - AUTOFF
pub type AUTOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPD` reader - DPD
pub type DPD_R = crate::BitReader;
///Field `DPD` writer - DPD
pub type DPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFPROT` reader - VREFPROT
pub type VREFPROT_R = crate::BitReader;
///Field `VREFPROT` writer - VREFPROT
pub type VREFPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFSECSMP` reader - VREFSECSMP
pub type VREFSECSMP_R = crate::BitReader;
///Field `VREFSECSMP` writer - VREFSECSMP
pub type VREFSECSMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AUTOFF
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DPD
    #[inline(always)]
    pub fn dpd(&self) -> DPD_R {
        DPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VREFPROT
    #[inline(always)]
    pub fn vrefprot(&self) -> VREFPROT_R {
        VREFPROT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VREFSECSMP
    #[inline(always)]
    pub fn vrefsecsmp(&self) -> VREFSECSMP_R {
        VREFSECSMP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_PWR")
            .field("vrefsecsmp", &self.vrefsecsmp())
            .field("vrefprot", &self.vrefprot())
            .field("dpd", &self.dpd())
            .field("autoff", &self.autoff())
            .finish()
    }
}
impl W {
    ///Bit 0 - AUTOFF
    #[inline(always)]
    #[must_use]
    pub fn autoff(&mut self) -> AUTOFF_W<ADC_PWRrs> {
        AUTOFF_W::new(self, 0)
    }
    ///Bit 1 - DPD
    #[inline(always)]
    #[must_use]
    pub fn dpd(&mut self) -> DPD_W<ADC_PWRrs> {
        DPD_W::new(self, 1)
    }
    ///Bit 2 - VREFPROT
    #[inline(always)]
    #[must_use]
    pub fn vrefprot(&mut self) -> VREFPROT_W<ADC_PWRrs> {
        VREFPROT_W::new(self, 2)
    }
    ///Bit 3 - VREFSECSMP
    #[inline(always)]
    #[must_use]
    pub fn vrefsecsmp(&mut self) -> VREFSECSMP_W<ADC_PWRrs> {
        VREFSECSMP_W::new(self, 3)
    }
}
/**ADC data register

You can [`read`](crate::Reg::read) this register and get [`adc_pwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_pwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#ADC4:ADC_PWR)*/
pub struct ADC_PWRrs;
impl crate::RegisterSpec for ADC_PWRrs {
    type Ux = u32;
}
///`read()` method returns [`adc_pwr::R`](R) reader structure
impl crate::Readable for ADC_PWRrs {}
///`write(|w| ..)` method takes [`adc_pwr::W`](W) writer structure
impl crate::Writable for ADC_PWRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_PWR to value 0
impl crate::Resettable for ADC_PWRrs {
    const RESET_VALUE: u32 = 0;
}
