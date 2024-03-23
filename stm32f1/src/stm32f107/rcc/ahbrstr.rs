#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AHBRSTRrs>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AHBRSTRrs>;
#[doc = "USB OTG FS reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSRST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<OTGFSRST> for bool {
    #[inline(always)]
    fn from(variant: OTGFSRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTGFSRST` reader - USB OTG FS reset"]
pub type OTGFSRST_R = crate::BitReader<OTGFSRST>;
impl OTGFSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OTGFSRST> {
        match self.bits {
            true => Some(OTGFSRST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OTGFSRST::Reset
    }
}
#[doc = "Field `OTGFSRST` writer - USB OTG FS reset"]
pub type OTGFSRST_W<'a, REG> = crate::BitWriter<'a, REG, OTGFSRST>;
impl<'a, REG> OTGFSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSRST::Reset)
    }
}
#[doc = "Field `ETHMACRST` reader - Ethernet MAC reset"]
pub use OTGFSRST_R as ETHMACRST_R;
#[doc = "Field `ETHMACRST` writer - Ethernet MAC reset"]
pub use OTGFSRST_W as ETHMACRST_W;
impl R {
    #[doc = "Bit 12 - USB OTG FS reset"]
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Ethernet MAC reset"]
    #[inline(always)]
    pub fn ethmacrst(&self) -> ETHMACRST_R {
        ETHMACRST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - USB OTG FS reset"]
    #[inline(always)]
    #[must_use]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<AHBRSTRrs> {
        OTGFSRST_W::new(self, 12)
    }
    #[doc = "Bit 14 - Ethernet MAC reset"]
    #[inline(always)]
    #[must_use]
    pub fn ethmacrst(&mut self) -> ETHMACRST_W<AHBRSTRrs> {
        ETHMACRST_W::new(self, 14)
    }
}
#[doc = "AHB peripheral clock reset register (RCC_AHBRSTR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRSTRrs;
impl crate::RegisterSpec for AHBRSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrstr::R`](R) reader structure"]
impl crate::Readable for AHBRSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure"]
impl crate::Writable for AHBRSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AHBRSTRrs {
    const RESET_VALUE: u32 = 0;
}
