#[doc = "Reader of register I2C_PECR"]
pub type R = crate::R<u32, super::I2C_PECR>;
#[doc = "Reader of field `PEC`"]
pub type PEC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PEC"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new((self.bits & 0xff) as u8)
    }
}
