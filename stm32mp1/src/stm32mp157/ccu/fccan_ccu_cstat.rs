#[doc = "Reader of register FCCAN_CCU_CSTAT"]
pub type R = crate::R<u32, super::FCCAN_CCU_CSTAT>;
#[doc = "Reader of field `OCPC`"]
pub type OCPC_R = crate::R<u32, u32>;
#[doc = "Reader of field `TQC`"]
pub type TQC_R = crate::R<u16, u16>;
#[doc = "Reader of field `CALS`"]
pub type CALS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:17 - OCPC"]
    #[inline(always)]
    pub fn ocpc(&self) -> OCPC_R {
        OCPC_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:28 - TQC"]
    #[inline(always)]
    pub fn tqc(&self) -> TQC_R {
        TQC_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bits 30:31 - CALS"]
    #[inline(always)]
    pub fn cals(&self) -> CALS_R {
        CALS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
