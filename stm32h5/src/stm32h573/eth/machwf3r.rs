#[doc = "Register `MACHWF3R` reader"]
pub type R = crate::R<MACHWF3Rrs>;
#[doc = "Field `NRVF` reader - Number of Extended VLAN Tag Filters Enabled This field indicates the Number of Extended VLAN Tag Filters selected: 110 to 111: Reserved, must not be used"]
pub type NRVF_R = crate::FieldReader;
#[doc = "Field `CBTISEL` reader - Queue/Channel based VLAN tag insertion on Tx enable This bit is set to 1 when the Enable Queue/Channel based VLAN tag insertion on Tx feature is selected."]
pub type CBTISEL_R = crate::BitReader;
#[doc = "Field `DVLAN` reader - Double VLAN processing enable This bit is set to 1 when Double VLAN processing is enabled."]
pub type DVLAN_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Number of Extended VLAN Tag Filters Enabled This field indicates the Number of Extended VLAN Tag Filters selected: 110 to 111: Reserved, must not be used"]
    #[inline(always)]
    pub fn nrvf(&self) -> NRVF_R {
        NRVF_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Queue/Channel based VLAN tag insertion on Tx enable This bit is set to 1 when the Enable Queue/Channel based VLAN tag insertion on Tx feature is selected."]
    #[inline(always)]
    pub fn cbtisel(&self) -> CBTISEL_R {
        CBTISEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Double VLAN processing enable This bit is set to 1 when Double VLAN processing is enabled."]
    #[inline(always)]
    pub fn dvlan(&self) -> DVLAN_R {
        DVLAN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "HW feature 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machwf3r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHWF3Rrs;
impl crate::RegisterSpec for MACHWF3Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machwf3r::R`](R) reader structure"]
impl crate::Readable for MACHWF3Rrs {}
#[doc = "`reset()` method sets MACHWF3R to value 0x20"]
impl crate::Resettable for MACHWF3Rrs {
    const RESET_VALUE: u32 = 0x20;
}
