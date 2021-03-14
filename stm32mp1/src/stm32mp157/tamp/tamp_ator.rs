#[doc = "Reader of register TAMP_ATOR"]
pub type R = crate::R<u32, super::TAMP_ATOR>;
#[doc = "Reader of field `PRNG`"]
pub type PRNG_R = crate::R<u8, u8>;
#[doc = "Reader of field `SEEDF`"]
pub type SEEDF_R = crate::R<bool, bool>;
#[doc = "Reader of field `INITS`"]
pub type INITS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7 - PRNG"]
    #[inline(always)]
    pub fn prng(&self) -> PRNG_R {
        PRNG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - SEEDF"]
    #[inline(always)]
    pub fn seedf(&self) -> SEEDF_R {
        SEEDF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - INITS"]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
