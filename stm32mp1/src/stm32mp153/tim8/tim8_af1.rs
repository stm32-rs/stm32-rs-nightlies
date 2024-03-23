#[doc = "Register `TIM8_AF1` reader"]
pub type R = crate::R<TIM8_AF1rs>;
#[doc = "Register `TIM8_AF1` writer"]
pub type W = crate::W<TIM8_AF1rs>;
#[doc = "Field `BKINE` reader - BKINE"]
pub type BKINE_R = crate::BitReader;
#[doc = "Field `BKINE` writer - BKINE"]
pub type BKINE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKDF1BK2E` reader - BKDF1BK2E"]
pub type BKDF1BK2E_R = crate::BitReader;
#[doc = "Field `BKDF1BK2E` writer - BKDF1BK2E"]
pub type BKDF1BK2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKINP` reader - BKINP"]
pub type BKINP_R = crate::BitReader;
#[doc = "Field `BKINP` writer - BKINP"]
pub type BKINP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETRSEL` reader - ETRSEL"]
pub type ETRSEL_R = crate::FieldReader;
#[doc = "Field `ETRSEL` writer - ETRSEL"]
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - BKINE"]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - BKDF1BK2E"]
    #[inline(always)]
    pub fn bkdf1bk2e(&self) -> BKDF1BK2E_R {
        BKDF1BK2E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BKINP"]
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 14:17 - ETRSEL"]
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BKINE"]
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BKINE_W<TIM8_AF1rs> {
        BKINE_W::new(self, 0)
    }
    #[doc = "Bit 8 - BKDF1BK2E"]
    #[inline(always)]
    #[must_use]
    pub fn bkdf1bk2e(&mut self) -> BKDF1BK2E_W<TIM8_AF1rs> {
        BKDF1BK2E_W::new(self, 8)
    }
    #[doc = "Bit 9 - BKINP"]
    #[inline(always)]
    #[must_use]
    pub fn bkinp(&mut self) -> BKINP_W<TIM8_AF1rs> {
        BKINP_W::new(self, 9)
    }
    #[doc = "Bits 14:17 - ETRSEL"]
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> ETRSEL_W<TIM8_AF1rs> {
        ETRSEL_W::new(self, 14)
    }
}
#[doc = "TIM8 Alternate function option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim8_af1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim8_af1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM8_AF1rs;
impl crate::RegisterSpec for TIM8_AF1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim8_af1::R`](R) reader structure"]
impl crate::Readable for TIM8_AF1rs {}
#[doc = "`write(|w| ..)` method takes [`tim8_af1::W`](W) writer structure"]
impl crate::Writable for TIM8_AF1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM8_AF1 to value 0x01"]
impl crate::Resettable for TIM8_AF1rs {
    const RESET_VALUE: u32 = 0x01;
}
