#[doc = "Register `MACHWF0R` reader"]
pub type R = crate::R<MACHWF0Rrs>;
#[doc = "Field `MIISEL` reader - 10 or 100 Mbps Support This bit is set to 1 when 10/100 Mbps is selected as operating mode."]
pub type MIISEL_R = crate::BitReader;
#[doc = "Field `GMIISEL` reader - 1000 Mbps Support This bit is set to 1 when 1000 Mbps is selected as operating mode."]
pub type GMIISEL_R = crate::BitReader;
#[doc = "Field `HDSEL` reader - Half-duplex Support This bit is set to 1 when the Half-duplex mode is selected"]
pub type HDSEL_R = crate::BitReader;
#[doc = "Field `PCSSEL` reader - PCS Registers (TBI, SGMII, or RTBI PHY interface) This bit is set to 1 when the TBI, SGMII, or RTBI PHY interface option is selected"]
pub type PCSSEL_R = crate::BitReader;
#[doc = "Field `VLHASH` reader - VLAN Hash Filter Selected This bit is set to 1 when the Enable VLAN Hash Table Based Filtering option is selected"]
pub type VLHASH_R = crate::BitReader;
#[doc = "Field `SMASEL` reader - SMA (MDIO) Interface This bit is set to 1 when the Enable Station Management (MDIO Interface) option is selected"]
pub type SMASEL_R = crate::BitReader;
#[doc = "Field `RWKSEL` reader - PMT Remote Wakeup Packet Enable This bit is set to 1 when the Enable Remote wakeup Packet Detection option is selected"]
pub type RWKSEL_R = crate::BitReader;
#[doc = "Field `MGKSEL` reader - PMT Magic Packet Enable This bit is set to 1 when the Enable Magic Packet Detection option is selected"]
pub type MGKSEL_R = crate::BitReader;
#[doc = "Field `MMCSEL` reader - RMON Module Enable This bit is set to 1 when the Enable MAC management counters (MMC) option is selected"]
pub type MMCSEL_R = crate::BitReader;
#[doc = "Field `ARPOFFSEL` reader - ARP Offload Enabled This bit is set to 1 when the Enable IPv4 ARP Offload option is selected"]
pub type ARPOFFSEL_R = crate::BitReader;
#[doc = "Field `TSSEL` reader - IEEE 1588-2008 Timestamp Enabled This bit is set to 1 when the Enable IEEE 1588 Timestamp Support option is selected"]
pub type TSSEL_R = crate::BitReader;
#[doc = "Field `EEESEL` reader - Energy Efficient Ethernet Enabled This bit is set to 1 when the Enable Energy Efficient Ethernet (EEE) option is selected"]
pub type EEESEL_R = crate::BitReader;
#[doc = "Field `TXCOESEL` reader - Transmit Checksum Offload Enabled This bit is set to 1 when the Enable Transmit TCP/IP Checksum Insertion option is selected"]
pub type TXCOESEL_R = crate::BitReader;
#[doc = "Field `RXCOESEL` reader - Receive Checksum Offload Enabled This bit is set to 1 when the Enable Receive TCP/IP Checksum Check option is selected"]
pub type RXCOESEL_R = crate::BitReader;
#[doc = "Field `ADDMACADRSEL` reader - MAC Addresses 1-31 Selected This bit is set to 1 when the Enable Additional 1-31 MAC Address Registers option is selected"]
pub type ADDMACADRSEL_R = crate::FieldReader;
#[doc = "Field `MACADR32SEL` reader - MAC Addresses 32-63 Selected This bit is set to 1 when the Enable Additional 32 MAC Address Registers (32-63) option is selected"]
pub type MACADR32SEL_R = crate::BitReader;
#[doc = "Field `MACADR64SEL` reader - MAC Addresses 64-127 Selected This bit is set to 1 when the Enable Additional 64 MAC Address Registers (64-127) option is selected"]
pub type MACADR64SEL_R = crate::BitReader;
#[doc = "Field `TSSTSSEL` reader - Timestamp System Time Source This bit indicates the source of the Timestamp system time: This bit is set to 1 when the Enable IEEE 1588 Timestamp Support option is selected"]
pub type TSSTSSEL_R = crate::FieldReader;
#[doc = "Field `SAVLANINS` reader - Source Address or VLAN Insertion Enable This bit is set to 1 when the Enable SA and VLAN Insertion on Tx option is selected"]
pub type SAVLANINS_R = crate::BitReader;
#[doc = "Field `ACTPHYSEL` reader - Active PHY Selected When you have multiple PHY interfaces in your configuration, this field indicates the sampled value of phy_intf_sel_i during reset de-assertion: Others: Reserved, must not be used"]
pub type ACTPHYSEL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 10 or 100 Mbps Support This bit is set to 1 when 10/100 Mbps is selected as operating mode."]
    #[inline(always)]
    pub fn miisel(&self) -> MIISEL_R {
        MIISEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1000 Mbps Support This bit is set to 1 when 1000 Mbps is selected as operating mode."]
    #[inline(always)]
    pub fn gmiisel(&self) -> GMIISEL_R {
        GMIISEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half-duplex Support This bit is set to 1 when the Half-duplex mode is selected"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCS Registers (TBI, SGMII, or RTBI PHY interface) This bit is set to 1 when the TBI, SGMII, or RTBI PHY interface option is selected"]
    #[inline(always)]
    pub fn pcssel(&self) -> PCSSEL_R {
        PCSSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VLAN Hash Filter Selected This bit is set to 1 when the Enable VLAN Hash Table Based Filtering option is selected"]
    #[inline(always)]
    pub fn vlhash(&self) -> VLHASH_R {
        VLHASH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMA (MDIO) Interface This bit is set to 1 when the Enable Station Management (MDIO Interface) option is selected"]
    #[inline(always)]
    pub fn smasel(&self) -> SMASEL_R {
        SMASEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PMT Remote Wakeup Packet Enable This bit is set to 1 when the Enable Remote wakeup Packet Detection option is selected"]
    #[inline(always)]
    pub fn rwksel(&self) -> RWKSEL_R {
        RWKSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PMT Magic Packet Enable This bit is set to 1 when the Enable Magic Packet Detection option is selected"]
    #[inline(always)]
    pub fn mgksel(&self) -> MGKSEL_R {
        MGKSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RMON Module Enable This bit is set to 1 when the Enable MAC management counters (MMC) option is selected"]
    #[inline(always)]
    pub fn mmcsel(&self) -> MMCSEL_R {
        MMCSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ARP Offload Enabled This bit is set to 1 when the Enable IPv4 ARP Offload option is selected"]
    #[inline(always)]
    pub fn arpoffsel(&self) -> ARPOFFSEL_R {
        ARPOFFSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - IEEE 1588-2008 Timestamp Enabled This bit is set to 1 when the Enable IEEE 1588 Timestamp Support option is selected"]
    #[inline(always)]
    pub fn tssel(&self) -> TSSEL_R {
        TSSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Energy Efficient Ethernet Enabled This bit is set to 1 when the Enable Energy Efficient Ethernet (EEE) option is selected"]
    #[inline(always)]
    pub fn eeesel(&self) -> EEESEL_R {
        EEESEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit Checksum Offload Enabled This bit is set to 1 when the Enable Transmit TCP/IP Checksum Insertion option is selected"]
    #[inline(always)]
    pub fn txcoesel(&self) -> TXCOESEL_R {
        TXCOESEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive Checksum Offload Enabled This bit is set to 1 when the Enable Receive TCP/IP Checksum Check option is selected"]
    #[inline(always)]
    pub fn rxcoesel(&self) -> RXCOESEL_R {
        RXCOESEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:22 - MAC Addresses 1-31 Selected This bit is set to 1 when the Enable Additional 1-31 MAC Address Registers option is selected"]
    #[inline(always)]
    pub fn addmacadrsel(&self) -> ADDMACADRSEL_R {
        ADDMACADRSEL_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - MAC Addresses 32-63 Selected This bit is set to 1 when the Enable Additional 32 MAC Address Registers (32-63) option is selected"]
    #[inline(always)]
    pub fn macadr32sel(&self) -> MACADR32SEL_R {
        MACADR32SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MAC Addresses 64-127 Selected This bit is set to 1 when the Enable Additional 64 MAC Address Registers (64-127) option is selected"]
    #[inline(always)]
    pub fn macadr64sel(&self) -> MACADR64SEL_R {
        MACADR64SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Timestamp System Time Source This bit indicates the source of the Timestamp system time: This bit is set to 1 when the Enable IEEE 1588 Timestamp Support option is selected"]
    #[inline(always)]
    pub fn tsstssel(&self) -> TSSTSSEL_R {
        TSSTSSEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Source Address or VLAN Insertion Enable This bit is set to 1 when the Enable SA and VLAN Insertion on Tx option is selected"]
    #[inline(always)]
    pub fn savlanins(&self) -> SAVLANINS_R {
        SAVLANINS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Active PHY Selected When you have multiple PHY interfaces in your configuration, this field indicates the sampled value of phy_intf_sel_i during reset de-assertion: Others: Reserved, must not be used"]
    #[inline(always)]
    pub fn actphysel(&self) -> ACTPHYSEL_R {
        ACTPHYSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "HW feature 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machwf0r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHWF0Rrs;
impl crate::RegisterSpec for MACHWF0Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machwf0r::R`](R) reader structure"]
impl crate::Readable for MACHWF0Rrs {}
#[doc = "`reset()` method sets MACHWF0R to value 0x0a0d_73f7"]
impl crate::Resettable for MACHWF0Rrs {
    const RESET_VALUE: u32 = 0x0a0d_73f7;
}
