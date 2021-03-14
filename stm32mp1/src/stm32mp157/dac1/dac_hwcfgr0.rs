#[doc = "Reader of register DAC_HWCFGR0"]
pub type R = crate::R<u32, super::DAC_HWCFGR0>;
#[doc = "Reader of field `DUAL`"]
pub type DUAL_R = crate::R<u8, u8>;
#[doc = "Reader of field `LFSR`"]
pub type LFSR_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIANGLE`"]
pub type TRIANGLE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SAMPLE`"]
pub type SAMPLE_R = crate::R<u8, u8>;
#[doc = "Reader of field `OR_CFG`"]
pub type OR_CFG_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - DUAL"]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - LFSR"]
    #[inline(always)]
    pub fn lfsr(&self) -> LFSR_R {
        LFSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TRIANGLE"]
    #[inline(always)]
    pub fn triangle(&self) -> TRIANGLE_R {
        TRIANGLE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SAMPLE"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - OR_CFG"]
    #[inline(always)]
    pub fn or_cfg(&self) -> OR_CFG_R {
        OR_CFG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
