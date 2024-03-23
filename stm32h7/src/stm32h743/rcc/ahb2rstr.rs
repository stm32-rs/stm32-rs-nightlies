#[doc = "Register `AHB2RSTR` reader"]
pub type R = crate::R<AHB2RSTRrs>;
#[doc = "Register `AHB2RSTR` writer"]
pub type W = crate::W<AHB2RSTRrs>;
#[doc = "CAMITF block reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAMITFRST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<CAMITFRST> for bool {
    #[inline(always)]
    fn from(variant: CAMITFRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAMITFRST` reader - CAMITF block reset"]
pub type CAMITFRST_R = crate::BitReader<CAMITFRST>;
impl CAMITFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CAMITFRST> {
        match self.bits {
            true => Some(CAMITFRST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CAMITFRST::Reset
    }
}
#[doc = "Field `CAMITFRST` writer - CAMITF block reset"]
pub type CAMITFRST_W<'a, REG> = crate::BitWriter<'a, REG, CAMITFRST>;
impl<'a, REG> CAMITFRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CAMITFRST::Reset)
    }
}
#[doc = "Field `CRYPTRST` reader - Cryptography block reset"]
pub use CAMITFRST_R as CRYPTRST_R;
#[doc = "Field `HASHRST` reader - Hash block reset"]
pub use CAMITFRST_R as HASHRST_R;
#[doc = "Field `RNGRST` reader - Random Number Generator block reset"]
pub use CAMITFRST_R as RNGRST_R;
#[doc = "Field `SDMMC2RST` reader - SDMMC2 and SDMMC2 Delay block reset"]
pub use CAMITFRST_R as SDMMC2RST_R;
#[doc = "Field `CRYPTRST` writer - Cryptography block reset"]
pub use CAMITFRST_W as CRYPTRST_W;
#[doc = "Field `HASHRST` writer - Hash block reset"]
pub use CAMITFRST_W as HASHRST_W;
#[doc = "Field `RNGRST` writer - Random Number Generator block reset"]
pub use CAMITFRST_W as RNGRST_W;
#[doc = "Field `SDMMC2RST` writer - SDMMC2 and SDMMC2 Delay block reset"]
pub use CAMITFRST_W as SDMMC2RST_W;
impl R {
    #[doc = "Bit 0 - CAMITF block reset"]
    #[inline(always)]
    pub fn camitfrst(&self) -> CAMITFRST_R {
        CAMITFRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Cryptography block reset"]
    #[inline(always)]
    pub fn cryptrst(&self) -> CRYPTRST_R {
        CRYPTRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hash block reset"]
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Random Number Generator block reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay block reset"]
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAMITF block reset"]
    #[inline(always)]
    #[must_use]
    pub fn camitfrst(&mut self) -> CAMITFRST_W<AHB2RSTRrs> {
        CAMITFRST_W::new(self, 0)
    }
    #[doc = "Bit 4 - Cryptography block reset"]
    #[inline(always)]
    #[must_use]
    pub fn cryptrst(&mut self) -> CRYPTRST_W<AHB2RSTRrs> {
        CRYPTRST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Hash block reset"]
    #[inline(always)]
    #[must_use]
    pub fn hashrst(&mut self) -> HASHRST_W<AHB2RSTRrs> {
        HASHRST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Random Number Generator block reset"]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<AHB2RSTRrs> {
        RNGRST_W::new(self, 6)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay block reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<AHB2RSTRrs> {
        SDMMC2RST_W::new(self, 9)
    }
}
#[doc = "RCC AHB2 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2RSTRrs;
impl crate::RegisterSpec for AHB2RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rstr::R`](R) reader structure"]
impl crate::Readable for AHB2RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure"]
impl crate::Writable for AHB2RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for AHB2RSTRrs {
    const RESET_VALUE: u32 = 0;
}
