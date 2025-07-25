///Register `MACHWF0R` reader
pub type R = crate::R<MACHWF0Rrs>;
///Field `MIISEL` reader - 10 or 100 Mbps Support
pub type MIISEL_R = crate::BitReader;
///Field `GMIISEL` reader - 1000 Mbps Support
pub type GMIISEL_R = crate::BitReader;
///Field `HDSEL` reader - Half-duplex Support
pub type HDSEL_R = crate::BitReader;
///Field `PCSSEL` reader - PCS Registers (TBI, SGMII, or RTBI PHY interface)
pub type PCSSEL_R = crate::BitReader;
///Field `VLHASH` reader - VLAN Hash Filter Selected
pub type VLHASH_R = crate::BitReader;
///Field `SMASEL` reader - SMA (MDIO) Interface
pub type SMASEL_R = crate::BitReader;
///Field `RWKSEL` reader - PMT Remote wake-up Packet Enable
pub type RWKSEL_R = crate::BitReader;
///Field `MGKSEL` reader - PMT Magic Packet Enable
pub type MGKSEL_R = crate::BitReader;
///Field `MMCSEL` reader - RMON Module Enable
pub type MMCSEL_R = crate::BitReader;
///Field `ARPOFFSEL` reader - ARP Offload Enabled
pub type ARPOFFSEL_R = crate::BitReader;
///Field `TSSEL` reader - IEEE 1588-2008 Timestamp Enabled
pub type TSSEL_R = crate::BitReader;
///Field `EEESEL` reader - Energy Efficient Ethernet Enabled
pub type EEESEL_R = crate::BitReader;
///Field `TXCOESEL` reader - Transmit Checksum Offload Enabled
pub type TXCOESEL_R = crate::BitReader;
///Field `RXCOESEL` reader - Receive Checksum Offload Enabled
pub type RXCOESEL_R = crate::BitReader;
///Field `ADDMACADRSEL` reader - MAC Addresses 1-31 Selected
pub type ADDMACADRSEL_R = crate::FieldReader;
///Field `MACADR32SEL` reader - MAC Addresses 32-63 Selected
pub type MACADR32SEL_R = crate::BitReader;
///Field `MACADR64SEL` reader - MAC Addresses 64-127 Selected
pub type MACADR64SEL_R = crate::BitReader;
///Field `TSSTSSEL` reader - Timestamp System Time Source
pub type TSSTSSEL_R = crate::FieldReader;
///Field `SAVLANINS` reader - Source Address or VLAN Insertion Enable
pub type SAVLANINS_R = crate::BitReader;
///Field `ACTPHYSEL` reader - Active PHY Selected
pub type ACTPHYSEL_R = crate::FieldReader;
impl R {
    ///Bit 0 - 10 or 100 Mbps Support
    #[inline(always)]
    pub fn miisel(&self) -> MIISEL_R {
        MIISEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1000 Mbps Support
    #[inline(always)]
    pub fn gmiisel(&self) -> GMIISEL_R {
        GMIISEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Half-duplex Support
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PCS Registers (TBI, SGMII, or RTBI PHY interface)
    #[inline(always)]
    pub fn pcssel(&self) -> PCSSEL_R {
        PCSSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VLAN Hash Filter Selected
    #[inline(always)]
    pub fn vlhash(&self) -> VLHASH_R {
        VLHASH_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SMA (MDIO) Interface
    #[inline(always)]
    pub fn smasel(&self) -> SMASEL_R {
        SMASEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PMT Remote wake-up Packet Enable
    #[inline(always)]
    pub fn rwksel(&self) -> RWKSEL_R {
        RWKSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PMT Magic Packet Enable
    #[inline(always)]
    pub fn mgksel(&self) -> MGKSEL_R {
        MGKSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RMON Module Enable
    #[inline(always)]
    pub fn mmcsel(&self) -> MMCSEL_R {
        MMCSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ARP Offload Enabled
    #[inline(always)]
    pub fn arpoffsel(&self) -> ARPOFFSEL_R {
        ARPOFFSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - IEEE 1588-2008 Timestamp Enabled
    #[inline(always)]
    pub fn tssel(&self) -> TSSEL_R {
        TSSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Energy Efficient Ethernet Enabled
    #[inline(always)]
    pub fn eeesel(&self) -> EEESEL_R {
        EEESEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Transmit Checksum Offload Enabled
    #[inline(always)]
    pub fn txcoesel(&self) -> TXCOESEL_R {
        TXCOESEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Receive Checksum Offload Enabled
    #[inline(always)]
    pub fn rxcoesel(&self) -> RXCOESEL_R {
        RXCOESEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 18:22 - MAC Addresses 1-31 Selected
    #[inline(always)]
    pub fn addmacadrsel(&self) -> ADDMACADRSEL_R {
        ADDMACADRSEL_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bit 23 - MAC Addresses 32-63 Selected
    #[inline(always)]
    pub fn macadr32sel(&self) -> MACADR32SEL_R {
        MACADR32SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - MAC Addresses 64-127 Selected
    #[inline(always)]
    pub fn macadr64sel(&self) -> MACADR64SEL_R {
        MACADR64SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - Timestamp System Time Source
    #[inline(always)]
    pub fn tsstssel(&self) -> TSSTSSEL_R {
        TSSTSSEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 27 - Source Address or VLAN Insertion Enable
    #[inline(always)]
    pub fn savlanins(&self) -> SAVLANINS_R {
        SAVLANINS_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:30 - Active PHY Selected
    #[inline(always)]
    pub fn actphysel(&self) -> ACTPHYSEL_R {
        ACTPHYSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHWF0R")
            .field("miisel", &self.miisel())
            .field("gmiisel", &self.gmiisel())
            .field("hdsel", &self.hdsel())
            .field("pcssel", &self.pcssel())
            .field("vlhash", &self.vlhash())
            .field("smasel", &self.smasel())
            .field("rwksel", &self.rwksel())
            .field("mgksel", &self.mgksel())
            .field("mmcsel", &self.mmcsel())
            .field("arpoffsel", &self.arpoffsel())
            .field("tssel", &self.tssel())
            .field("eeesel", &self.eeesel())
            .field("txcoesel", &self.txcoesel())
            .field("rxcoesel", &self.rxcoesel())
            .field("addmacadrsel", &self.addmacadrsel())
            .field("macadr32sel", &self.macadr32sel())
            .field("macadr64sel", &self.macadr64sel())
            .field("tsstssel", &self.tsstssel())
            .field("savlanins", &self.savlanins())
            .field("actphysel", &self.actphysel())
            .finish()
    }
}
/**HW feature 0 register

You can [`read`](crate::Reg::read) this register and get [`machwf0r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACHWF0R)*/
pub struct MACHWF0Rrs;
impl crate::RegisterSpec for MACHWF0Rrs {
    type Ux = u32;
}
///`read()` method returns [`machwf0r::R`](R) reader structure
impl crate::Readable for MACHWF0Rrs {}
///`reset()` method sets MACHWF0R to value 0x0e0d_73f7
impl crate::Resettable for MACHWF0Rrs {
    const RESET_VALUE: u32 = 0x0e0d_73f7;
}
