#[doc = "Register `APB1HLPENR` reader"]
pub type R = crate::R<APB1HLPENRrs>;
#[doc = "Register `APB1HLPENR` writer"]
pub type W = crate::W<APB1HLPENRrs>;
#[doc = "UART9 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART9LPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<UART9LPEN> for bool {
    #[inline(always)]
    fn from(variant: UART9LPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART9LPEN` reader - UART9 clock enable during sleep mode Set and reset by software."]
pub type UART9LPEN_R = crate::BitReader<UART9LPEN>;
impl UART9LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UART9LPEN {
        match self.bits {
            false => UART9LPEN::Disabled,
            true => UART9LPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UART9LPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UART9LPEN::Enabled
    }
}
#[doc = "Field `UART9LPEN` writer - UART9 clock enable during sleep mode Set and reset by software."]
pub type UART9LPEN_W<'a, REG> = crate::BitWriter<'a, REG, UART9LPEN>;
impl<'a, REG> UART9LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART9LPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART9LPEN::Enabled)
    }
}
#[doc = "Field `UART12LPEN` reader - UART12 clock enable during sleep mode Set and reset by software."]
pub use UART9LPEN_R as UART12LPEN_R;
#[doc = "Field `DTSLPEN` reader - DTS clock enable during sleep mode Set and reset by software."]
pub use UART9LPEN_R as DTSLPEN_R;
#[doc = "Field `LPTIM2LPEN` reader - LPTIM2 clock enable during sleep mode Set and reset by software."]
pub use UART9LPEN_R as LPTIM2LPEN_R;
#[doc = "Field `FDCAN12LPEN` reader - FDCAN1 and FDCAN2 peripheral clock enable during sleep mode Set and reset by software."]
pub use UART9LPEN_R as FDCAN12LPEN_R;
#[doc = "Field `UCPDLPEN` reader - UCPD clock enable during sleep mode Set and reset by software."]
pub use UART9LPEN_R as UCPDLPEN_R;
#[doc = "Field `UART12LPEN` writer - UART12 clock enable during sleep mode Set and reset by software."]
pub use UART9LPEN_W as UART12LPEN_W;
#[doc = "Field `DTSLPEN` writer - DTS clock enable during sleep mode Set and reset by software."]
pub use UART9LPEN_W as DTSLPEN_W;
#[doc = "Field `LPTIM2LPEN` writer - LPTIM2 clock enable during sleep mode Set and reset by software."]
pub use UART9LPEN_W as LPTIM2LPEN_W;
#[doc = "Field `FDCAN12LPEN` writer - FDCAN1 and FDCAN2 peripheral clock enable during sleep mode Set and reset by software."]
pub use UART9LPEN_W as FDCAN12LPEN_W;
#[doc = "Field `UCPDLPEN` writer - UCPD clock enable during sleep mode Set and reset by software."]
pub use UART9LPEN_W as UCPDLPEN_W;
impl R {
    #[doc = "Bit 0 - UART9 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn uart9lpen(&self) -> UART9LPEN_R {
        UART9LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART12 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn uart12lpen(&self) -> UART12LPEN_R {
        UART12LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
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
    #[doc = "Bit 9 - FDCAN1 and FDCAN2 peripheral clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn fdcan12lpen(&self) -> FDCAN12LPEN_R {
        FDCAN12LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 23 - UCPD clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn ucpdlpen(&self) -> UCPDLPEN_R {
        UCPDLPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART9 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart9lpen(&mut self) -> UART9LPEN_W<APB1HLPENRrs> {
        UART9LPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - UART12 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart12lpen(&mut self) -> UART12LPEN_W<APB1HLPENRrs> {
        UART12LPEN_W::new(self, 1)
    }
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
    #[doc = "Bit 9 - FDCAN1 and FDCAN2 peripheral clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fdcan12lpen(&mut self) -> FDCAN12LPEN_W<APB1HLPENRrs> {
        FDCAN12LPEN_W::new(self, 9)
    }
    #[doc = "Bit 23 - UCPD clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ucpdlpen(&mut self) -> UCPDLPEN_W<APB1HLPENRrs> {
        UCPDLPEN_W::new(self, 23)
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
