#[doc = "Register `TCR` reader"]
pub type R = crate::R<TCRrs>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TCRrs>;
#[doc = "Field `DCYC` reader - Number of dummy cycles"]
pub type DCYC_R = crate::FieldReader;
#[doc = "Field `DCYC` writer - Number of dummy cycles"]
pub type DCYC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Delay hold quarter cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DHQC {
    #[doc = "0: No delay hold"]
    NoDelay = 0,
    #[doc = "1: 1/4 cycle hold"]
    QuarterCycleHold = 1,
}
impl From<DHQC> for bool {
    #[inline(always)]
    fn from(variant: DHQC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DHQC` reader - Delay hold quarter cycle"]
pub type DHQC_R = crate::BitReader<DHQC>;
impl DHQC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DHQC {
        match self.bits {
            false => DHQC::NoDelay,
            true => DHQC::QuarterCycleHold,
        }
    }
    #[doc = "No delay hold"]
    #[inline(always)]
    pub fn is_no_delay(&self) -> bool {
        *self == DHQC::NoDelay
    }
    #[doc = "1/4 cycle hold"]
    #[inline(always)]
    pub fn is_quarter_cycle_hold(&self) -> bool {
        *self == DHQC::QuarterCycleHold
    }
}
#[doc = "Field `DHQC` writer - Delay hold quarter cycle"]
pub type DHQC_W<'a, REG> = crate::BitWriter<'a, REG, DHQC>;
impl<'a, REG> DHQC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No delay hold"]
    #[inline(always)]
    pub fn no_delay(self) -> &'a mut crate::W<REG> {
        self.variant(DHQC::NoDelay)
    }
    #[doc = "1/4 cycle hold"]
    #[inline(always)]
    pub fn quarter_cycle_hold(self) -> &'a mut crate::W<REG> {
        self.variant(DHQC::QuarterCycleHold)
    }
}
#[doc = "Sample shift\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSHIFT {
    #[doc = "0: No shift"]
    NoShift = 0,
    #[doc = "1: 1/2 cycle shift"]
    HalfCycleShift = 1,
}
impl From<SSHIFT> for bool {
    #[inline(always)]
    fn from(variant: SSHIFT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSHIFT` reader - Sample shift"]
pub type SSHIFT_R = crate::BitReader<SSHIFT>;
impl SSHIFT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSHIFT {
        match self.bits {
            false => SSHIFT::NoShift,
            true => SSHIFT::HalfCycleShift,
        }
    }
    #[doc = "No shift"]
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == SSHIFT::NoShift
    }
    #[doc = "1/2 cycle shift"]
    #[inline(always)]
    pub fn is_half_cycle_shift(&self) -> bool {
        *self == SSHIFT::HalfCycleShift
    }
}
#[doc = "Field `SSHIFT` writer - Sample shift"]
pub type SSHIFT_W<'a, REG> = crate::BitWriter<'a, REG, SSHIFT>;
impl<'a, REG> SSHIFT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No shift"]
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut crate::W<REG> {
        self.variant(SSHIFT::NoShift)
    }
    #[doc = "1/2 cycle shift"]
    #[inline(always)]
    pub fn half_cycle_shift(self) -> &'a mut crate::W<REG> {
        self.variant(SSHIFT::HalfCycleShift)
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles"]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 28 - Delay hold quarter cycle"]
    #[inline(always)]
    pub fn dhqc(&self) -> DHQC_R {
        DHQC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Sample shift"]
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of dummy cycles"]
    #[inline(always)]
    #[must_use]
    pub fn dcyc(&mut self) -> DCYC_W<TCRrs> {
        DCYC_W::new(self, 0)
    }
    #[doc = "Bit 28 - Delay hold quarter cycle"]
    #[inline(always)]
    #[must_use]
    pub fn dhqc(&mut self) -> DHQC_W<TCRrs> {
        DHQC_W::new(self, 28)
    }
    #[doc = "Bit 30 - Sample shift"]
    #[inline(always)]
    #[must_use]
    pub fn sshift(&mut self) -> SSHIFT_W<TCRrs> {
        SSHIFT_W::new(self, 30)
    }
}
#[doc = "timing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCRrs;
impl crate::RegisterSpec for TCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TCRrs {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCRrs {
    const RESET_VALUE: u32 = 0;
}
