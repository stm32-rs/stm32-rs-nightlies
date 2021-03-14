#[doc = "Reader of register ITLINE23"]
pub type R = crate::R<u32, super::ITLINE23>;
#[doc = "Reader of field `I2C1`"]
pub type I2C1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - I2C1"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new((self.bits & 0x01) != 0)
    }
}
