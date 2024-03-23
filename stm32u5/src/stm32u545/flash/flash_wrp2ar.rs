#[doc = "Register `FLASH_WRP2AR` reader"]
pub type R = crate::R<FLASH_WRP2ARrs>;
#[doc = "Register `FLASH_WRP2AR` writer"]
pub type W = crate::W<FLASH_WRP2ARrs>;
#[doc = "Field `WRP2A_PSTRT` reader - Bank 2 WPR first area A start page This field contains the first page of the first WRP area for bank 2."]
pub type WRP2A_PSTRT_R = crate::FieldReader;
#[doc = "Field `WRP2A_PSTRT` writer - Bank 2 WPR first area A start page This field contains the first page of the first WRP area for bank 2."]
pub type WRP2A_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRP2A_PEND` reader - Bank 2 WPR first area A end page This field contains the last page of the first WRP area in bank 2."]
pub type WRP2A_PEND_R = crate::FieldReader;
#[doc = "Field `WRP2A_PEND` writer - Bank 2 WPR first area A end page This field contains the last page of the first WRP area in bank 2."]
pub type WRP2A_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UNLOCK` reader - Bank 2 WPR first area A unlock"]
pub type UNLOCK_R = crate::BitReader;
#[doc = "Field `UNLOCK` writer - Bank 2 WPR first area A unlock"]
pub type UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Bank 2 WPR first area A start page This field contains the first page of the first WRP area for bank 2."]
    #[inline(always)]
    pub fn wrp2a_pstrt(&self) -> WRP2A_PSTRT_R {
        WRP2A_PSTRT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Bank 2 WPR first area A end page This field contains the last page of the first WRP area in bank 2."]
    #[inline(always)]
    pub fn wrp2a_pend(&self) -> WRP2A_PEND_R {
        WRP2A_PEND_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Bank 2 WPR first area A unlock"]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank 2 WPR first area A start page This field contains the first page of the first WRP area for bank 2."]
    #[inline(always)]
    #[must_use]
    pub fn wrp2a_pstrt(&mut self) -> WRP2A_PSTRT_W<FLASH_WRP2ARrs> {
        WRP2A_PSTRT_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Bank 2 WPR first area A end page This field contains the last page of the first WRP area in bank 2."]
    #[inline(always)]
    #[must_use]
    pub fn wrp2a_pend(&mut self) -> WRP2A_PEND_W<FLASH_WRP2ARrs> {
        WRP2A_PEND_W::new(self, 16)
    }
    #[doc = "Bit 31 - Bank 2 WPR first area A unlock"]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<FLASH_WRP2ARrs> {
        UNLOCK_W::new(self, 31)
    }
}
#[doc = "FLASH WPR2 area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_wrp2ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_wrp2ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_WRP2ARrs;
impl crate::RegisterSpec for FLASH_WRP2ARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_wrp2ar::R`](R) reader structure"]
impl crate::Readable for FLASH_WRP2ARrs {}
#[doc = "`write(|w| ..)` method takes [`flash_wrp2ar::W`](W) writer structure"]
impl crate::Writable for FLASH_WRP2ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_WRP2AR to value 0x0f00_ff00"]
impl crate::Resettable for FLASH_WRP2ARrs {
    const RESET_VALUE: u32 = 0x0f00_ff00;
}
