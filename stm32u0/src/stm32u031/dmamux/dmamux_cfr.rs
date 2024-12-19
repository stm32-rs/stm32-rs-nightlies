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
///Field `CSOF5` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
pub type CSOF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF6` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
pub type CSOF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF7` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
pub type CSOF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF8` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
pub type CSOF8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF9` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
pub type CSOF9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF10` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
pub type CSOF10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF11` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
pub type CSOF11_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DMAMUX_CFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof0(&mut self) -> CSOF0_W<DMAMUX_CFRrs> {
        CSOF0_W::new(self, 0)
    }
    ///Bit 1 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof1(&mut self) -> CSOF1_W<DMAMUX_CFRrs> {
        CSOF1_W::new(self, 1)
    }
    ///Bit 2 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof2(&mut self) -> CSOF2_W<DMAMUX_CFRrs> {
        CSOF2_W::new(self, 2)
    }
    ///Bit 3 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof3(&mut self) -> CSOF3_W<DMAMUX_CFRrs> {
        CSOF3_W::new(self, 3)
    }
    ///Bit 4 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof4(&mut self) -> CSOF4_W<DMAMUX_CFRrs> {
        CSOF4_W::new(self, 4)
    }
    ///Bit 5 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof5(&mut self) -> CSOF5_W<DMAMUX_CFRrs> {
        CSOF5_W::new(self, 5)
    }
    ///Bit 6 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof6(&mut self) -> CSOF6_W<DMAMUX_CFRrs> {
        CSOF6_W::new(self, 6)
    }
    ///Bit 7 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof7(&mut self) -> CSOF7_W<DMAMUX_CFRrs> {
        CSOF7_W::new(self, 7)
    }
    ///Bit 8 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof8(&mut self) -> CSOF8_W<DMAMUX_CFRrs> {
        CSOF8_W::new(self, 8)
    }
    ///Bit 9 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof9(&mut self) -> CSOF9_W<DMAMUX_CFRrs> {
        CSOF9_W::new(self, 9)
    }
    ///Bit 10 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof10(&mut self) -> CSOF10_W<DMAMUX_CFRrs> {
        CSOF10_W::new(self, 10)
    }
    ///Bit 11 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
    #[inline(always)]
    pub fn csof11(&mut self) -> CSOF11_W<DMAMUX_CFRrs> {
        CSOF11_W::new(self, 11)
    }
}
/**DMAMUX request line multiplexer interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:DMAMUX_CFR)*/
pub struct DMAMUX_CFRrs;
impl crate::RegisterSpec for DMAMUX_CFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dmamux_cfr::W`](W) writer structure
impl crate::Writable for DMAMUX_CFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMAMUX_CFR to value 0
impl crate::Resettable for DMAMUX_CFRrs {
    const RESET_VALUE: u32 = 0;
}
