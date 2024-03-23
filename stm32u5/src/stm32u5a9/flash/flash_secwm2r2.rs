#[doc = "Register `FLASH_SECWM2R2` reader"]
pub type R = crate::R<FLASH_SECWM2R2rs>;
#[doc = "Register `FLASH_SECWM2R2` writer"]
pub type W = crate::W<FLASH_SECWM2R2rs>;
#[doc = "Field `PCROP2_PSTRT` reader - Start page of PCROP2 area PRCROP2_PSTRT contains the first page of the PCROP area in bank 2."]
pub type PCROP2_PSTRT_R = crate::FieldReader;
#[doc = "Field `PCROP2_PSTRT` writer - Start page of PCROP2 area PRCROP2_PSTRT contains the first page of the PCROP area in bank 2."]
pub type PCROP2_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PCROP2EN` reader - PCROP2 area enable"]
pub type PCROP2EN_R = crate::BitReader;
#[doc = "Field `PCROP2EN` writer - PCROP2 area enable"]
pub type PCROP2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDP2_PEND` reader - End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2."]
pub type HDP2_PEND_R = crate::FieldReader;
#[doc = "Field `HDP2_PEND` writer - End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2."]
pub type HDP2_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HDP2EN` reader - Hide protection second area enable"]
pub type HDP2EN_R = crate::BitReader;
#[doc = "Field `HDP2EN` writer - Hide protection second area enable"]
pub type HDP2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Start page of PCROP2 area PRCROP2_PSTRT contains the first page of the PCROP area in bank 2."]
    #[inline(always)]
    pub fn pcrop2_pstrt(&self) -> PCROP2_PSTRT_R {
        PCROP2_PSTRT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - PCROP2 area enable"]
    #[inline(always)]
    pub fn pcrop2en(&self) -> PCROP2EN_R {
        PCROP2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2."]
    #[inline(always)]
    pub fn hdp2_pend(&self) -> HDP2_PEND_R {
        HDP2_PEND_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Hide protection second area enable"]
    #[inline(always)]
    pub fn hdp2en(&self) -> HDP2EN_R {
        HDP2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start page of PCROP2 area PRCROP2_PSTRT contains the first page of the PCROP area in bank 2."]
    #[inline(always)]
    #[must_use]
    pub fn pcrop2_pstrt(&mut self) -> PCROP2_PSTRT_W<FLASH_SECWM2R2rs> {
        PCROP2_PSTRT_W::new(self, 0)
    }
    #[doc = "Bit 15 - PCROP2 area enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop2en(&mut self) -> PCROP2EN_W<FLASH_SECWM2R2rs> {
        PCROP2EN_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2."]
    #[inline(always)]
    #[must_use]
    pub fn hdp2_pend(&mut self) -> HDP2_PEND_W<FLASH_SECWM2R2rs> {
        HDP2_PEND_W::new(self, 16)
    }
    #[doc = "Bit 31 - Hide protection second area enable"]
    #[inline(always)]
    #[must_use]
    pub fn hdp2en(&mut self) -> HDP2EN_W<FLASH_SECWM2R2rs> {
        HDP2EN_W::new(self, 31)
    }
}
#[doc = "FLASH secure watermark2 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_secwm2r2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_secwm2r2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_SECWM2R2rs;
impl crate::RegisterSpec for FLASH_SECWM2R2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_secwm2r2::R`](R) reader structure"]
impl crate::Readable for FLASH_SECWM2R2rs {}
#[doc = "`write(|w| ..)` method takes [`flash_secwm2r2::W`](W) writer structure"]
impl crate::Writable for FLASH_SECWM2R2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_SECWM2R2 to value 0x0f00_0f00"]
impl crate::Resettable for FLASH_SECWM2R2rs {
    const RESET_VALUE: u32 = 0x0f00_0f00;
}
