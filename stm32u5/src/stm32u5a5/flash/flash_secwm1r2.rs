#[doc = "Register `FLASH_SECWM1R2` reader"]
pub type R = crate::R<FLASH_SECWM1R2rs>;
#[doc = "Register `FLASH_SECWM1R2` writer"]
pub type W = crate::W<FLASH_SECWM1R2rs>;
#[doc = "Field `PCROP1_PSTRT` reader - Start page of first PCROP area This field contains the first page of the PCROP area in bank 1."]
pub type PCROP1_PSTRT_R = crate::FieldReader;
#[doc = "Field `PCROP1_PSTRT` writer - Start page of first PCROP area This field contains the first page of the PCROP area in bank 1."]
pub type PCROP1_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PCROP1EN` reader - PCROP1 area enable"]
pub type PCROP1EN_R = crate::BitReader;
#[doc = "Field `PCROP1EN` writer - PCROP1 area enable"]
pub type PCROP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDP1_PEND` reader - End page of first hide protection area This field contains the last page of the HDP area in bank 1."]
pub type HDP1_PEND_R = crate::FieldReader;
#[doc = "Field `HDP1_PEND` writer - End page of first hide protection area This field contains the last page of the HDP area in bank 1."]
pub type HDP1_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HDP1EN` reader - Hide protection first area enable"]
pub type HDP1EN_R = crate::BitReader;
#[doc = "Field `HDP1EN` writer - Hide protection first area enable"]
pub type HDP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Start page of first PCROP area This field contains the first page of the PCROP area in bank 1."]
    #[inline(always)]
    pub fn pcrop1_pstrt(&self) -> PCROP1_PSTRT_R {
        PCROP1_PSTRT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - PCROP1 area enable"]
    #[inline(always)]
    pub fn pcrop1en(&self) -> PCROP1EN_R {
        PCROP1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - End page of first hide protection area This field contains the last page of the HDP area in bank 1."]
    #[inline(always)]
    pub fn hdp1_pend(&self) -> HDP1_PEND_R {
        HDP1_PEND_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Hide protection first area enable"]
    #[inline(always)]
    pub fn hdp1en(&self) -> HDP1EN_R {
        HDP1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start page of first PCROP area This field contains the first page of the PCROP area in bank 1."]
    #[inline(always)]
    #[must_use]
    pub fn pcrop1_pstrt(&mut self) -> PCROP1_PSTRT_W<FLASH_SECWM1R2rs> {
        PCROP1_PSTRT_W::new(self, 0)
    }
    #[doc = "Bit 15 - PCROP1 area enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop1en(&mut self) -> PCROP1EN_W<FLASH_SECWM1R2rs> {
        PCROP1EN_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - End page of first hide protection area This field contains the last page of the HDP area in bank 1."]
    #[inline(always)]
    #[must_use]
    pub fn hdp1_pend(&mut self) -> HDP1_PEND_W<FLASH_SECWM1R2rs> {
        HDP1_PEND_W::new(self, 16)
    }
    #[doc = "Bit 31 - Hide protection first area enable"]
    #[inline(always)]
    #[must_use]
    pub fn hdp1en(&mut self) -> HDP1EN_W<FLASH_SECWM1R2rs> {
        HDP1EN_W::new(self, 31)
    }
}
#[doc = "FLASH secure watermark1 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_secwm1r2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_secwm1r2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_SECWM1R2rs;
impl crate::RegisterSpec for FLASH_SECWM1R2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_secwm1r2::R`](R) reader structure"]
impl crate::Readable for FLASH_SECWM1R2rs {}
#[doc = "`write(|w| ..)` method takes [`flash_secwm1r2::W`](W) writer structure"]
impl crate::Writable for FLASH_SECWM1R2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_SECWM1R2 to value 0x0f00_0f00"]
impl crate::Resettable for FLASH_SECWM1R2rs {
    const RESET_VALUE: u32 = 0x0f00_0f00;
}
