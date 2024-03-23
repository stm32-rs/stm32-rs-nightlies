#[doc = "Register `FLASH_WRP2BR` reader"]
pub type R = crate::R<FLASH_WRP2BRrs>;
#[doc = "Register `FLASH_WRP2BR` writer"]
pub type W = crate::W<FLASH_WRP2BRrs>;
#[doc = "Field `WRP2B_PSTRT` reader - Bank 2 WPR second area B start page This field contains the first page of the second WRP area for bank 2."]
pub type WRP2B_PSTRT_R = crate::FieldReader;
#[doc = "Field `WRP2B_PSTRT` writer - Bank 2 WPR second area B start page This field contains the first page of the second WRP area for bank 2."]
pub type WRP2B_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRP2B_PEND` reader - Bank 2 WPR second area B end page This field contains the last page of the second WRP area in bank 2."]
pub type WRP2B_PEND_R = crate::FieldReader;
#[doc = "Field `WRP2B_PEND` writer - Bank 2 WPR second area B end page This field contains the last page of the second WRP area in bank 2."]
pub type WRP2B_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `UNLOCK` reader - Bank 2 WPR second area B unlock"]
pub type UNLOCK_R = crate::BitReader;
#[doc = "Field `UNLOCK` writer - Bank 2 WPR second area B unlock"]
pub type UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Bank 2 WPR second area B start page This field contains the first page of the second WRP area for bank 2."]
    #[inline(always)]
    pub fn wrp2b_pstrt(&self) -> WRP2B_PSTRT_R {
        WRP2B_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank 2 WPR second area B end page This field contains the last page of the second WRP area in bank 2."]
    #[inline(always)]
    pub fn wrp2b_pend(&self) -> WRP2B_PEND_R {
        WRP2B_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Bank 2 WPR second area B unlock"]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Bank 2 WPR second area B start page This field contains the first page of the second WRP area for bank 2."]
    #[inline(always)]
    #[must_use]
    pub fn wrp2b_pstrt(&mut self) -> WRP2B_PSTRT_W<FLASH_WRP2BRrs> {
        WRP2B_PSTRT_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - Bank 2 WPR second area B end page This field contains the last page of the second WRP area in bank 2."]
    #[inline(always)]
    #[must_use]
    pub fn wrp2b_pend(&mut self) -> WRP2B_PEND_W<FLASH_WRP2BRrs> {
        WRP2B_PEND_W::new(self, 16)
    }
    #[doc = "Bit 31 - Bank 2 WPR second area B unlock"]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<FLASH_WRP2BRrs> {
        UNLOCK_W::new(self, 31)
    }
}
#[doc = "FLASH WPR2 area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_wrp2br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_wrp2br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_WRP2BRrs;
impl crate::RegisterSpec for FLASH_WRP2BRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_wrp2br::R`](R) reader structure"]
impl crate::Readable for FLASH_WRP2BRrs {}
#[doc = "`write(|w| ..)` method takes [`flash_wrp2br::W`](W) writer structure"]
impl crate::Writable for FLASH_WRP2BRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_WRP2BR to value 0x0f00_ff00"]
impl crate::Resettable for FLASH_WRP2BRrs {
    const RESET_VALUE: u32 = 0x0f00_ff00;
}
