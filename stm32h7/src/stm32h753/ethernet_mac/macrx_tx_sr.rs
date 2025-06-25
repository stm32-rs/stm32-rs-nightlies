///Register `MACRxTxSR` reader
pub type R = crate::R<MACRX_TX_SRrs>;
///Field `TJT` reader - Transmit Jabber Timeout
pub type TJT_R = crate::BitReader;
///Field `NCARR` reader - No Carrier
pub type NCARR_R = crate::BitReader;
///Field `LCARR` reader - Loss of Carrier
pub type LCARR_R = crate::BitReader;
///Field `EXDEF` reader - Excessive Deferral
pub type EXDEF_R = crate::BitReader;
///Field `LCOL` reader - Late Collision
pub type LCOL_R = crate::BitReader;
///Field `EXCOL` reader - Excessive Collisions
pub type EXCOL_R = crate::BitReader;
///Field `RWT` reader - Receive Watchdog Timeout
pub type RWT_R = crate::BitReader;
impl R {
    ///Bit 0 - Transmit Jabber Timeout
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - No Carrier
    #[inline(always)]
    pub fn ncarr(&self) -> NCARR_R {
        NCARR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Loss of Carrier
    #[inline(always)]
    pub fn lcarr(&self) -> LCARR_R {
        LCARR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Excessive Deferral
    #[inline(always)]
    pub fn exdef(&self) -> EXDEF_R {
        EXDEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Late Collision
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Excessive Collisions
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Receive Watchdog Timeout
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
/**Rx Tx status register

You can [`read`](crate::Reg::read) this register and get [`macrx_tx_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#Ethernet_MAC:MACRxTxSR)*/
pub struct MACRX_TX_SRrs;
impl crate::RegisterSpec for MACRX_TX_SRrs {
    type Ux = u32;
}
///`read()` method returns [`macrx_tx_sr::R`](R) reader structure
impl crate::Readable for MACRX_TX_SRrs {}
///`reset()` method sets MACRxTxSR to value 0
impl crate::Resettable for MACRX_TX_SRrs {}
