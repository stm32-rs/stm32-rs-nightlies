#[doc = "Register `FLASH_SECWM1R1` reader"]
pub type R = crate::R<FLASH_SECWM1R1rs>;
#[doc = "Register `FLASH_SECWM1R1` writer"]
pub type W = crate::W<FLASH_SECWM1R1rs>;
#[doc = "Field `SECWM1_PSTRT` reader - Start page of first secure area This field contains the first page of the secure area in bank 1."]
pub type SECWM1_PSTRT_R = crate::FieldReader;
#[doc = "Field `SECWM1_PSTRT` writer - Start page of first secure area This field contains the first page of the secure area in bank 1."]
pub type SECWM1_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SECWM1_PEND` reader - End page of first secure area This field contains the last page of the secure area in bank 1."]
pub type SECWM1_PEND_R = crate::FieldReader;
#[doc = "Field `SECWM1_PEND` writer - End page of first secure area This field contains the last page of the secure area in bank 1."]
pub type SECWM1_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Start page of first secure area This field contains the first page of the secure area in bank 1."]
    #[inline(always)]
    pub fn secwm1_pstrt(&self) -> SECWM1_PSTRT_R {
        SECWM1_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - End page of first secure area This field contains the last page of the secure area in bank 1."]
    #[inline(always)]
    pub fn secwm1_pend(&self) -> SECWM1_PEND_R {
        SECWM1_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Start page of first secure area This field contains the first page of the secure area in bank 1."]
    #[inline(always)]
    #[must_use]
    pub fn secwm1_pstrt(&mut self) -> SECWM1_PSTRT_W<FLASH_SECWM1R1rs> {
        SECWM1_PSTRT_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - End page of first secure area This field contains the last page of the secure area in bank 1."]
    #[inline(always)]
    #[must_use]
    pub fn secwm1_pend(&mut self) -> SECWM1_PEND_W<FLASH_SECWM1R1rs> {
        SECWM1_PEND_W::new(self, 16)
    }
}
#[doc = "FLASH secure watermark1 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_secwm1r1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_secwm1r1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_SECWM1R1rs;
impl crate::RegisterSpec for FLASH_SECWM1R1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_secwm1r1::R`](R) reader structure"]
impl crate::Readable for FLASH_SECWM1R1rs {}
#[doc = "`write(|w| ..)` method takes [`flash_secwm1r1::W`](W) writer structure"]
impl crate::Writable for FLASH_SECWM1R1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_SECWM1R1 to value 0xff00_ff00"]
impl crate::Resettable for FLASH_SECWM1R1rs {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
