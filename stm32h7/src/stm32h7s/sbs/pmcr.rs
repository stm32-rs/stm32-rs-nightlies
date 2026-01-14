///Register `PMCR` reader
pub type R = crate::R<PMCRrs>;
///Register `PMCR` writer
pub type W = crate::W<PMCRrs>;
///Field `FMPLUS_PB6` reader - Fast-mode Plus on PB(6)
pub type FMPLUS_PB6_R = crate::BitReader;
///Field `FMPLUS_PB6` writer - Fast-mode Plus on PB(6)
pub type FMPLUS_PB6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMPLUS_PB7` reader - Fast-mode Plus on PB(7)
pub type FMPLUS_PB7_R = crate::BitReader;
///Field `FMPLUS_PB7` writer - Fast-mode Plus on PB(7)
pub type FMPLUS_PB7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMPLUS_PB8` reader - Fast-mode Plus on PB(8)
pub type FMPLUS_PB8_R = crate::BitReader;
///Field `FMPLUS_PB8` writer - Fast-mode Plus on PB(8)
pub type FMPLUS_PB8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMPLUS_PB9` reader - Fast-mode Plus on PB(9)
pub type FMPLUS_PB9_R = crate::BitReader;
///Field `FMPLUS_PB9` writer - Fast-mode Plus on PB(9)
pub type FMPLUS_PB9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOSTEN` reader - booster enable Set this bit to reduce the THD of the analog switches when the supply voltage is below 2.7 V. guaranteeing the same performance as with the full voltage range. To avoid current consumption due to booster activation when V<sub>DDA</sub> < 2.7 V and V<sub>DD</sub> > 2.7 V, V<sub>DD</sub> can be selected as supply voltage for analog switches by setting BOOSTVDDSEL bit in SBS_PMCR. In this case, the BOOSTEN bit must be cleared to avoid unwanted power consumption.
pub type BOOSTEN_R = crate::BitReader;
///Field `BOOSTEN` writer - booster enable Set this bit to reduce the THD of the analog switches when the supply voltage is below 2.7 V. guaranteeing the same performance as with the full voltage range. To avoid current consumption due to booster activation when V<sub>DDA</sub> < 2.7 V and V<sub>DD</sub> > 2.7 V, V<sub>DD</sub> can be selected as supply voltage for analog switches by setting BOOSTVDDSEL bit in SBS_PMCR. In this case, the BOOSTEN bit must be cleared to avoid unwanted power consumption.
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOSTVDDSEL` reader - booster V<sub>DD</sub> selection This bit selects the analog switch supply voltage, between V<sub>DD</sub>, V<sub>DDA</sub> and booster. To avoid current consumption due to booster activation when V<sub>DDA</sub> < 2.7 V and V<sub>DD</sub> > 2.7 V, V<sub>DD</sub> can be selected as supply voltage for analog switches. In this case, the BOOSTEN bit must be cleared to avoid unwanted power consumption. When both V<sub>DD and </sub>V<sub>DDA</sub> are below 2.7 V, the booster is still needed to obtain full AC performances from the I/O analog switches.
pub type BOOSTVDDSEL_R = crate::BitReader;
///Field `BOOSTVDDSEL` writer - booster V<sub>DD</sub> selection This bit selects the analog switch supply voltage, between V<sub>DD</sub>, V<sub>DDA</sub> and booster. To avoid current consumption due to booster activation when V<sub>DDA</sub> < 2.7 V and V<sub>DD</sub> > 2.7 V, V<sub>DD</sub> can be selected as supply voltage for analog switches. In this case, the BOOSTEN bit must be cleared to avoid unwanted power consumption. When both V<sub>DD and </sub>V<sub>DDA</sub> are below 2.7 V, the booster is still needed to obtain full AC performances from the I/O analog switches.
pub type BOOSTVDDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH_PHYSEL` reader - Ethernet PHY interface selection Other: reserved
pub type ETH_PHYSEL_R = crate::FieldReader;
///Field `ETH_PHYSEL` writer - Ethernet PHY interface selection Other: reserved
pub type ETH_PHYSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AXIRAM_WS` reader - AXIRAM wait state Set this bit to add one wait state to all AXIRAMs when ECC = 0. When ECC = 1 there is one wait state by default.
pub type AXIRAM_WS_R = crate::BitReader;
///Field `AXIRAM_WS` writer - AXIRAM wait state Set this bit to add one wait state to all AXIRAMs when ECC = 0. When ECC = 1 there is one wait state by default.
pub type AXIRAM_WS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - Fast-mode Plus on PB(6)
    #[inline(always)]
    pub fn fmplus_pb6(&self) -> FMPLUS_PB6_R {
        FMPLUS_PB6_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Fast-mode Plus on PB(7)
    #[inline(always)]
    pub fn fmplus_pb7(&self) -> FMPLUS_PB7_R {
        FMPLUS_PB7_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Fast-mode Plus on PB(8)
    #[inline(always)]
    pub fn fmplus_pb8(&self) -> FMPLUS_PB8_R {
        FMPLUS_PB8_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Fast-mode Plus on PB(9)
    #[inline(always)]
    pub fn fmplus_pb9(&self) -> FMPLUS_PB9_R {
        FMPLUS_PB9_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - booster enable Set this bit to reduce the THD of the analog switches when the supply voltage is below 2.7 V. guaranteeing the same performance as with the full voltage range. To avoid current consumption due to booster activation when V<sub>DDA</sub> < 2.7 V and V<sub>DD</sub> > 2.7 V, V<sub>DD</sub> can be selected as supply voltage for analog switches by setting BOOSTVDDSEL bit in SBS_PMCR. In this case, the BOOSTEN bit must be cleared to avoid unwanted power consumption.
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - booster V<sub>DD</sub> selection This bit selects the analog switch supply voltage, between V<sub>DD</sub>, V<sub>DDA</sub> and booster. To avoid current consumption due to booster activation when V<sub>DDA</sub> < 2.7 V and V<sub>DD</sub> > 2.7 V, V<sub>DD</sub> can be selected as supply voltage for analog switches. In this case, the BOOSTEN bit must be cleared to avoid unwanted power consumption. When both V<sub>DD and </sub>V<sub>DDA</sub> are below 2.7 V, the booster is still needed to obtain full AC performances from the I/O analog switches.
    #[inline(always)]
    pub fn boostvddsel(&self) -> BOOSTVDDSEL_R {
        BOOSTVDDSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 21:23 - Ethernet PHY interface selection Other: reserved
    #[inline(always)]
    pub fn eth_physel(&self) -> ETH_PHYSEL_R {
        ETH_PHYSEL_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bit 28 - AXIRAM wait state Set this bit to add one wait state to all AXIRAMs when ECC = 0. When ECC = 1 there is one wait state by default.
    #[inline(always)]
    pub fn axiram_ws(&self) -> AXIRAM_WS_R {
        AXIRAM_WS_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMCR")
            .field("fmplus_pb6", &self.fmplus_pb6())
            .field("fmplus_pb7", &self.fmplus_pb7())
            .field("fmplus_pb8", &self.fmplus_pb8())
            .field("fmplus_pb9", &self.fmplus_pb9())
            .field("boosten", &self.boosten())
            .field("boostvddsel", &self.boostvddsel())
            .field("eth_physel", &self.eth_physel())
            .field("axiram_ws", &self.axiram_ws())
            .finish()
    }
}
impl W {
    ///Bit 4 - Fast-mode Plus on PB(6)
    #[inline(always)]
    pub fn fmplus_pb6(&mut self) -> FMPLUS_PB6_W<'_, PMCRrs> {
        FMPLUS_PB6_W::new(self, 4)
    }
    ///Bit 5 - Fast-mode Plus on PB(7)
    #[inline(always)]
    pub fn fmplus_pb7(&mut self) -> FMPLUS_PB7_W<'_, PMCRrs> {
        FMPLUS_PB7_W::new(self, 5)
    }
    ///Bit 6 - Fast-mode Plus on PB(8)
    #[inline(always)]
    pub fn fmplus_pb8(&mut self) -> FMPLUS_PB8_W<'_, PMCRrs> {
        FMPLUS_PB8_W::new(self, 6)
    }
    ///Bit 7 - Fast-mode Plus on PB(9)
    #[inline(always)]
    pub fn fmplus_pb9(&mut self) -> FMPLUS_PB9_W<'_, PMCRrs> {
        FMPLUS_PB9_W::new(self, 7)
    }
    ///Bit 8 - booster enable Set this bit to reduce the THD of the analog switches when the supply voltage is below 2.7 V. guaranteeing the same performance as with the full voltage range. To avoid current consumption due to booster activation when V<sub>DDA</sub> < 2.7 V and V<sub>DD</sub> > 2.7 V, V<sub>DD</sub> can be selected as supply voltage for analog switches by setting BOOSTVDDSEL bit in SBS_PMCR. In this case, the BOOSTEN bit must be cleared to avoid unwanted power consumption.
    #[inline(always)]
    pub fn boosten(&mut self) -> BOOSTEN_W<'_, PMCRrs> {
        BOOSTEN_W::new(self, 8)
    }
    ///Bit 9 - booster V<sub>DD</sub> selection This bit selects the analog switch supply voltage, between V<sub>DD</sub>, V<sub>DDA</sub> and booster. To avoid current consumption due to booster activation when V<sub>DDA</sub> < 2.7 V and V<sub>DD</sub> > 2.7 V, V<sub>DD</sub> can be selected as supply voltage for analog switches. In this case, the BOOSTEN bit must be cleared to avoid unwanted power consumption. When both V<sub>DD and </sub>V<sub>DDA</sub> are below 2.7 V, the booster is still needed to obtain full AC performances from the I/O analog switches.
    #[inline(always)]
    pub fn boostvddsel(&mut self) -> BOOSTVDDSEL_W<'_, PMCRrs> {
        BOOSTVDDSEL_W::new(self, 9)
    }
    ///Bits 21:23 - Ethernet PHY interface selection Other: reserved
    #[inline(always)]
    pub fn eth_physel(&mut self) -> ETH_PHYSEL_W<'_, PMCRrs> {
        ETH_PHYSEL_W::new(self, 21)
    }
    ///Bit 28 - AXIRAM wait state Set this bit to add one wait state to all AXIRAMs when ECC = 0. When ECC = 1 there is one wait state by default.
    #[inline(always)]
    pub fn axiram_ws(&mut self) -> AXIRAM_WS_W<'_, PMCRrs> {
        AXIRAM_WS_W::new(self, 28)
    }
}
/**SBS product mode and configuration register

You can [`read`](crate::Reg::read) this register and get [`pmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:PMCR)*/
pub struct PMCRrs;
impl crate::RegisterSpec for PMCRrs {
    type Ux = u32;
}
///`read()` method returns [`pmcr::R`](R) reader structure
impl crate::Readable for PMCRrs {}
///`write(|w| ..)` method takes [`pmcr::W`](W) writer structure
impl crate::Writable for PMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMCR to value 0
impl crate::Resettable for PMCRrs {}
