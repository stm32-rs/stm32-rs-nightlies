#[doc = "Register `DMAMFBOCR` reader"]
pub type R = crate::R<DMAMFBOCRrs>;
#[doc = "Field `MFC` reader - Missed frames by the controller"]
pub type MFC_R = crate::FieldReader<u16>;
#[doc = "Field `OMFC` reader - Overflow bit for missed frame counter"]
pub type OMFC_R = crate::BitReader;
#[doc = "Field `MFA` reader - Missed frames by the application"]
pub type MFA_R = crate::FieldReader<u16>;
#[doc = "Field `OFOC` reader - Overflow bit for FIFO overflow counter"]
pub type OFOC_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Missed frames by the controller"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for missed frame counter"]
    #[inline(always)]
    pub fn omfc(&self) -> OMFC_R {
        OMFC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - Missed frames by the application"]
    #[inline(always)]
    pub fn mfa(&self) -> MFA_R {
        MFA_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO overflow counter"]
    #[inline(always)]
    pub fn ofoc(&self) -> OFOC_R {
        OFOC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Ethernet DMA missed frame and buffer overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamfbocr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMFBOCRrs;
impl crate::RegisterSpec for DMAMFBOCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamfbocr::R`](R) reader structure"]
impl crate::Readable for DMAMFBOCRrs {}
#[doc = "`reset()` method sets DMAMFBOCR to value 0"]
impl crate::Resettable for DMAMFBOCRrs {
    const RESET_VALUE: u32 = 0;
}
