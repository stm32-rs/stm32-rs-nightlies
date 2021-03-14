#[doc = "Reader of register ITLINE31"]
pub type R = crate::R<u32, super::ITLINE31>;
#[doc = "Reader of field `RNG`"]
pub type RNG_R = crate::R<bool, bool>;
#[doc = "Reader of field `AES`"]
pub type AES_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AES"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
