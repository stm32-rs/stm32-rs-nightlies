///Register `MACPFR` reader
pub type R = crate::R<MACPFRrs>;
///Register `MACPFR` writer
pub type W = crate::W<MACPFRrs>;
///Field `PR` reader - Promiscuous Mode
pub type PR_R = crate::BitReader;
///Field `PR` writer - Promiscuous Mode
pub type PR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HUC` reader - Hash Unicast
pub type HUC_R = crate::BitReader;
///Field `HUC` writer - Hash Unicast
pub type HUC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HMC` reader - Hash Multicast
pub type HMC_R = crate::BitReader;
///Field `HMC` writer - Hash Multicast
pub type HMC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAIF` reader - DA Inverse Filtering
pub type DAIF_R = crate::BitReader;
///Field `DAIF` writer - DA Inverse Filtering
pub type DAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PM` reader - Pass All Multicast
pub type PM_R = crate::BitReader;
///Field `PM` writer - Pass All Multicast
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBF` reader - Disable Broadcast Packets
pub type DBF_R = crate::BitReader;
///Field `DBF` writer - Disable Broadcast Packets
pub type DBF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCF` reader - Pass Control Packets
pub type PCF_R = crate::FieldReader;
///Field `PCF` writer - Pass Control Packets
pub type PCF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SAIF` reader - SA Inverse Filtering
pub type SAIF_R = crate::BitReader;
///Field `SAIF` writer - SA Inverse Filtering
pub type SAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAF` reader - Source Address Filter Enable
pub type SAF_R = crate::BitReader;
///Field `SAF` writer - Source Address Filter Enable
pub type SAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPF` reader - Hash or Perfect Filter
pub type HPF_R = crate::BitReader;
///Field `HPF` writer - Hash or Perfect Filter
pub type HPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VTFE` reader - VLAN Tag Filter Enable
pub type VTFE_R = crate::BitReader;
///Field `VTFE` writer - VLAN Tag Filter Enable
pub type VTFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPFE` reader - Layer 3 and Layer 4 Filter Enable
pub type IPFE_R = crate::BitReader;
///Field `IPFE` writer - Layer 3 and Layer 4 Filter Enable
pub type IPFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DNTU` reader - Drop Non-TCP/UDP over IP Packets
pub type DNTU_R = crate::BitReader;
///Field `DNTU` writer - Drop Non-TCP/UDP over IP Packets
pub type DNTU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RA` reader - Receive All
pub type RA_R = crate::BitReader;
///Field `RA` writer - Receive All
pub type RA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Promiscuous Mode
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Hash Unicast
    #[inline(always)]
    pub fn huc(&self) -> HUC_R {
        HUC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Hash Multicast
    #[inline(always)]
    pub fn hmc(&self) -> HMC_R {
        HMC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DA Inverse Filtering
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pass All Multicast
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Disable Broadcast Packets
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Pass Control Packets
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - SA Inverse Filtering
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Source Address Filter Enable
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Hash or Perfect Filter
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - VLAN Tag Filter Enable
    #[inline(always)]
    pub fn vtfe(&self) -> VTFE_R {
        VTFE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - Layer 3 and Layer 4 Filter Enable
    #[inline(always)]
    pub fn ipfe(&self) -> IPFE_R {
        IPFE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Drop Non-TCP/UDP over IP Packets
    #[inline(always)]
    pub fn dntu(&self) -> DNTU_R {
        DNTU_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 31 - Receive All
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPFR")
            .field("pr", &self.pr())
            .field("huc", &self.huc())
            .field("hmc", &self.hmc())
            .field("daif", &self.daif())
            .field("pm", &self.pm())
            .field("dbf", &self.dbf())
            .field("pcf", &self.pcf())
            .field("saif", &self.saif())
            .field("saf", &self.saf())
            .field("hpf", &self.hpf())
            .field("vtfe", &self.vtfe())
            .field("ipfe", &self.ipfe())
            .field("dntu", &self.dntu())
            .field("ra", &self.ra())
            .finish()
    }
}
impl W {
    ///Bit 0 - Promiscuous Mode
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<'_, MACPFRrs> {
        PR_W::new(self, 0)
    }
    ///Bit 1 - Hash Unicast
    #[inline(always)]
    pub fn huc(&mut self) -> HUC_W<'_, MACPFRrs> {
        HUC_W::new(self, 1)
    }
    ///Bit 2 - Hash Multicast
    #[inline(always)]
    pub fn hmc(&mut self) -> HMC_W<'_, MACPFRrs> {
        HMC_W::new(self, 2)
    }
    ///Bit 3 - DA Inverse Filtering
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W<'_, MACPFRrs> {
        DAIF_W::new(self, 3)
    }
    ///Bit 4 - Pass All Multicast
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<'_, MACPFRrs> {
        PM_W::new(self, 4)
    }
    ///Bit 5 - Disable Broadcast Packets
    #[inline(always)]
    pub fn dbf(&mut self) -> DBF_W<'_, MACPFRrs> {
        DBF_W::new(self, 5)
    }
    ///Bits 6:7 - Pass Control Packets
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W<'_, MACPFRrs> {
        PCF_W::new(self, 6)
    }
    ///Bit 8 - SA Inverse Filtering
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W<'_, MACPFRrs> {
        SAIF_W::new(self, 8)
    }
    ///Bit 9 - Source Address Filter Enable
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W<'_, MACPFRrs> {
        SAF_W::new(self, 9)
    }
    ///Bit 10 - Hash or Perfect Filter
    #[inline(always)]
    pub fn hpf(&mut self) -> HPF_W<'_, MACPFRrs> {
        HPF_W::new(self, 10)
    }
    ///Bit 16 - VLAN Tag Filter Enable
    #[inline(always)]
    pub fn vtfe(&mut self) -> VTFE_W<'_, MACPFRrs> {
        VTFE_W::new(self, 16)
    }
    ///Bit 20 - Layer 3 and Layer 4 Filter Enable
    #[inline(always)]
    pub fn ipfe(&mut self) -> IPFE_W<'_, MACPFRrs> {
        IPFE_W::new(self, 20)
    }
    ///Bit 21 - Drop Non-TCP/UDP over IP Packets
    #[inline(always)]
    pub fn dntu(&mut self) -> DNTU_W<'_, MACPFRrs> {
        DNTU_W::new(self, 21)
    }
    ///Bit 31 - Receive All
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<'_, MACPFRrs> {
        RA_W::new(self, 31)
    }
}
/**Packet filtering control register

You can [`read`](crate::Reg::read) this register and get [`macpfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#Ethernet_MAC:MACPFR)*/
pub struct MACPFRrs;
impl crate::RegisterSpec for MACPFRrs {
    type Ux = u32;
}
///`read()` method returns [`macpfr::R`](R) reader structure
impl crate::Readable for MACPFRrs {}
///`write(|w| ..)` method takes [`macpfr::W`](W) writer structure
impl crate::Writable for MACPFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPFR to value 0
impl crate::Resettable for MACPFRrs {}
