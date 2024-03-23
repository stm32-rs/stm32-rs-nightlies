#[doc = "Register `DDRCTRL_RFSHCTL0` reader"]
pub type R = crate::R<DDRCTRL_RFSHCTL0rs>;
#[doc = "Register `DDRCTRL_RFSHCTL0` writer"]
pub type W = crate::W<DDRCTRL_RFSHCTL0rs>;
#[doc = "Field `PER_BANK_REFRESH` reader - PER_BANK_REFRESH"]
pub type PER_BANK_REFRESH_R = crate::BitReader;
#[doc = "Field `PER_BANK_REFRESH` writer - PER_BANK_REFRESH"]
pub type PER_BANK_REFRESH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFRESH_BURST` reader - REFRESH_BURST"]
pub type REFRESH_BURST_R = crate::FieldReader;
#[doc = "Field `REFRESH_BURST` writer - REFRESH_BURST"]
pub type REFRESH_BURST_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REFRESH_TO_X32` reader - REFRESH_TO_X32"]
pub type REFRESH_TO_X32_R = crate::FieldReader;
#[doc = "Field `REFRESH_TO_X32` writer - REFRESH_TO_X32"]
pub type REFRESH_TO_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REFRESH_MARGIN` reader - REFRESH_MARGIN"]
pub type REFRESH_MARGIN_R = crate::FieldReader;
#[doc = "Field `REFRESH_MARGIN` writer - REFRESH_MARGIN"]
pub type REFRESH_MARGIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 2 - PER_BANK_REFRESH"]
    #[inline(always)]
    pub fn per_bank_refresh(&self) -> PER_BANK_REFRESH_R {
        PER_BANK_REFRESH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:8 - REFRESH_BURST"]
    #[inline(always)]
    pub fn refresh_burst(&self) -> REFRESH_BURST_R {
        REFRESH_BURST_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - REFRESH_TO_X32"]
    #[inline(always)]
    pub fn refresh_to_x32(&self) -> REFRESH_TO_X32_R {
        REFRESH_TO_X32_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - REFRESH_MARGIN"]
    #[inline(always)]
    pub fn refresh_margin(&self) -> REFRESH_MARGIN_R {
        REFRESH_MARGIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - PER_BANK_REFRESH"]
    #[inline(always)]
    #[must_use]
    pub fn per_bank_refresh(&mut self) -> PER_BANK_REFRESH_W<DDRCTRL_RFSHCTL0rs> {
        PER_BANK_REFRESH_W::new(self, 2)
    }
    #[doc = "Bits 4:8 - REFRESH_BURST"]
    #[inline(always)]
    #[must_use]
    pub fn refresh_burst(&mut self) -> REFRESH_BURST_W<DDRCTRL_RFSHCTL0rs> {
        REFRESH_BURST_W::new(self, 4)
    }
    #[doc = "Bits 12:16 - REFRESH_TO_X32"]
    #[inline(always)]
    #[must_use]
    pub fn refresh_to_x32(&mut self) -> REFRESH_TO_X32_W<DDRCTRL_RFSHCTL0rs> {
        REFRESH_TO_X32_W::new(self, 12)
    }
    #[doc = "Bits 20:23 - REFRESH_MARGIN"]
    #[inline(always)]
    #[must_use]
    pub fn refresh_margin(&mut self) -> REFRESH_MARGIN_W<DDRCTRL_RFSHCTL0rs> {
        REFRESH_MARGIN_W::new(self, 20)
    }
}
#[doc = "DDRCTRL refresh control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_rfshctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_rfshctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_RFSHCTL0rs;
impl crate::RegisterSpec for DDRCTRL_RFSHCTL0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_rfshctl0::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_RFSHCTL0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_rfshctl0::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_RFSHCTL0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_RFSHCTL0 to value 0x0021_0000"]
impl crate::Resettable for DDRCTRL_RFSHCTL0rs {
    const RESET_VALUE: u32 = 0x0021_0000;
}
