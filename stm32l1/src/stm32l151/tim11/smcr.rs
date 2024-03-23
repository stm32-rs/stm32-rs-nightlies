#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SMCRrs>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SMCRrs>;
#[doc = "Field `ETF` reader - External trigger filter"]
pub type ETF_R = crate::FieldReader;
#[doc = "Field `ETF` writer - External trigger filter"]
pub type ETF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETPS` reader - External trigger prescaler"]
pub type ETPS_R = crate::FieldReader;
#[doc = "Field `ETPS` writer - External trigger prescaler"]
pub type ETPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECE` reader - External clock enable"]
pub type ECE_R = crate::BitReader;
#[doc = "Field `ECE` writer - External clock enable"]
pub type ECE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETP` reader - External trigger polarity"]
pub type ETP_R = crate::BitReader;
#[doc = "Field `ETP` writer - External trigger polarity"]
pub type ETP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<SMCRrs> {
        ETF_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn etps(&mut self) -> ETPS_W<SMCRrs> {
        ETPS_W::new(self, 12)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ece(&mut self) -> ECE_W<SMCRrs> {
        ECE_W::new(self, 14)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<SMCRrs> {
        ETP_W::new(self, 15)
    }
}
#[doc = "TIM 11 slave mode control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SMCRrs {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SMCRrs {
    const RESET_VALUE: u32 = 0;
}
