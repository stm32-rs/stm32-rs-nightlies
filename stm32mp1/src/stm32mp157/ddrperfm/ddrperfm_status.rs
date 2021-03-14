#[doc = "Reader of register DDRPERFM_STATUS"]
pub type R = crate::R<u32, super::DDRPERFM_STATUS>;
#[doc = "Reader of field `COVF`"]
pub type COVF_R = crate::R<u8, u8>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOVF`"]
pub type TOVF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - COVF"]
    #[inline(always)]
    pub fn covf(&self) -> COVF_R {
        COVF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TOVF"]
    #[inline(always)]
    pub fn tovf(&self) -> TOVF_R {
        TOVF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
