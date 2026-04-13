///Register `DMAMUX_CFR` writer
pub type W = crate::W<DMAMUX_CFRrs>;
///Field `CSOF0` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
pub type CSOF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF1` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
pub type CSOF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF2` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
pub type CSOF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF3` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
pub type CSOF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF4` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
pub type CSOF4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DMAMUX_CFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof0(&mut self) -> CSOF0_W<'_, DMAMUX_CFRrs> {
        CSOF0_W::new(self, 0)
    }
    ///Bit 1 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof1(&mut self) -> CSOF1_W<'_, DMAMUX_CFRrs> {
        CSOF1_W::new(self, 1)
    }
    ///Bit 2 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof2(&mut self) -> CSOF2_W<'_, DMAMUX_CFRrs> {
        CSOF2_W::new(self, 2)
    }
    ///Bit 3 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof3(&mut self) -> CSOF3_W<'_, DMAMUX_CFRrs> {
        CSOF3_W::new(self, 3)
    }
    ///Bit 4 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof4(&mut self) -> CSOF4_W<'_, DMAMUX_CFRrs> {
        CSOF4_W::new(self, 4)
    }
}
/**DMAMUX request line multiplexer interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMAMUX:DMAMUX_CFR)*/
pub struct DMAMUX_CFRrs;
impl crate::RegisterSpec for DMAMUX_CFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dmamux_cfr::W`](W) writer structure
impl crate::Writable for DMAMUX_CFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAMUX_CFR to value 0
impl crate::Resettable for DMAMUX_CFRrs {}
