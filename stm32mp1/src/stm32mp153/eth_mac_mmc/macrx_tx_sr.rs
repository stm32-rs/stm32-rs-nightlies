///Register `MACRxTxSR` reader
pub type R = crate::R<MACRX_TX_SRrs>;
///Field `TJT` reader - TJT
pub type TJT_R = crate::BitReader;
///Field `NCARR` reader - NCARR
pub type NCARR_R = crate::BitReader;
///Field `LCARR` reader - LCARR
pub type LCARR_R = crate::BitReader;
///Field `EXDEF` reader - EXDEF
pub type EXDEF_R = crate::BitReader;
///Field `LCOL` reader - LCOL
pub type LCOL_R = crate::BitReader;
///Field `EXCOL` reader - EXCOL
pub type EXCOL_R = crate::BitReader;
///Field `RWT` reader - RWT
pub type RWT_R = crate::BitReader;
impl R {
    ///Bit 0 - TJT
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - NCARR
    #[inline(always)]
    pub fn ncarr(&self) -> NCARR_R {
        NCARR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LCARR
    #[inline(always)]
    pub fn lcarr(&self) -> LCARR_R {
        LCARR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EXDEF
    #[inline(always)]
    pub fn exdef(&self) -> EXDEF_R {
        EXDEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LCOL
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - EXCOL
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - RWT
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACRxTxSR")
            .field("tjt", &self.tjt())
            .field("ncarr", &self.ncarr())
            .field("lcarr", &self.lcarr())
            .field("exdef", &self.exdef())
            .field("lcol", &self.lcol())
            .field("excol", &self.excol())
            .field("rwt", &self.rwt())
            .finish()
    }
}
/**The Receive Transmit Status register contains the Receive and Transmit Error status.

You can [`read`](crate::Reg::read) this register and get [`macrx_tx_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACRxTxSR)*/
pub struct MACRX_TX_SRrs;
impl crate::RegisterSpec for MACRX_TX_SRrs {
    type Ux = u32;
}
///`read()` method returns [`macrx_tx_sr::R`](R) reader structure
impl crate::Readable for MACRX_TX_SRrs {}
///`reset()` method sets MACRxTxSR to value 0
impl crate::Resettable for MACRX_TX_SRrs {}
