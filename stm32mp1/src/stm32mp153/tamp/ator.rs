#[doc = "Register `ATOR` reader"]
pub type R = crate::R<ATORrs>;
#[doc = "Field `PRNG` reader - PRNG"]
pub type PRNG_R = crate::FieldReader;
#[doc = "Field `SEEDF` reader - SEEDF"]
pub type SEEDF_R = crate::BitReader;
#[doc = "Field `INITS` reader - INITS"]
pub type INITS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - PRNG"]
    #[inline(always)]
    pub fn prng(&self) -> PRNG_R {
        PRNG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - SEEDF"]
    #[inline(always)]
    pub fn seedf(&self) -> SEEDF_R {
        SEEDF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - INITS"]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ator::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATORrs;
impl crate::RegisterSpec for ATORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ator::R`](R) reader structure"]
impl crate::Readable for ATORrs {}
#[doc = "`reset()` method sets ATOR to value 0"]
impl crate::Resettable for ATORrs {
    const RESET_VALUE: u32 = 0;
}
