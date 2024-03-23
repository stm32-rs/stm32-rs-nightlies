#[doc = "Register `FLASH_WRP1AR` reader"]
pub type R = crate::R<FLASH_WRP1ARrs>;
#[doc = "Register `FLASH_WRP1AR` writer"]
pub type W = crate::W<FLASH_WRP1ARrs>;
#[doc = "Field `WRP1A_PSTRT` reader - bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1."]
pub type WRP1A_PSTRT_R = crate::FieldReader;
#[doc = "Field `WRP1A_PSTRT` writer - bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1."]
pub type WRP1A_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRP1A_PEND` reader - Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1."]
pub type WRP1A_PEND_R = crate::FieldReader;
#[doc = "Field `WRP1A_PEND` writer - Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1."]
pub type WRP1A_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UNLOCK` reader - Bank 1 WPR first area A unlock"]
pub type UNLOCK_R = crate::BitReader;
#[doc = "Field `UNLOCK` writer - Bank 1 WPR first area A unlock"]
pub type UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1."]
    #[inline(always)]
    pub fn wrp1a_pstrt(&self) -> WRP1A_PSTRT_R {
        WRP1A_PSTRT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1."]
    #[inline(always)]
    pub fn wrp1a_pend(&self) -> WRP1A_PEND_R {
        WRP1A_PEND_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Bank 1 WPR first area A unlock"]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1."]
    #[inline(always)]
    #[must_use]
    pub fn wrp1a_pstrt(&mut self) -> WRP1A_PSTRT_W<FLASH_WRP1ARrs> {
        WRP1A_PSTRT_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1."]
    #[inline(always)]
    #[must_use]
    pub fn wrp1a_pend(&mut self) -> WRP1A_PEND_W<FLASH_WRP1ARrs> {
        WRP1A_PEND_W::new(self, 16)
    }
    #[doc = "Bit 31 - Bank 1 WPR first area A unlock"]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<FLASH_WRP1ARrs> {
        UNLOCK_W::new(self, 31)
    }
}
#[doc = "FLASH WRP1 area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_wrp1ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_wrp1ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_WRP1ARrs;
impl crate::RegisterSpec for FLASH_WRP1ARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_wrp1ar::R`](R) reader structure"]
impl crate::Readable for FLASH_WRP1ARrs {}
#[doc = "`write(|w| ..)` method takes [`flash_wrp1ar::W`](W) writer structure"]
impl crate::Writable for FLASH_WRP1ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_WRP1AR to value 0x0f00_ff00"]
impl crate::Resettable for FLASH_WRP1ARrs {
    const RESET_VALUE: u32 = 0x0f00_ff00;
}
