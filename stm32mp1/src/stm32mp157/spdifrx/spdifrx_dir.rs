#[doc = "Register `SPDIFRX_DIR` reader"]
pub type R = crate::R<SPDIFRX_DIRrs>;
#[doc = "Field `THI` reader - THI"]
pub type THI_R = crate::FieldReader<u16>;
#[doc = "Field `TLO` reader - TLO"]
pub type TLO_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - THI"]
    #[inline(always)]
    pub fn thi(&self) -> THI_R {
        THI_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - TLO"]
    #[inline(always)]
    pub fn tlo(&self) -> TLO_R {
        TLO_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "Debug information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_dir::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPDIFRX_DIRrs;
impl crate::RegisterSpec for SPDIFRX_DIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdifrx_dir::R`](R) reader structure"]
impl crate::Readable for SPDIFRX_DIRrs {}
#[doc = "`reset()` method sets SPDIFRX_DIR to value 0"]
impl crate::Resettable for SPDIFRX_DIRrs {
    const RESET_VALUE: u32 = 0;
}
