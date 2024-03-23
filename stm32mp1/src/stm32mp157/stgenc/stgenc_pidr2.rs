#[doc = "Register `STGENC_PIDR2` reader"]
pub type R = crate::R<STGENC_PIDR2rs>;
#[doc = "Field `DES_1` reader - DES_1"]
pub type DES_1_R = crate::FieldReader;
#[doc = "Field `JEDEC` reader - JEDEC"]
pub type JEDEC_R = crate::BitReader;
#[doc = "Field `REVISION` reader - REVISION"]
pub type REVISION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - DES_1"]
    #[inline(always)]
    pub fn des_1(&self) -> DES_1_R {
        DES_1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - JEDEC"]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - REVISION"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "STGENC peripheral ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenc_pidr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENC_PIDR2rs;
impl crate::RegisterSpec for STGENC_PIDR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenc_pidr2::R`](R) reader structure"]
impl crate::Readable for STGENC_PIDR2rs {}
#[doc = "`reset()` method sets STGENC_PIDR2 to value 0x1b"]
impl crate::Resettable for STGENC_PIDR2rs {
    const RESET_VALUE: u32 = 0x1b;
}
