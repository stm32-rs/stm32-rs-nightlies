#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BSRRrs>;
#[doc = "Port x set bit y (y= 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS0W {
    #[doc = "1: Sets the corresponding ODRx bit"]
    Set = 1,
}
impl From<BS0W> for bool {
    #[inline(always)]
    fn from(variant: BS0W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS0` writer - Port x set bit y (y= 0..15)"]
pub type BS0_W<'a, REG> = crate::BitWriter<'a, REG, BS0W>;
impl<'a, REG> BS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sets the corresponding ODRx bit"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(BS0W::Set)
    }
}
#[doc = "Field `BS1` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS1_W;
#[doc = "Field `BS2` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS2_W;
#[doc = "Field `BS3` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS3_W;
#[doc = "Field `BS4` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS4_W;
#[doc = "Field `BS5` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS5_W;
#[doc = "Field `BS6` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS6_W;
#[doc = "Field `BS7` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS7_W;
#[doc = "Field `BS8` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS8_W;
#[doc = "Field `BS9` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS9_W;
#[doc = "Field `BS10` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS10_W;
#[doc = "Field `BS11` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS11_W;
#[doc = "Field `BS12` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS12_W;
#[doc = "Field `BS13` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS13_W;
#[doc = "Field `BS14` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS14_W;
#[doc = "Field `BS15` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS15_W;
#[doc = "Port x set bit y (y= 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0W {
    #[doc = "1: Resets the corresponding ODRx bit"]
    Reset = 1,
}
impl From<BR0W> for bool {
    #[inline(always)]
    fn from(variant: BR0W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR0` writer - Port x set bit y (y= 0..15)"]
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG, BR0W>;
impl<'a, REG> BR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the corresponding ODRx bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BR0W::Reset)
    }
}
#[doc = "Field `BR1` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR1_W;
#[doc = "Field `BR2` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR2_W;
#[doc = "Field `BR3` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR3_W;
#[doc = "Field `BR4` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR4_W;
#[doc = "Field `BR5` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR5_W;
#[doc = "Field `BR6` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR6_W;
#[doc = "Field `BR7` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR7_W;
#[doc = "Field `BR8` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR8_W;
#[doc = "Field `BR9` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR9_W;
#[doc = "Field `BR10` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR10_W;
#[doc = "Field `BR11` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR11_W;
#[doc = "Field `BR12` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR12_W;
#[doc = "Field `BR13` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR13_W;
#[doc = "Field `BR14` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR14_W;
#[doc = "Field `BR15` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR15_W;
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
    #[doc = "Bit 2 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs2(&mut self) -> BS2_W<BSRRrs> {
        BS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> BS3_W<BSRRrs> {
        BS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs4(&mut self) -> BS4_W<BSRRrs> {
        BS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs5(&mut self) -> BS5_W<BSRRrs> {
        BS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs6(&mut self) -> BS6_W<BSRRrs> {
        BS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs7(&mut self) -> BS7_W<BSRRrs> {
        BS7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs8(&mut self) -> BS8_W<BSRRrs> {
        BS8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs9(&mut self) -> BS9_W<BSRRrs> {
        BS9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs10(&mut self) -> BS10_W<BSRRrs> {
        BS10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs11(&mut self) -> BS11_W<BSRRrs> {
        BS11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs12(&mut self) -> BS12_W<BSRRrs> {
        BS12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs13(&mut self) -> BS13_W<BSRRrs> {
        BS13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs14(&mut self) -> BS14_W<BSRRrs> {
        BS14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs15(&mut self) -> BS15_W<BSRRrs> {
        BS15_W::new(self, 15)
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
    #[doc = "Bit 18 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<BSRRrs> {
        BR2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<BSRRrs> {
        BR3_W::new(self, 19)
    }
    #[doc = "Bit 20 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<BSRRrs> {
        BR4_W::new(self, 20)
    }
    #[doc = "Bit 21 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<BSRRrs> {
        BR5_W::new(self, 21)
    }
    #[doc = "Bit 22 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<BSRRrs> {
        BR6_W::new(self, 22)
    }
    #[doc = "Bit 23 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<BSRRrs> {
        BR7_W::new(self, 23)
    }
    #[doc = "Bit 24 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<BSRRrs> {
        BR8_W::new(self, 24)
    }
    #[doc = "Bit 25 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR9_W<BSRRrs> {
        BR9_W::new(self, 25)
    }
    #[doc = "Bit 26 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR10_W<BSRRrs> {
        BR10_W::new(self, 26)
    }
    #[doc = "Bit 27 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR11_W<BSRRrs> {
        BR11_W::new(self, 27)
    }
    #[doc = "Bit 28 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR12_W<BSRRrs> {
        BR12_W::new(self, 28)
    }
    #[doc = "Bit 29 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<BSRRrs> {
        BR13_W::new(self, 29)
    }
    #[doc = "Bit 30 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<BSRRrs> {
        BR14_W::new(self, 30)
    }
    #[doc = "Bit 31 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR15_W<BSRRrs> {
        BR15_W::new(self, 31)
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
