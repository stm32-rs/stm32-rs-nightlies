///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
///Field `OVSE` reader - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).
pub type OVSE_R = crate::BitReader;
///Field `OVSE` writer - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).
pub type OVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVSR` reader - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).
pub type OVSR_R = crate::FieldReader;
///Field `OVSR` writer - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).
pub type OVSR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OVSS` reader - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1(which ensures that no conversion is ongoing).
pub type OVSS_R = crate::FieldReader;
///Field `OVSS` writer - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1(which ensures that no conversion is ongoing).
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TOVS` reader - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).
pub type TOVS_R = crate::BitReader;
///Field `TOVS` writer - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).
pub type TOVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFTRIG` reader - Low frequency trigger mode enable This bit must be set by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).
pub type LFTRIG_R = crate::BitReader;
///Field `LFTRIG` writer - Low frequency trigger mode enable This bit must be set by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).
pub type LFTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:4 - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:8 - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1(which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 29 - Low frequency trigger mode enable This bit must be set by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("ovse", &self.ovse())
            .field("ovsr", &self.ovsr())
            .field("ovss", &self.ovss())
            .field("tovs", &self.tovs())
            .field("lftrig", &self.lftrig())
            .finish()
    }
}
impl W {
    ///Bit 0 - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovse(&mut self) -> OVSE_W<'_, CFGR2rs> {
        OVSE_W::new(self, 0)
    }
    ///Bits 2:4 - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovsr(&mut self) -> OVSR_W<'_, CFGR2rs> {
        OVSR_W::new(self, 2)
    }
    ///Bits 5:8 - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1(which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W<'_, CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    ///Bit 9 - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn tovs(&mut self) -> TOVS_W<'_, CFGR2rs> {
        TOVS_W::new(self, 9)
    }
    ///Bit 29 - Low frequency trigger mode enable This bit must be set by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing ADSTP to 1 (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn lftrig(&mut self) -> LFTRIG_W<'_, CFGR2rs> {
        LFTRIG_W::new(self, 29)
    }
}
/**ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#ADC4:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
