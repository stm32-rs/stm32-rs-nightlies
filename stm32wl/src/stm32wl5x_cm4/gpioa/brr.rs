///Register `BRR` reader
pub type R = crate::R<BRRrs>;
///Register `BRR` writer
pub type W = crate::W<BRRrs>;
/**Port Reset bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0W {
    ///0: No action on the corresponding ODx bit
    NoAction = 0,
    ///1: Reset the ODx bit
    Reset = 1,
}
impl From<BR0W> for bool {
    #[inline(always)]
    fn from(variant: BR0W) -> Self {
        variant as u8 != 0
    }
}
///Field `BR0` reader - Port Reset bit
pub type BR0_R = crate::BitReader<BR0W>;
impl BR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BR0W {
        match self.bits {
            false => BR0W::NoAction,
            true => BR0W::Reset,
        }
    }
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == BR0W::NoAction
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BR0W::Reset
    }
}
///Field `BR0` writer - Port Reset bit
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG, BR0W>;
impl<'a, REG> BR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(BR0W::NoAction)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BR0W::Reset)
    }
}
///Field `BR1` reader - Port Reset bit
pub use BR0_R as BR1_R;
///Field `BR2` reader - Port Reset bit
pub use BR0_R as BR2_R;
///Field `BR3` reader - Port Reset bit
pub use BR0_R as BR3_R;
///Field `BR4` reader - Port Reset bit
pub use BR0_R as BR4_R;
///Field `BR5` reader - Port Reset bit
pub use BR0_R as BR5_R;
///Field `BR6` reader - Port Reset bit
pub use BR0_R as BR6_R;
///Field `BR7` reader - Port Reset bit
pub use BR0_R as BR7_R;
///Field `BR8` reader - Port Reset bit
pub use BR0_R as BR8_R;
///Field `BR9` reader - Port Reset bit
pub use BR0_R as BR9_R;
///Field `BR10` reader - Port Reset bit
pub use BR0_R as BR10_R;
///Field `BR11` reader - Port Reset bit
pub use BR0_R as BR11_R;
///Field `BR12` reader - Port Reset bit
pub use BR0_R as BR12_R;
///Field `BR13` reader - Port Reset bit
pub use BR0_R as BR13_R;
///Field `BR14` reader - Port Reset bit
pub use BR0_R as BR14_R;
///Field `BR15` reader - Port Reset bit
pub use BR0_R as BR15_R;
///Field `BR1` writer - Port Reset bit
pub use BR0_W as BR1_W;
///Field `BR2` writer - Port Reset bit
pub use BR0_W as BR2_W;
///Field `BR3` writer - Port Reset bit
pub use BR0_W as BR3_W;
///Field `BR4` writer - Port Reset bit
pub use BR0_W as BR4_W;
///Field `BR5` writer - Port Reset bit
pub use BR0_W as BR5_W;
///Field `BR6` writer - Port Reset bit
pub use BR0_W as BR6_W;
///Field `BR7` writer - Port Reset bit
pub use BR0_W as BR7_W;
///Field `BR8` writer - Port Reset bit
pub use BR0_W as BR8_W;
///Field `BR9` writer - Port Reset bit
pub use BR0_W as BR9_W;
///Field `BR10` writer - Port Reset bit
pub use BR0_W as BR10_W;
///Field `BR11` writer - Port Reset bit
pub use BR0_W as BR11_W;
///Field `BR12` writer - Port Reset bit
pub use BR0_W as BR12_W;
///Field `BR13` writer - Port Reset bit
pub use BR0_W as BR13_W;
///Field `BR14` writer - Port Reset bit
pub use BR0_W as BR14_W;
///Field `BR15` writer - Port Reset bit
pub use BR0_W as BR15_W;
impl R {
    ///Bit 0 - Port Reset bit
    #[inline(always)]
    pub fn br0(&self) -> BR0_R {
        BR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port Reset bit
    #[inline(always)]
    pub fn br1(&self) -> BR1_R {
        BR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port Reset bit
    #[inline(always)]
    pub fn br2(&self) -> BR2_R {
        BR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port Reset bit
    #[inline(always)]
    pub fn br3(&self) -> BR3_R {
        BR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port Reset bit
    #[inline(always)]
    pub fn br4(&self) -> BR4_R {
        BR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port Reset bit
    #[inline(always)]
    pub fn br5(&self) -> BR5_R {
        BR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port Reset bit
    #[inline(always)]
    pub fn br6(&self) -> BR6_R {
        BR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port Reset bit
    #[inline(always)]
    pub fn br7(&self) -> BR7_R {
        BR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port Reset bit
    #[inline(always)]
    pub fn br8(&self) -> BR8_R {
        BR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port Reset bit
    #[inline(always)]
    pub fn br9(&self) -> BR9_R {
        BR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port Reset bit
    #[inline(always)]
    pub fn br10(&self) -> BR10_R {
        BR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port Reset bit
    #[inline(always)]
    pub fn br11(&self) -> BR11_R {
        BR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port Reset bit
    #[inline(always)]
    pub fn br12(&self) -> BR12_R {
        BR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port Reset bit
    #[inline(always)]
    pub fn br13(&self) -> BR13_R {
        BR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port Reset bit
    #[inline(always)]
    pub fn br14(&self) -> BR14_R {
        BR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port Reset bit
    #[inline(always)]
    pub fn br15(&self) -> BR15_R {
        BR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRR")
            .field("br0", &self.br0())
            .field("br1", &self.br1())
            .field("br2", &self.br2())
            .field("br3", &self.br3())
            .field("br4", &self.br4())
            .field("br5", &self.br5())
            .field("br6", &self.br6())
            .field("br7", &self.br7())
            .field("br8", &self.br8())
            .field("br9", &self.br9())
            .field("br10", &self.br10())
            .field("br11", &self.br11())
            .field("br12", &self.br12())
            .field("br13", &self.br13())
            .field("br14", &self.br14())
            .field("br15", &self.br15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port Reset bit
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W<BRRrs> {
        BR0_W::new(self, 0)
    }
    ///Bit 1 - Port Reset bit
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W<BRRrs> {
        BR1_W::new(self, 1)
    }
    ///Bit 2 - Port Reset bit
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W<BRRrs> {
        BR2_W::new(self, 2)
    }
    ///Bit 3 - Port Reset bit
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<BRRrs> {
        BR3_W::new(self, 3)
    }
    ///Bit 4 - Port Reset bit
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W<BRRrs> {
        BR4_W::new(self, 4)
    }
    ///Bit 5 - Port Reset bit
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W<BRRrs> {
        BR5_W::new(self, 5)
    }
    ///Bit 6 - Port Reset bit
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W<BRRrs> {
        BR6_W::new(self, 6)
    }
    ///Bit 7 - Port Reset bit
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W<BRRrs> {
        BR7_W::new(self, 7)
    }
    ///Bit 8 - Port Reset bit
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W<BRRrs> {
        BR8_W::new(self, 8)
    }
    ///Bit 9 - Port Reset bit
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W<BRRrs> {
        BR9_W::new(self, 9)
    }
    ///Bit 10 - Port Reset bit
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W<BRRrs> {
        BR10_W::new(self, 10)
    }
    ///Bit 11 - Port Reset bit
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W<BRRrs> {
        BR11_W::new(self, 11)
    }
    ///Bit 12 - Port Reset bit
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W<BRRrs> {
        BR12_W::new(self, 12)
    }
    ///Bit 13 - Port Reset bit
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W<BRRrs> {
        BR13_W::new(self, 13)
    }
    ///Bit 14 - Port Reset bit
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W<BRRrs> {
        BR14_W::new(self, 14)
    }
    ///Bit 15 - Port Reset bit
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W<BRRrs> {
        BR15_W::new(self, 15)
    }
}
/**GPIO port bit reset register

You can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#GPIOA:BRR)*/
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
///`read()` method returns [`brr::R`](R) reader structure
impl crate::Readable for BRRrs {}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRRrs {
    const RESET_VALUE: u32 = 0;
}
