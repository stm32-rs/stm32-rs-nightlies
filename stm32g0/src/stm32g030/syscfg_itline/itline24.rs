#[doc = "Reader of register ITLINE24"]
pub type R = crate::R<u32, super::ITLINE24>;
#[doc = "Reader of field `I2C2`"]
pub type I2C2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - I2C2"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new((self.bits & 0x01) != 0)
    }
}
