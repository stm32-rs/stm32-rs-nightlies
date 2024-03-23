#[doc = "Register `RGCFR` writer"]
pub type W = crate::W<RGCFRrs>;
#[doc = "Field `COF(0-7)` writer - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
pub type COF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `COF0` field"]
    #[inline(always)]
    #[must_use]
    pub fn cof(&mut self, n: u8) -> COF_W<RGCFRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        COF_W::new(self, n)
    }
    #[doc = "Bit 0 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof0(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof1(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof2(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof3(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof4(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof5(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof6(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof7(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 7)
    }
}
#[doc = "DMAMux - DMA request generator clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgcfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGCFRrs;
impl crate::RegisterSpec for RGCFRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rgcfr::W`](W) writer structure"]
impl crate::Writable for RGCFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RGCFR to value 0"]
impl crate::Resettable for RGCFRrs {
    const RESET_VALUE: u32 = 0;
}
