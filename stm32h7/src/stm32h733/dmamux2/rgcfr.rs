///Register `RGCFR` writer
pub type W = crate::W<RGCFRrs>;
///Field `COF(0-7)` writer - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
pub type COF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<RGCFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `COF0` field.</div>
    #[inline(always)]
    pub fn cof(&mut self, n: u8) -> COF_W<RGCFRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        COF_W::new(self, n)
    }
    ///Bit 0 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    #[inline(always)]
    pub fn cof0(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 0)
    }
    ///Bit 1 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    #[inline(always)]
    pub fn cof1(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 1)
    }
    ///Bit 2 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    #[inline(always)]
    pub fn cof2(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 2)
    }
    ///Bit 3 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    #[inline(always)]
    pub fn cof3(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 3)
    }
    ///Bit 4 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    #[inline(always)]
    pub fn cof4(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 4)
    }
    ///Bit 5 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    #[inline(always)]
    pub fn cof5(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 5)
    }
    ///Bit 6 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    #[inline(always)]
    pub fn cof6(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 6)
    }
    ///Bit 7 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    #[inline(always)]
    pub fn cof7(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 7)
    }
}
/**DMAMux - DMA request generator clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#DMAMUX2:RGCFR)*/
pub struct RGCFRrs;
impl crate::RegisterSpec for RGCFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`rgcfr::W`](W) writer structure
impl crate::Writable for RGCFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RGCFR to value 0
impl crate::Resettable for RGCFRrs {
    const RESET_VALUE: u32 = 0;
}
