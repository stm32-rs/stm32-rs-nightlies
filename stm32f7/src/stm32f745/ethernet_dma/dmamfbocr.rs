#[doc = "Register `DMAMFBOCR` reader"]
pub type R = crate::R<DMAMFBOCRrs>;
#[doc = "Register `DMAMFBOCR` writer"]
pub type W = crate::W<DMAMFBOCRrs>;
#[doc = "Field `MFC` reader - Missed frames by the controller"]
pub type MFC_R = crate::FieldReader<u16>;
#[doc = "Field `MFC` writer - Missed frames by the controller"]
pub type MFC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OMFC` reader - Overflow bit for missed frame counter"]
pub type OMFC_R = crate::BitReader;
#[doc = "Field `OMFC` writer - Overflow bit for missed frame counter"]
pub type OMFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFA` reader - Missed frames by the application"]
pub type MFA_R = crate::FieldReader<u16>;
#[doc = "Field `MFA` writer - Missed frames by the application"]
pub type MFA_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `OFOC` reader - Overflow bit for FIFO overflow counter"]
pub type OFOC_R = crate::BitReader;
#[doc = "Field `OFOC` writer - Overflow bit for FIFO overflow counter"]
pub type OFOC_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bits 0:15 - Missed frames by the controller"]
    #[inline(always)]
    #[must_use]
    pub fn mfc(&mut self) -> MFC_W<DMAMFBOCRrs> {
        MFC_W::new(self, 0)
    }
    #[doc = "Bit 16 - Overflow bit for missed frame counter"]
    #[inline(always)]
    #[must_use]
    pub fn omfc(&mut self) -> OMFC_W<DMAMFBOCRrs> {
        OMFC_W::new(self, 16)
    }
    #[doc = "Bits 17:27 - Missed frames by the application"]
    #[inline(always)]
    #[must_use]
    pub fn mfa(&mut self) -> MFA_W<DMAMFBOCRrs> {
        MFA_W::new(self, 17)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO overflow counter"]
    #[inline(always)]
    #[must_use]
    pub fn ofoc(&mut self) -> OFOC_W<DMAMFBOCRrs> {
        OFOC_W::new(self, 28)
    }
}
#[doc = "Ethernet DMA missed frame and buffer overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamfbocr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamfbocr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMFBOCRrs;
impl crate::RegisterSpec for DMAMFBOCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamfbocr::R`](R) reader structure"]
impl crate::Readable for DMAMFBOCRrs {}
#[doc = "`write(|w| ..)` method takes [`dmamfbocr::W`](W) writer structure"]
impl crate::Writable for DMAMFBOCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMFBOCR to value 0"]
impl crate::Resettable for DMAMFBOCRrs {
    const RESET_VALUE: u32 = 0;
}
