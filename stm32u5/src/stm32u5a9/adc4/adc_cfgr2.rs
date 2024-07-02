///Register `ADC_CFGR2` reader
pub type R = crate::R<ADC_CFGR2rs>;
///Register `ADC_CFGR2` writer
pub type W = crate::W<ADC_CFGR2rs>;
///Field `OVSE` reader - OVSE
pub type OVSE_R = crate::BitReader;
///Field `OVSE` writer - OVSE
pub type OVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVSR` reader - OVSR
pub type OVSR_R = crate::FieldReader;
///Field `OVSR` writer - OVSR
pub type OVSR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OVSS` reader - OVSS
pub type OVSS_R = crate::FieldReader;
///Field `OVSS` writer - OVSS
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TOVS` reader - TOVS
pub type TOVS_R = crate::BitReader;
///Field `TOVS` writer - TOVS
pub type TOVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFTRIG` reader - LFTRIG
pub type LFTRIG_R = crate::BitReader;
///Field `LFTRIG` writer - LFTRIG
pub type LFTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - OVSE
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:4 - OVSR
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:8 - OVSS
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - TOVS
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 29 - LFTRIG
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CFGR2")
            .field("lftrig", &self.lftrig())
            .field("tovs", &self.tovs())
            .field("ovss", &self.ovss())
            .field("ovsr", &self.ovsr())
            .field("ovse", &self.ovse())
            .finish()
    }
}
impl W {
    ///Bit 0 - OVSE
    #[inline(always)]
    #[must_use]
    pub fn ovse(&mut self) -> OVSE_W<ADC_CFGR2rs> {
        OVSE_W::new(self, 0)
    }
    ///Bits 2:4 - OVSR
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<ADC_CFGR2rs> {
        OVSR_W::new(self, 2)
    }
    ///Bits 5:8 - OVSS
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<ADC_CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    ///Bit 9 - TOVS
    #[inline(always)]
    #[must_use]
    pub fn tovs(&mut self) -> TOVS_W<ADC_CFGR2rs> {
        TOVS_W::new(self, 9)
    }
    ///Bit 29 - LFTRIG
    #[inline(always)]
    #[must_use]
    pub fn lftrig(&mut self) -> LFTRIG_W<ADC_CFGR2rs> {
        LFTRIG_W::new(self, 29)
    }
}
/**ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`adc_cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#ADC4:ADC_CFGR2)*/
pub struct ADC_CFGR2rs;
impl crate::RegisterSpec for ADC_CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`adc_cfgr2::R`](R) reader structure
impl crate::Readable for ADC_CFGR2rs {}
///`write(|w| ..)` method takes [`adc_cfgr2::W`](W) writer structure
impl crate::Writable for ADC_CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_CFGR2 to value 0
impl crate::Resettable for ADC_CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
