#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BSRRrs>;
#[doc = "Field `BS0` writer - Port x set bit y (y= 0..15)"]
pub type BS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS1` writer - Port x set bit y (y= 0..15)"]
pub type BS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS3` writer - Port x set bit y (y= 0..15)"]
pub type BS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR0` writer - Port x set bit y (y= 0..15)"]
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` writer - Port x reset bit y (y = 0..15)"]
pub type BR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR3` writer - Port x reset bit y (y = 0..15)"]
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs0(&mut self) -> BS0_W<BSRRrs> {
        BS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs1(&mut self) -> BS1_W<BSRRrs> {
        BS1_W::new(self, 1)
    }
    #[doc = "Bit 3 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> BS3_W<BSRRrs> {
        BS3_W::new(self, 3)
    }
    #[doc = "Bit 16 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<BSRRrs> {
        BR0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<BSRRrs> {
        BR1_W::new(self, 17)
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
