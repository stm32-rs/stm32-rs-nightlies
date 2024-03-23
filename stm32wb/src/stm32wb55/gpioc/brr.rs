#[doc = "Register `BRR` writer"]
pub type W = crate::W<BRRrs>;
#[doc = "Field `BR0` writer - Port Reset bit"]
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` writer - Port Reset bit"]
pub type BR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR2` writer - Port Reset bit"]
pub type BR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR3` writer - Port Reset bit"]
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR4` writer - Port Reset bit"]
pub type BR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR5` writer - Port Reset bit"]
pub type BR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR6` writer - Port Reset bit"]
pub type BR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR7` writer - Port Reset bit"]
pub type BR7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR8` writer - Port Reset bit"]
pub type BR8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR9` writer - Port Reset bit"]
pub type BR9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR10` writer - Port Reset bit"]
pub type BR10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR11` writer - Port Reset bit"]
pub type BR11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR12` writer - Port Reset bit"]
pub type BR12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR13` writer - Port Reset bit"]
pub type BR13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR14` writer - Port Reset bit"]
pub type BR14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR15` writer - Port Reset bit"]
pub type BR15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<BRRrs> {
        BR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<BRRrs> {
        BR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<BRRrs> {
        BR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<BRRrs> {
        BR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<BRRrs> {
        BR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<BRRrs> {
        BR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<BRRrs> {
        BR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<BRRrs> {
        BR7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<BRRrs> {
        BR8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR9_W<BRRrs> {
        BR9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR10_W<BRRrs> {
        BR10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR11_W<BRRrs> {
        BR11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR12_W<BRRrs> {
        BR12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<BRRrs> {
        BR13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<BRRrs> {
        BR14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR15_W<BRRrs> {
        BR15_W::new(self, 15)
    }
}
#[doc = "port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
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
