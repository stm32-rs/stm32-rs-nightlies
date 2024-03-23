#[doc = "Register `STGENR_PIDR4` reader"]
pub type R = crate::R<STGENR_PIDR4rs>;
#[doc = "Field `DES_2` reader - DES_2"]
pub type DES_2_R = crate::FieldReader;
#[doc = "Field `SIZE` reader - SIZE"]
pub type SIZE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - DES_2"]
    #[inline(always)]
    pub fn des_2(&self) -> DES_2_R {
        DES_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SIZE"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "STGENR peripheral ID4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_pidr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENR_PIDR4rs;
impl crate::RegisterSpec for STGENR_PIDR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenr_pidr4::R`](R) reader structure"]
impl crate::Readable for STGENR_PIDR4rs {}
#[doc = "`reset()` method sets STGENR_PIDR4 to value 0x04"]
impl crate::Resettable for STGENR_PIDR4rs {
    const RESET_VALUE: u32 = 0x04;
}
