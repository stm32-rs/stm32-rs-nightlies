///Register `CFR` writer
pub type W = crate::W<CFRrs>;
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
///Field `CSOF5` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
pub type CSOF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF6` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
pub type CSOF6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof0(&mut self) -> CSOF0_W<CFRrs> {
        CSOF0_W::new(self, 0)
    }
    ///Bit 1 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof1(&mut self) -> CSOF1_W<CFRrs> {
        CSOF1_W::new(self, 1)
    }
    ///Bit 2 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof2(&mut self) -> CSOF2_W<CFRrs> {
        CSOF2_W::new(self, 2)
    }
    ///Bit 3 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof3(&mut self) -> CSOF3_W<CFRrs> {
        CSOF3_W::new(self, 3)
    }
    ///Bit 4 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof4(&mut self) -> CSOF4_W<CFRrs> {
        CSOF4_W::new(self, 4)
    }
    ///Bit 5 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof5(&mut self) -> CSOF5_W<CFRrs> {
        CSOF5_W::new(self, 5)
    }
    ///Bit 6 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof6(&mut self) -> CSOF6_W<CFRrs> {
        CSOF6_W::new(self, 6)
    }
}
/**DMAMUX request line multiplexer interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#DMAMUX:CFR)*/
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cfr::W`](W) writer structure
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFR to value 0
impl crate::Resettable for CFRrs {
    const RESET_VALUE: u32 = 0;
}
