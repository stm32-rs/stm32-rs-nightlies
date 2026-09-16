///Register `RX_CHAIN_ENG` reader
pub type R = crate::R<RX_CHAIN_ENGrs>;
///Register `RX_CHAIN_ENG` writer
pub type W = crate::W<RX_CHAIN_ENGrs>;
///Field `LNA_ISOL_ENA` reader - Option for LNA during the EN_RX state of the Radio FSM:
pub type LNA_ISOL_ENA_R = crate::BitReader;
///Field `LNA_ISOL_ENA` writer - Option for LNA during the EN_RX state of the Radio FSM:
pub type LNA_ISOL_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGA_PRECH_ENA` reader - Option for PGA precharge during the EN_RX state of the Radio FSM:
pub type PGA_PRECH_ENA_R = crate::BitReader;
///Field `PGA_PRECH_ENA` writer - Option for PGA precharge during the EN_RX state of the Radio FSM:
pub type PGA_PRECH_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Option for LNA during the EN_RX state of the Radio FSM:
    #[inline(always)]
    pub fn lna_isol_ena(&self) -> LNA_ISOL_ENA_R {
        LNA_ISOL_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Option for PGA precharge during the EN_RX state of the Radio FSM:
    #[inline(always)]
    pub fn pga_prech_ena(&self) -> PGA_PRECH_ENA_R {
        PGA_PRECH_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CHAIN_ENG")
            .field("lna_isol_ena", &self.lna_isol_ena())
            .field("pga_prech_ena", &self.pga_prech_ena())
            .finish()
    }
}
impl W {
    ///Bit 0 - Option for LNA during the EN_RX state of the Radio FSM:
    #[inline(always)]
    pub fn lna_isol_ena(&mut self) -> LNA_ISOL_ENA_W<'_, RX_CHAIN_ENGrs> {
        LNA_ISOL_ENA_W::new(self, 0)
    }
    ///Bit 1 - Option for PGA precharge during the EN_RX state of the Radio FSM:
    #[inline(always)]
    pub fn pga_prech_ena(&mut self) -> PGA_PRECH_ENA_W<'_, RX_CHAIN_ENGrs> {
        PGA_PRECH_ENA_W::new(self, 1)
    }
}
/**RX_CHAIN_ENG register

You can [`read`](crate::Reg::read) this register and get [`rx_chain_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_chain_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RX_CHAIN_ENG)*/
pub struct RX_CHAIN_ENGrs;
impl crate::RegisterSpec for RX_CHAIN_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`rx_chain_eng::R`](R) reader structure
impl crate::Readable for RX_CHAIN_ENGrs {}
///`write(|w| ..)` method takes [`rx_chain_eng::W`](W) writer structure
impl crate::Writable for RX_CHAIN_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RX_CHAIN_ENG to value 0x03
impl crate::Resettable for RX_CHAIN_ENGrs {
    const RESET_VALUE: u32 = 0x03;
}
