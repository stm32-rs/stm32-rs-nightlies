#[doc = "Register `APB1HLPENR` reader"]
pub type R = crate::R<APB1HLPENRrs>;
#[doc = "Register `APB1HLPENR` writer"]
pub type W = crate::W<APB1HLPENRrs>;
#[doc = "DTS clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTSLPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<DTSLPEN> for bool {
    #[inline(always)]
    fn from(variant: DTSLPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTSLPEN` reader - DTS clock enable during sleep mode Set and reset by software."]
pub type DTSLPEN_R = crate::BitReader<DTSLPEN>;
impl DTSLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTSLPEN {
        match self.bits {
            false => DTSLPEN::Disabled,
            true => DTSLPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTSLPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTSLPEN::Enabled
    }
}
#[doc = "Field `DTSLPEN` writer - DTS clock enable during sleep mode Set and reset by software."]
pub type DTSLPEN_W<'a, REG> = crate::BitWriter<'a, REG, DTSLPEN>;
impl<'a, REG> DTSLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTSLPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTSLPEN::Enabled)
    }
}
#[doc = "Field `LPTIM2LPEN` reader - LPTIM2 clock enable during sleep mode Set and reset by software."]
pub use DTSLPEN_R as LPTIM2LPEN_R;
#[doc = "Field `FDCANLPEN` reader - FDCAN peripheral clock enable during sleep mode"]
pub use DTSLPEN_R as FDCANLPEN_R;
#[doc = "Field `LPTIM2LPEN` writer - LPTIM2 clock enable during sleep mode Set and reset by software."]
pub use DTSLPEN_W as LPTIM2LPEN_W;
#[doc = "Field `FDCANLPEN` writer - FDCAN peripheral clock enable during sleep mode"]
pub use DTSLPEN_W as FDCANLPEN_W;
impl R {
    #[doc = "Bit 3 - DTS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dtslpen(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN peripheral clock enable during sleep mode"]
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - DTS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dtslpen(&mut self) -> DTSLPEN_W<APB1HLPENRrs> {
        DTSLPEN_W::new(self, 3)
    }
    #[doc = "Bit 5 - LPTIM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<APB1HLPENRrs> {
        LPTIM2LPEN_W::new(self, 5)
    }
    #[doc = "Bit 9 - FDCAN peripheral clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W<APB1HLPENRrs> {
        FDCANLPEN_W::new(self, 9)
    }
}
#[doc = "RCC APB1 sleep clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hlpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hlpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1HLPENRrs;
impl crate::RegisterSpec for APB1HLPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1hlpenr::R`](R) reader structure"]
impl crate::Readable for APB1HLPENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1hlpenr::W`](W) writer structure"]
impl crate::Writable for APB1HLPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1HLPENR to value 0x4080_022b"]
impl crate::Resettable for APB1HLPENRrs {
    const RESET_VALUE: u32 = 0x4080_022b;
}
