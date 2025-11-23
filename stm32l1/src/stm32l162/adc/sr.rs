///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `AWD` reader - Analog watchdog flag
pub type AWD_R = crate::BitReader;
///Field `AWD` writer - Analog watchdog flag
pub type AWD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOC` reader - Regular channel end of conversion
pub type EOC_R = crate::BitReader;
///Field `EOC` writer - Regular channel end of conversion
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEOC` reader - Injected channel end of conversion
pub type JEOC_R = crate::BitReader;
///Field `JEOC` writer - Injected channel end of conversion
pub type JEOC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JSTRT` reader - Injected channel start flag
pub type JSTRT_R = crate::BitReader;
///Field `JSTRT` writer - Injected channel start flag
pub type JSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRT` reader - Regular channel start flag
pub type STRT_R = crate::BitReader;
///Field `STRT` writer - Regular channel start flag
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVR` reader - Overrun
pub type OVR_R = crate::BitReader;
///Field `OVR` writer - Overrun
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADONS` reader - ADC ON status
pub type ADONS_R = crate::BitReader;
///Field `RCNR` reader - Regular channel not ready
pub type RCNR_R = crate::BitReader;
///Field `JCNR` reader - Injected channel not ready
pub type JCNR_R = crate::BitReader;
impl R {
    ///Bit 0 - Analog watchdog flag
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Regular channel end of conversion
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected channel end of conversion
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Injected channel start flag
    #[inline(always)]
    pub fn jstrt(&self) -> JSTRT_R {
        JSTRT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Regular channel start flag
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Overrun
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ADC ON status
    #[inline(always)]
    pub fn adons(&self) -> ADONS_R {
        ADONS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Regular channel not ready
    #[inline(always)]
    pub fn rcnr(&self) -> RCNR_R {
        RCNR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Injected channel not ready
    #[inline(always)]
    pub fn jcnr(&self) -> JCNR_R {
        JCNR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("jcnr", &self.jcnr())
            .field("rcnr", &self.rcnr())
            .field("adons", &self.adons())
            .field("ovr", &self.ovr())
            .field("strt", &self.strt())
            .field("jstrt", &self.jstrt())
            .field("jeoc", &self.jeoc())
            .field("eoc", &self.eoc())
            .field("awd", &self.awd())
            .finish()
    }
}
impl W {
    ///Bit 0 - Analog watchdog flag
    #[inline(always)]
    pub fn awd(&mut self) -> AWD_W<'_, SRrs> {
        AWD_W::new(self, 0)
    }
    ///Bit 1 - Regular channel end of conversion
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<'_, SRrs> {
        EOC_W::new(self, 1)
    }
    ///Bit 2 - Injected channel end of conversion
    #[inline(always)]
    pub fn jeoc(&mut self) -> JEOC_W<'_, SRrs> {
        JEOC_W::new(self, 2)
    }
    ///Bit 3 - Injected channel start flag
    #[inline(always)]
    pub fn jstrt(&mut self) -> JSTRT_W<'_, SRrs> {
        JSTRT_W::new(self, 3)
    }
    ///Bit 4 - Regular channel start flag
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<'_, SRrs> {
        STRT_W::new(self, 4)
    }
    ///Bit 5 - Overrun
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<'_, SRrs> {
        OVR_W::new(self, 5)
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#ADC:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
