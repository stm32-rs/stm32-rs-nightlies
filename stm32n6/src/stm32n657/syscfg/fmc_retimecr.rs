///Register `FMC_RETIMECR` reader
pub type R = crate::R<FMC_RETIMECRrs>;
///Register `FMC_RETIMECR` writer
pub type W = crate::W<FMC_RETIMECRrs>;
///Field `CFG_RETIME_RX` reader - Retiming on Rx path
pub type CFG_RETIME_RX_R = crate::BitReader;
///Field `CFG_RETIME_RX` writer - Retiming on Rx path
pub type CFG_RETIME_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFG_RETIME_TX` reader - Retiming on Tx path
pub type CFG_RETIME_TX_R = crate::BitReader;
///Field `CFG_RETIME_TX` writer - Retiming on Tx path
pub type CFG_RETIME_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDFBCLK_180` reader - Delay on feedback clock
pub type SDFBCLK_180_R = crate::BitReader;
///Field `SDFBCLK_180` writer - Delay on feedback clock
pub type SDFBCLK_180_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Retiming on Rx path
    #[inline(always)]
    pub fn cfg_retime_rx(&self) -> CFG_RETIME_RX_R {
        CFG_RETIME_RX_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Retiming on Tx path
    #[inline(always)]
    pub fn cfg_retime_tx(&self) -> CFG_RETIME_TX_R {
        CFG_RETIME_TX_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Delay on feedback clock
    #[inline(always)]
    pub fn sdfbclk_180(&self) -> SDFBCLK_180_R {
        SDFBCLK_180_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMC_RETIMECR")
            .field("cfg_retime_rx", &self.cfg_retime_rx())
            .field("cfg_retime_tx", &self.cfg_retime_tx())
            .field("sdfbclk_180", &self.sdfbclk_180())
            .finish()
    }
}
impl W {
    ///Bit 0 - Retiming on Rx path
    #[inline(always)]
    pub fn cfg_retime_rx(&mut self) -> CFG_RETIME_RX_W<'_, FMC_RETIMECRrs> {
        CFG_RETIME_RX_W::new(self, 0)
    }
    ///Bit 1 - Retiming on Tx path
    #[inline(always)]
    pub fn cfg_retime_tx(&mut self) -> CFG_RETIME_TX_W<'_, FMC_RETIMECRrs> {
        CFG_RETIME_TX_W::new(self, 1)
    }
    ///Bit 2 - Delay on feedback clock
    #[inline(always)]
    pub fn sdfbclk_180(&mut self) -> SDFBCLK_180_W<'_, FMC_RETIMECRrs> {
        SDFBCLK_180_W::new(self, 2)
    }
}
/**SYSCFG FMC retiming logic control register

You can [`read`](crate::Reg::read) this register and get [`fmc_retimecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc_retimecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SYSCFG:FMC_RETIMECR)*/
pub struct FMC_RETIMECRrs;
impl crate::RegisterSpec for FMC_RETIMECRrs {
    type Ux = u32;
}
///`read()` method returns [`fmc_retimecr::R`](R) reader structure
impl crate::Readable for FMC_RETIMECRrs {}
///`write(|w| ..)` method takes [`fmc_retimecr::W`](W) writer structure
impl crate::Writable for FMC_RETIMECRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FMC_RETIMECR to value 0
impl crate::Resettable for FMC_RETIMECRrs {}
