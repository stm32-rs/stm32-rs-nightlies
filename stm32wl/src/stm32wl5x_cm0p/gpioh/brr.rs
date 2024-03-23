#[doc = "Register `BRR` reader"]
pub type R = crate::R<BRRrs>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BRRrs>;
#[doc = "Port Reset bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR3W {
    #[doc = "0: No action on the corresponding ODx bit"]
    NoAction = 0,
    #[doc = "1: Reset the ODx bit"]
    Reset = 1,
}
impl From<BR3W> for bool {
    #[inline(always)]
    fn from(variant: BR3W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR3` reader - Port Reset bit"]
pub type BR3_R = crate::BitReader<BR3W>;
impl BR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BR3W {
        match self.bits {
            false => BR3W::NoAction,
            true => BR3W::Reset,
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == BR3W::NoAction
    }
    #[doc = "Reset the ODx bit"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BR3W::Reset
    }
}
#[doc = "Field `BR3` writer - Port Reset bit"]
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG, BR3W>;
impl<'a, REG> BR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(BR3W::NoAction)
    }
    #[doc = "Reset the ODx bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BR3W::Reset)
    }
}
impl R {
    #[doc = "Bit 3 - Port Reset bit"]
    #[inline(always)]
    pub fn br3(&self) -> BR3_R {
        BR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<BRRrs> {
        BR3_W::new(self, 3)
    }
}
#[doc = "GPIO port bit reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BRRrs {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRRrs {
    const RESET_VALUE: u32 = 0;
}
