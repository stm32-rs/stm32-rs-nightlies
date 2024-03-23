#[doc = "Register `BRR` writer"]
pub type W = crate::W<BRRrs>;
#[doc = "Port x reset pin %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0W {
    #[doc = "0: No action on the corresponding ODx bit"]
    NoAction = 0,
    #[doc = "1: Reset the ODx bit"]
    Reset = 1,
}
impl From<BR0W> for bool {
    #[inline(always)]
    fn from(variant: BR0W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR(0-15)` writer - Port x reset pin %s"]
pub type BR_W<'a, REG> = crate::BitWriter<'a, REG, BR0W>;
impl<'a, REG> BR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(BR0W::NoAction)
    }
    #[doc = "Reset the ODx bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BR0W::Reset)
    }
}
impl W {
    #[doc = "Port x reset pin (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `BR0` field"]
    #[inline(always)]
    #[must_use]
    pub fn br(&mut self, n: u8) -> BR_W<BRRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        BR_W::new(self, n)
    }
    #[doc = "Bit 0 - Port x reset pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x reset pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x reset pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x reset pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x reset pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x reset pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x reset pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x reset pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x reset pin 8"]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x reset pin 9"]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x reset pin 10"]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x reset pin 11"]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x reset pin 12"]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x reset pin 13"]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x reset pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x reset pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 15)
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
