#[doc = "Register `ETH_MACPFR` reader"]
pub type R = crate::R<ETH_MACPFRrs>;
#[doc = "Register `ETH_MACPFR` writer"]
pub type W = crate::W<ETH_MACPFRrs>;
#[doc = "Field `PR` reader - PR"]
pub type PR_R = crate::BitReader;
#[doc = "Field `PR` writer - PR"]
pub type PR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUC` reader - HUC"]
pub type HUC_R = crate::BitReader;
#[doc = "Field `HUC` writer - HUC"]
pub type HUC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMC` reader - HMC"]
pub type HMC_R = crate::BitReader;
#[doc = "Field `HMC` writer - HMC"]
pub type HMC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAIF` reader - DAIF"]
pub type DAIF_R = crate::BitReader;
#[doc = "Field `DAIF` writer - DAIF"]
pub type DAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM` reader - PM"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - PM"]
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBF` reader - DBF"]
pub type DBF_R = crate::BitReader;
#[doc = "Field `DBF` writer - DBF"]
pub type DBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCF` reader - PCF"]
pub type PCF_R = crate::FieldReader;
#[doc = "Field `PCF` writer - PCF"]
pub type PCF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAIF` reader - SAIF"]
pub type SAIF_R = crate::BitReader;
#[doc = "Field `SAIF` writer - SAIF"]
pub type SAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAF` reader - SAF"]
pub type SAF_R = crate::BitReader;
#[doc = "Field `SAF` writer - SAF"]
pub type SAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPF` reader - HPF"]
pub type HPF_R = crate::BitReader;
#[doc = "Field `HPF` writer - HPF"]
pub type HPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTFE` reader - VTFE"]
pub type VTFE_R = crate::BitReader;
#[doc = "Field `VTFE` writer - VTFE"]
pub type VTFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPFE` reader - IPFE"]
pub type IPFE_R = crate::BitReader;
#[doc = "Field `IPFE` writer - IPFE"]
pub type IPFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNTU` reader - DNTU"]
pub type DNTU_R = crate::BitReader;
#[doc = "Field `DNTU` writer - DNTU"]
pub type DNTU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RA` reader - RA"]
pub type RA_R = crate::BitReader;
#[doc = "Field `RA` writer - RA"]
pub type RA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PR"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HUC"]
    #[inline(always)]
    pub fn huc(&self) -> HUC_R {
        HUC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HMC"]
    #[inline(always)]
    pub fn hmc(&self) -> HMC_R {
        HMC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DAIF"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DBF"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - PCF"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SAIF"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SAF"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HPF"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - VTFE"]
    #[inline(always)]
    pub fn vtfe(&self) -> VTFE_R {
        VTFE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - IPFE"]
    #[inline(always)]
    pub fn ipfe(&self) -> IPFE_R {
        IPFE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DNTU"]
    #[inline(always)]
    pub fn dntu(&self) -> DNTU_R {
        DNTU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - RA"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PR"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<ETH_MACPFRrs> {
        PR_W::new(self, 0)
    }
    #[doc = "Bit 1 - HUC"]
    #[inline(always)]
    #[must_use]
    pub fn huc(&mut self) -> HUC_W<ETH_MACPFRrs> {
        HUC_W::new(self, 1)
    }
    #[doc = "Bit 2 - HMC"]
    #[inline(always)]
    #[must_use]
    pub fn hmc(&mut self) -> HMC_W<ETH_MACPFRrs> {
        HMC_W::new(self, 2)
    }
    #[doc = "Bit 3 - DAIF"]
    #[inline(always)]
    #[must_use]
    pub fn daif(&mut self) -> DAIF_W<ETH_MACPFRrs> {
        DAIF_W::new(self, 3)
    }
    #[doc = "Bit 4 - PM"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<ETH_MACPFRrs> {
        PM_W::new(self, 4)
    }
    #[doc = "Bit 5 - DBF"]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DBF_W<ETH_MACPFRrs> {
        DBF_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - PCF"]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PCF_W<ETH_MACPFRrs> {
        PCF_W::new(self, 6)
    }
    #[doc = "Bit 8 - SAIF"]
    #[inline(always)]
    #[must_use]
    pub fn saif(&mut self) -> SAIF_W<ETH_MACPFRrs> {
        SAIF_W::new(self, 8)
    }
    #[doc = "Bit 9 - SAF"]
    #[inline(always)]
    #[must_use]
    pub fn saf(&mut self) -> SAF_W<ETH_MACPFRrs> {
        SAF_W::new(self, 9)
    }
    #[doc = "Bit 10 - HPF"]
    #[inline(always)]
    #[must_use]
    pub fn hpf(&mut self) -> HPF_W<ETH_MACPFRrs> {
        HPF_W::new(self, 10)
    }
    #[doc = "Bit 16 - VTFE"]
    #[inline(always)]
    #[must_use]
    pub fn vtfe(&mut self) -> VTFE_W<ETH_MACPFRrs> {
        VTFE_W::new(self, 16)
    }
    #[doc = "Bit 20 - IPFE"]
    #[inline(always)]
    #[must_use]
    pub fn ipfe(&mut self) -> IPFE_W<ETH_MACPFRrs> {
        IPFE_W::new(self, 20)
    }
    #[doc = "Bit 21 - DNTU"]
    #[inline(always)]
    #[must_use]
    pub fn dntu(&mut self) -> DNTU_W<ETH_MACPFRrs> {
        DNTU_W::new(self, 21)
    }
    #[doc = "Bit 31 - RA"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<ETH_MACPFRrs> {
        RA_W::new(self, 31)
    }
}
#[doc = "The MAC Packet Filter register contains the filter controls for receiving packets. Some of the controls from this register go to the address check block of the MAC which performs the first level of address filtering. The second level of filtering is performed on the incoming packet based on other controls such as Pass Bad Packets and Pass Control Packets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macpfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macpfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACPFRrs;
impl crate::RegisterSpec for ETH_MACPFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macpfr::R`](R) reader structure"]
impl crate::Readable for ETH_MACPFRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macpfr::W`](W) writer structure"]
impl crate::Writable for ETH_MACPFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACPFR to value 0"]
impl crate::Resettable for ETH_MACPFRrs {
    const RESET_VALUE: u32 = 0;
}
