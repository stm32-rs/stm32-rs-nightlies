#[doc = "Register `GPIOI_BRR` writer"]
pub type W = crate::W<GPIOI_BRRrs>;
#[doc = "Field `BR0` writer - BR0"]
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` writer - BR1"]
pub type BR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR2` writer - BR2"]
pub type BR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR3` writer - BR3"]
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR4` writer - BR4"]
pub type BR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR5` writer - BR5"]
pub type BR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR6` writer - BR6"]
pub type BR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR7` writer - BR7"]
pub type BR7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR8` writer - BR8"]
pub type BR8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR9` writer - BR9"]
pub type BR9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR10` writer - BR10"]
pub type BR10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR11` writer - BR11"]
pub type BR11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR12` writer - BR12"]
pub type BR12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR13` writer - BR13"]
pub type BR13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR14` writer - BR14"]
pub type BR14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR15` writer - BR15"]
pub type BR15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - BR0"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<GPIOI_BRRrs> {
        BR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - BR1"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<GPIOI_BRRrs> {
        BR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - BR2"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<GPIOI_BRRrs> {
        BR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - BR3"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<GPIOI_BRRrs> {
        BR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - BR4"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<GPIOI_BRRrs> {
        BR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - BR5"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<GPIOI_BRRrs> {
        BR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - BR6"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<GPIOI_BRRrs> {
        BR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - BR7"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<GPIOI_BRRrs> {
        BR7_W::new(self, 7)
    }
    #[doc = "Bit 8 - BR8"]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<GPIOI_BRRrs> {
        BR8_W::new(self, 8)
    }
    #[doc = "Bit 9 - BR9"]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR9_W<GPIOI_BRRrs> {
        BR9_W::new(self, 9)
    }
    #[doc = "Bit 10 - BR10"]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR10_W<GPIOI_BRRrs> {
        BR10_W::new(self, 10)
    }
    #[doc = "Bit 11 - BR11"]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR11_W<GPIOI_BRRrs> {
        BR11_W::new(self, 11)
    }
    #[doc = "Bit 12 - BR12"]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR12_W<GPIOI_BRRrs> {
        BR12_W::new(self, 12)
    }
    #[doc = "Bit 13 - BR13"]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<GPIOI_BRRrs> {
        BR13_W::new(self, 13)
    }
    #[doc = "Bit 14 - BR14"]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<GPIOI_BRRrs> {
        BR14_W::new(self, 14)
    }
    #[doc = "Bit 15 - BR15"]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR15_W<GPIOI_BRRrs> {
        BR15_W::new(self, 15)
    }
}
#[doc = "GPIO port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioi_brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOI_BRRrs;
impl crate::RegisterSpec for GPIOI_BRRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpioi_brr::W`](W) writer structure"]
impl crate::Writable for GPIOI_BRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOI_BRR to value 0"]
impl crate::Resettable for GPIOI_BRRrs {
    const RESET_VALUE: u32 = 0;
}
