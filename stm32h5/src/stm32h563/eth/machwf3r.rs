///Register `MACHWF3R` reader
pub type R = crate::R<MACHWF3Rrs>;
///Field `NRVF` reader - Number of Extended VLAN Tag Filters Enabled This field indicates the Number of Extended VLAN Tag Filters selected: 110 to 111: Reserved, must not be used
pub type NRVF_R = crate::FieldReader;
///Field `CBTISEL` reader - Queue/Channel based VLAN tag insertion on Tx enable This bit is set to 1 when the Enable Queue/Channel based VLAN tag insertion on Tx feature is selected.
pub type CBTISEL_R = crate::BitReader;
///Field `DVLAN` reader - Double VLAN processing enable This bit is set to 1 when Double VLAN processing is enabled.
pub type DVLAN_R = crate::BitReader;
impl R {
    ///Bits 0:2 - Number of Extended VLAN Tag Filters Enabled This field indicates the Number of Extended VLAN Tag Filters selected: 110 to 111: Reserved, must not be used
    #[inline(always)]
    pub fn nrvf(&self) -> NRVF_R {
        NRVF_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - Queue/Channel based VLAN tag insertion on Tx enable This bit is set to 1 when the Enable Queue/Channel based VLAN tag insertion on Tx feature is selected.
    #[inline(always)]
    pub fn cbtisel(&self) -> CBTISEL_R {
        CBTISEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Double VLAN processing enable This bit is set to 1 when Double VLAN processing is enabled.
    #[inline(always)]
    pub fn dvlan(&self) -> DVLAN_R {
        DVLAN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHWF3R")
            .field("nrvf", &self.nrvf())
            .field("cbtisel", &self.cbtisel())
            .field("dvlan", &self.dvlan())
            .finish()
    }
}
/**HW feature 3 register

You can [`read`](crate::Reg::read) this register and get [`machwf3r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#ETH:MACHWF3R)*/
pub struct MACHWF3Rrs;
impl crate::RegisterSpec for MACHWF3Rrs {
    type Ux = u32;
}
///`read()` method returns [`machwf3r::R`](R) reader structure
impl crate::Readable for MACHWF3Rrs {}
///`reset()` method sets MACHWF3R to value 0x20
impl crate::Resettable for MACHWF3Rrs {
    const RESET_VALUE: u32 = 0x20;
}
