#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BSRRrs>;
#[doc = "Port x set bit y (y= 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS3W {
    #[doc = "1: Sets the corresponding ODRx bit"]
    Set = 1,
}
impl From<BS3W> for bool {
    #[inline(always)]
    fn from(variant: BS3W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS3` writer - Port x set bit y (y= 0..15)"]
pub type BS3_W<'a, REG> = crate::BitWriter<'a, REG, BS3W>;
impl<'a, REG> BS3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sets the corresponding ODRx bit"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(BS3W::Set)
    }
}
#[doc = "Port x reset bit y (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR3W {
    #[doc = "1: Resets the corresponding ODRx bit"]
    Reset = 1,
}
impl From<BR3W> for bool {
    #[inline(always)]
    fn from(variant: BR3W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR3` writer - Port x reset bit y (y = 0..15)"]
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG, BR3W>;
impl<'a, REG> BR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the corresponding ODRx bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BR3W::Reset)
    }
}
impl W {
    #[doc = "Bit 3 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> BS3_W<BSRRrs> {
        BS3_W::new(self, 3)
    }
    #[doc = "Bit 19 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<BSRRrs> {
        BR3_W::new(self, 19)
    }
}
#[doc = "GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSRRrs;
impl crate::RegisterSpec for BSRRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bsrr::W`](W) writer structure"]
impl crate::Writable for BSRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BSRRrs {
    const RESET_VALUE: u32 = 0;
}
