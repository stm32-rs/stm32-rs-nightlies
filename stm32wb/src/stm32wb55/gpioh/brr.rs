#[doc = "Register `BRR` writer"]
pub type W = crate::W<BRRrs>;
#[doc = "Field `BR0` writer - Port Reset bit"]
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` writer - Port Reset bit"]
pub type BR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR3` writer - Port Reset bit"]
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - Port Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<BRRrs> {
        BR3_W::new(self, 3)
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
