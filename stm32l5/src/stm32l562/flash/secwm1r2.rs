#[doc = "Register `SECWM1R2` reader"]
pub type R = crate::R<SECWM1R2rs>;
#[doc = "Register `SECWM1R2` writer"]
pub type W = crate::W<SECWM1R2rs>;
#[doc = "Field `PCROP1_PSTRT` reader - PCROP1_PSTRT"]
pub type PCROP1_PSTRT_R = crate::FieldReader;
#[doc = "Field `PCROP1_PSTRT` writer - PCROP1_PSTRT"]
pub type PCROP1_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PCROP1EN` reader - PCROP1EN"]
pub type PCROP1EN_R = crate::BitReader;
#[doc = "Field `PCROP1EN` writer - PCROP1EN"]
pub type PCROP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDP1_PEND` reader - HDP1_PEND"]
pub type HDP1_PEND_R = crate::FieldReader;
#[doc = "Field `HDP1_PEND` writer - HDP1_PEND"]
pub type HDP1_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HDP1EN` reader - HDP1EN"]
pub type HDP1EN_R = crate::BitReader;
#[doc = "Field `HDP1EN` writer - HDP1EN"]
pub type HDP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - PCROP1_PSTRT"]
    #[inline(always)]
    pub fn pcrop1_pstrt(&self) -> PCROP1_PSTRT_R {
        PCROP1_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 15 - PCROP1EN"]
    #[inline(always)]
    pub fn pcrop1en(&self) -> PCROP1EN_R {
        PCROP1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - HDP1_PEND"]
    #[inline(always)]
    pub fn hdp1_pend(&self) -> HDP1_PEND_R {
        HDP1_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - HDP1EN"]
    #[inline(always)]
    pub fn hdp1en(&self) -> HDP1EN_R {
        HDP1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - PCROP1_PSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop1_pstrt(&mut self) -> PCROP1_PSTRT_W<SECWM1R2rs> {
        PCROP1_PSTRT_W::new(self, 0)
    }
    #[doc = "Bit 15 - PCROP1EN"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop1en(&mut self) -> PCROP1EN_W<SECWM1R2rs> {
        PCROP1EN_W::new(self, 15)
    }
    #[doc = "Bits 16:22 - HDP1_PEND"]
    #[inline(always)]
    #[must_use]
    pub fn hdp1_pend(&mut self) -> HDP1_PEND_W<SECWM1R2rs> {
        HDP1_PEND_W::new(self, 16)
    }
    #[doc = "Bit 31 - HDP1EN"]
    #[inline(always)]
    #[must_use]
    pub fn hdp1en(&mut self) -> HDP1EN_W<SECWM1R2rs> {
        HDP1EN_W::new(self, 31)
    }
}
#[doc = "Flash secure watermak1 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secwm1r2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secwm1r2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECWM1R2rs;
impl crate::RegisterSpec for SECWM1R2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secwm1r2::R`](R) reader structure"]
impl crate::Readable for SECWM1R2rs {}
#[doc = "`write(|w| ..)` method takes [`secwm1r2::W`](W) writer structure"]
impl crate::Writable for SECWM1R2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECWM1R2 to value 0x0f00_0f00"]
impl crate::Resettable for SECWM1R2rs {
    const RESET_VALUE: u32 = 0x0f00_0f00;
}
