#[doc = "Register `MACPFR` reader"]
pub type R = crate::R<MACPFRrs>;
#[doc = "Register `MACPFR` writer"]
pub type W = crate::W<MACPFRrs>;
#[doc = "Field `PR` reader - Promiscuous Mode"]
pub type PR_R = crate::BitReader;
#[doc = "Field `PR` writer - Promiscuous Mode"]
pub type PR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUC` reader - Hash Unicast"]
pub type HUC_R = crate::BitReader;
#[doc = "Field `HUC` writer - Hash Unicast"]
pub type HUC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMC` reader - Hash Multicast"]
pub type HMC_R = crate::BitReader;
#[doc = "Field `HMC` writer - Hash Multicast"]
pub type HMC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAIF` reader - DA Inverse Filtering"]
pub type DAIF_R = crate::BitReader;
#[doc = "Field `DAIF` writer - DA Inverse Filtering"]
pub type DAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM` reader - Pass All Multicast"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - Pass All Multicast"]
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBF` reader - Disable Broadcast Packets"]
pub type DBF_R = crate::BitReader;
#[doc = "Field `DBF` writer - Disable Broadcast Packets"]
pub type DBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCF` reader - Pass Control Packets"]
pub type PCF_R = crate::FieldReader;
#[doc = "Field `PCF` writer - Pass Control Packets"]
pub type PCF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAIF` reader - SA Inverse Filtering"]
pub type SAIF_R = crate::BitReader;
#[doc = "Field `SAIF` writer - SA Inverse Filtering"]
pub type SAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAF` reader - Source Address Filter Enable"]
pub type SAF_R = crate::BitReader;
#[doc = "Field `SAF` writer - Source Address Filter Enable"]
pub type SAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPF` reader - Hash or Perfect Filter"]
pub type HPF_R = crate::BitReader;
#[doc = "Field `HPF` writer - Hash or Perfect Filter"]
pub type HPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTFE` reader - VLAN Tag Filter Enable"]
pub type VTFE_R = crate::BitReader;
#[doc = "Field `VTFE` writer - VLAN Tag Filter Enable"]
pub type VTFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPFE` reader - Layer 3 and Layer 4 Filter Enable"]
pub type IPFE_R = crate::BitReader;
#[doc = "Field `IPFE` writer - Layer 3 and Layer 4 Filter Enable"]
pub type IPFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNTU` reader - Drop Non-TCP/UDP over IP Packets"]
pub type DNTU_R = crate::BitReader;
#[doc = "Field `DNTU` writer - Drop Non-TCP/UDP over IP Packets"]
pub type DNTU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RA` reader - Receive All"]
pub type RA_R = crate::BitReader;
#[doc = "Field `RA` writer - Receive All"]
pub type RA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    pub fn huc(&self) -> HUC_R {
        HUC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    pub fn hmc(&self) -> HMC_R {
        HMC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DA Inverse Filtering"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Broadcast Packets"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Packets"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SA Inverse Filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter Enable"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable"]
    #[inline(always)]
    pub fn vtfe(&self) -> VTFE_R {
        VTFE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Layer 3 and Layer 4 Filter Enable"]
    #[inline(always)]
    pub fn ipfe(&self) -> IPFE_R {
        IPFE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Drop Non-TCP/UDP over IP Packets"]
    #[inline(always)]
    pub fn dntu(&self) -> DNTU_R {
        DNTU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<MACPFRrs> {
        PR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    #[must_use]
    pub fn huc(&mut self) -> HUC_W<MACPFRrs> {
        HUC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    #[must_use]
    pub fn hmc(&mut self) -> HMC_W<MACPFRrs> {
        HMC_W::new(self, 2)
    }
    #[doc = "Bit 3 - DA Inverse Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn daif(&mut self) -> DAIF_W<MACPFRrs> {
        DAIF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<MACPFRrs> {
        PM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Disable Broadcast Packets"]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DBF_W<MACPFRrs> {
        DBF_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Pass Control Packets"]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PCF_W<MACPFRrs> {
        PCF_W::new(self, 6)
    }
    #[doc = "Bit 8 - SA Inverse Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn saif(&mut self) -> SAIF_W<MACPFRrs> {
        SAIF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Source Address Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn saf(&mut self) -> SAF_W<MACPFRrs> {
        SAF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    #[must_use]
    pub fn hpf(&mut self) -> HPF_W<MACPFRrs> {
        HPF_W::new(self, 10)
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vtfe(&mut self) -> VTFE_W<MACPFRrs> {
        VTFE_W::new(self, 16)
    }
    #[doc = "Bit 20 - Layer 3 and Layer 4 Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ipfe(&mut self) -> IPFE_W<MACPFRrs> {
        IPFE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Drop Non-TCP/UDP over IP Packets"]
    #[inline(always)]
    #[must_use]
    pub fn dntu(&mut self) -> DNTU_W<MACPFRrs> {
        DNTU_W::new(self, 21)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<MACPFRrs> {
        RA_W::new(self, 31)
    }
}
#[doc = "Packet filtering control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPFRrs;
impl crate::RegisterSpec for MACPFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macpfr::R`](R) reader structure"]
impl crate::Readable for MACPFRrs {}
#[doc = "`write(|w| ..)` method takes [`macpfr::W`](W) writer structure"]
impl crate::Writable for MACPFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACPFR to value 0"]
impl crate::Resettable for MACPFRrs {
    const RESET_VALUE: u32 = 0;
}
