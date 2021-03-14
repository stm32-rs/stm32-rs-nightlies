#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Reader of field `ADDRDY_MST`"]
pub type ADDRDY_MST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOSMP_MST`"]
pub type EOSMP_MST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC_MST`"]
pub type EOC_MST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOS_MST`"]
pub type EOS_MST_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVR_MST`"]
pub type OVR_MST_R = crate::R<bool, bool>;
#[doc = "Reader of field `JEOC_MST`"]
pub type JEOC_MST_R = crate::R<bool, bool>;
#[doc = "Reader of field `JEOS_MST`"]
pub type JEOS_MST_R = crate::R<bool, bool>;
#[doc = "Reader of field `AWD1_MST`"]
pub type AWD1_MST_R = crate::R<bool, bool>;
#[doc = "Reader of field `AWD2_MST`"]
pub type AWD2_MST_R = crate::R<bool, bool>;
#[doc = "Reader of field `AWD3_MST`"]
pub type AWD3_MST_R = crate::R<bool, bool>;
#[doc = "Reader of field `JQOVF_MST`"]
pub type JQOVF_MST_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADRDY_SLV`"]
pub type ADRDY_SLV_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOSMP_SLV`"]
pub type EOSMP_SLV_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC_SLV`"]
pub type EOC_SLV_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOS_SLV`"]
pub type EOS_SLV_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVR_SLV`"]
pub type OVR_SLV_R = crate::R<bool, bool>;
#[doc = "Reader of field `JEOC_SLV`"]
pub type JEOC_SLV_R = crate::R<bool, bool>;
#[doc = "Reader of field `JEOS_SLV`"]
pub type JEOS_SLV_R = crate::R<bool, bool>;
#[doc = "Reader of field `AWD1_SLV`"]
pub type AWD1_SLV_R = crate::R<bool, bool>;
#[doc = "Reader of field `AWD2_SLV`"]
pub type AWD2_SLV_R = crate::R<bool, bool>;
#[doc = "Reader of field `AWD3_SLV`"]
pub type AWD3_SLV_R = crate::R<bool, bool>;
#[doc = "Reader of field `JQOVF_SLV`"]
pub type JQOVF_SLV_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ADDRDY_MST"]
    #[inline(always)]
    pub fn addrdy_mst(&self) -> ADDRDY_MST_R {
        ADDRDY_MST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EOSMP_MST"]
    #[inline(always)]
    pub fn eosmp_mst(&self) -> EOSMP_MST_R {
        EOSMP_MST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EOC_MST"]
    #[inline(always)]
    pub fn eoc_mst(&self) -> EOC_MST_R {
        EOC_MST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EOS_MST"]
    #[inline(always)]
    pub fn eos_mst(&self) -> EOS_MST_R {
        EOS_MST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OVR_MST"]
    #[inline(always)]
    pub fn ovr_mst(&self) -> OVR_MST_R {
        OVR_MST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - JEOC_MST"]
    #[inline(always)]
    pub fn jeoc_mst(&self) -> JEOC_MST_R {
        JEOC_MST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - JEOS_MST"]
    #[inline(always)]
    pub fn jeos_mst(&self) -> JEOS_MST_R {
        JEOS_MST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AWD1_MST"]
    #[inline(always)]
    pub fn awd1_mst(&self) -> AWD1_MST_R {
        AWD1_MST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AWD2_MST"]
    #[inline(always)]
    pub fn awd2_mst(&self) -> AWD2_MST_R {
        AWD2_MST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AWD3_MST"]
    #[inline(always)]
    pub fn awd3_mst(&self) -> AWD3_MST_R {
        AWD3_MST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - JQOVF_MST"]
    #[inline(always)]
    pub fn jqovf_mst(&self) -> JQOVF_MST_R {
        JQOVF_MST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADRDY_SLV"]
    #[inline(always)]
    pub fn adrdy_slv(&self) -> ADRDY_SLV_R {
        ADRDY_SLV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - EOSMP_SLV"]
    #[inline(always)]
    pub fn eosmp_slv(&self) -> EOSMP_SLV_R {
        EOSMP_SLV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - End of regular conversion of the slave ADC"]
    #[inline(always)]
    pub fn eoc_slv(&self) -> EOC_SLV_R {
        EOC_SLV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - End of regular sequence flag of the slave ADC"]
    #[inline(always)]
    pub fn eos_slv(&self) -> EOS_SLV_R {
        EOS_SLV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Overrun flag of the slave ADC"]
    #[inline(always)]
    pub fn ovr_slv(&self) -> OVR_SLV_R {
        OVR_SLV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - End of injected conversion flag of the slave ADC"]
    #[inline(always)]
    pub fn jeoc_slv(&self) -> JEOC_SLV_R {
        JEOC_SLV_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - End of injected sequence flag of the slave ADC"]
    #[inline(always)]
    pub fn jeos_slv(&self) -> JEOS_SLV_R {
        JEOS_SLV_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog 1 flag of the slave ADC"]
    #[inline(always)]
    pub fn awd1_slv(&self) -> AWD1_SLV_R {
        AWD1_SLV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Analog watchdog 2 flag of the slave ADC"]
    #[inline(always)]
    pub fn awd2_slv(&self) -> AWD2_SLV_R {
        AWD2_SLV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Analog watchdog 3 flag of the slave ADC"]
    #[inline(always)]
    pub fn awd3_slv(&self) -> AWD3_SLV_R {
        AWD3_SLV_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Injected Context Queue Overflow flag of the slave ADC"]
    #[inline(always)]
    pub fn jqovf_slv(&self) -> JQOVF_SLV_R {
        JQOVF_SLV_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
