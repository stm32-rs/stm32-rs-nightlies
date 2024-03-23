#[doc = "Register `GPIOK_BSRR` writer"]
pub type W = crate::W<GPIOK_BSRRrs>;
#[doc = "Field `BS0` writer - BS0"]
pub type BS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS1` writer - BS1"]
pub type BS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS2` writer - BS2"]
pub type BS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS3` writer - BS3"]
pub type BS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS4` writer - BS4"]
pub type BS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS5` writer - BS5"]
pub type BS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS6` writer - BS6"]
pub type BS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS7` writer - BS7"]
pub type BS7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS8` writer - BS8"]
pub type BS8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS9` writer - BS9"]
pub type BS9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS10` writer - BS10"]
pub type BS10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS11` writer - BS11"]
pub type BS11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS12` writer - BS12"]
pub type BS12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS13` writer - BS13"]
pub type BS13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS14` writer - BS14"]
pub type BS14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS15` writer - BS15"]
pub type BS15_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - BS0"]
    #[inline(always)]
    #[must_use]
    pub fn bs0(&mut self) -> BS0_W<GPIOK_BSRRrs> {
        BS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - BS1"]
    #[inline(always)]
    #[must_use]
    pub fn bs1(&mut self) -> BS1_W<GPIOK_BSRRrs> {
        BS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - BS2"]
    #[inline(always)]
    #[must_use]
    pub fn bs2(&mut self) -> BS2_W<GPIOK_BSRRrs> {
        BS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - BS3"]
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> BS3_W<GPIOK_BSRRrs> {
        BS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - BS4"]
    #[inline(always)]
    #[must_use]
    pub fn bs4(&mut self) -> BS4_W<GPIOK_BSRRrs> {
        BS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - BS5"]
    #[inline(always)]
    #[must_use]
    pub fn bs5(&mut self) -> BS5_W<GPIOK_BSRRrs> {
        BS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - BS6"]
    #[inline(always)]
    #[must_use]
    pub fn bs6(&mut self) -> BS6_W<GPIOK_BSRRrs> {
        BS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - BS7"]
    #[inline(always)]
    #[must_use]
    pub fn bs7(&mut self) -> BS7_W<GPIOK_BSRRrs> {
        BS7_W::new(self, 7)
    }
    #[doc = "Bit 8 - BS8"]
    #[inline(always)]
    #[must_use]
    pub fn bs8(&mut self) -> BS8_W<GPIOK_BSRRrs> {
        BS8_W::new(self, 8)
    }
    #[doc = "Bit 9 - BS9"]
    #[inline(always)]
    #[must_use]
    pub fn bs9(&mut self) -> BS9_W<GPIOK_BSRRrs> {
        BS9_W::new(self, 9)
    }
    #[doc = "Bit 10 - BS10"]
    #[inline(always)]
    #[must_use]
    pub fn bs10(&mut self) -> BS10_W<GPIOK_BSRRrs> {
        BS10_W::new(self, 10)
    }
    #[doc = "Bit 11 - BS11"]
    #[inline(always)]
    #[must_use]
    pub fn bs11(&mut self) -> BS11_W<GPIOK_BSRRrs> {
        BS11_W::new(self, 11)
    }
    #[doc = "Bit 12 - BS12"]
    #[inline(always)]
    #[must_use]
    pub fn bs12(&mut self) -> BS12_W<GPIOK_BSRRrs> {
        BS12_W::new(self, 12)
    }
    #[doc = "Bit 13 - BS13"]
    #[inline(always)]
    #[must_use]
    pub fn bs13(&mut self) -> BS13_W<GPIOK_BSRRrs> {
        BS13_W::new(self, 13)
    }
    #[doc = "Bit 14 - BS14"]
    #[inline(always)]
    #[must_use]
    pub fn bs14(&mut self) -> BS14_W<GPIOK_BSRRrs> {
        BS14_W::new(self, 14)
    }
    #[doc = "Bit 15 - BS15"]
    #[inline(always)]
    #[must_use]
    pub fn bs15(&mut self) -> BS15_W<GPIOK_BSRRrs> {
        BS15_W::new(self, 15)
    }
    #[doc = "Bit 16 - BR0"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<GPIOK_BSRRrs> {
        BR0_W::new(self, 16)
    }
    #[doc = "Bit 17 - BR1"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<GPIOK_BSRRrs> {
        BR1_W::new(self, 17)
    }
    #[doc = "Bit 18 - BR2"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<GPIOK_BSRRrs> {
        BR2_W::new(self, 18)
    }
    #[doc = "Bit 19 - BR3"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<GPIOK_BSRRrs> {
        BR3_W::new(self, 19)
    }
    #[doc = "Bit 20 - BR4"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<GPIOK_BSRRrs> {
        BR4_W::new(self, 20)
    }
    #[doc = "Bit 21 - BR5"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<GPIOK_BSRRrs> {
        BR5_W::new(self, 21)
    }
    #[doc = "Bit 22 - BR6"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<GPIOK_BSRRrs> {
        BR6_W::new(self, 22)
    }
    #[doc = "Bit 23 - BR7"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<GPIOK_BSRRrs> {
        BR7_W::new(self, 23)
    }
    #[doc = "Bit 24 - BR8"]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<GPIOK_BSRRrs> {
        BR8_W::new(self, 24)
    }
    #[doc = "Bit 25 - BR9"]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR9_W<GPIOK_BSRRrs> {
        BR9_W::new(self, 25)
    }
    #[doc = "Bit 26 - BR10"]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR10_W<GPIOK_BSRRrs> {
        BR10_W::new(self, 26)
    }
    #[doc = "Bit 27 - BR11"]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR11_W<GPIOK_BSRRrs> {
        BR11_W::new(self, 27)
    }
    #[doc = "Bit 28 - BR12"]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR12_W<GPIOK_BSRRrs> {
        BR12_W::new(self, 28)
    }
    #[doc = "Bit 29 - BR13"]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<GPIOK_BSRRrs> {
        BR13_W::new(self, 29)
    }
    #[doc = "Bit 30 - BR14"]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<GPIOK_BSRRrs> {
        BR14_W::new(self, 30)
    }
    #[doc = "Bit 31 - BR15"]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR15_W<GPIOK_BSRRrs> {
        BR15_W::new(self, 31)
    }
}
#[doc = "GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiok_bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOK_BSRRrs;
impl crate::RegisterSpec for GPIOK_BSRRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpiok_bsrr::W`](W) writer structure"]
impl crate::Writable for GPIOK_BSRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOK_BSRR to value 0"]
impl crate::Resettable for GPIOK_BSRRrs {
    const RESET_VALUE: u32 = 0;
}
