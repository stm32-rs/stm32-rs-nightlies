#[doc = "Register `SPDIFRX_CSR` reader"]
pub type R = crate::R<SPDIFRX_CSRrs>;
#[doc = "Field `USR` reader - USR"]
pub type USR_R = crate::FieldReader<u16>;
#[doc = "Field `CS` reader - CS"]
pub type CS_R = crate::FieldReader;
#[doc = "Field `SOB` reader - SOB"]
pub type SOB_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - USR"]
    #[inline(always)]
    pub fn usr(&self) -> USR_R {
        USR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - CS"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - SOB"]
    #[inline(always)]
    pub fn sob(&self) -> SOB_R {
        SOB_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_csr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPDIFRX_CSRrs;
impl crate::RegisterSpec for SPDIFRX_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdifrx_csr::R`](R) reader structure"]
impl crate::Readable for SPDIFRX_CSRrs {}
#[doc = "`reset()` method sets SPDIFRX_CSR to value 0"]
impl crate::Resettable for SPDIFRX_CSRrs {
    const RESET_VALUE: u32 = 0;
}
