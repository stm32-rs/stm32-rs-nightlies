///Register `BRR` reader
pub type R = crate::R<BRRrs>;
///Register `BRR` writer
pub type W = crate::W<BRRrs>;
/**Port x reset pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_RESET {
    ///0: No action on the corresponding ODx bit
    NoAction = 0,
    ///1: Reset the ODx bit
    Reset = 1,
}
impl From<BIT_RESET> for bool {
    #[inline(always)]
    fn from(variant: BIT_RESET) -> Self {
        variant as u8 != 0
    }
}
///Field `BR(0-15)` reader - Port x reset pin %s
pub type BR_R = crate::BitReader<BIT_RESET>;
impl BR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BIT_RESET {
        match self.bits {
            false => BIT_RESET::NoAction,
            true => BIT_RESET::Reset,
        }
    }
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == BIT_RESET::NoAction
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BIT_RESET::Reset
    }
}
///Field `BR(0-15)` writer - Port x reset pin %s
pub type BR_W<'a, REG> = crate::BitWriter<'a, REG, BIT_RESET>;
impl<'a, REG> BR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_RESET::NoAction)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_RESET::Reset)
    }
}
impl R {
    ///Port x reset pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BR0` field.</div>
    #[inline(always)]
    pub fn br(&self, n: u8) -> BR_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        BR_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Port x reset pin (0-15)
    #[inline(always)]
    pub fn br_iter(&self) -> impl Iterator<Item = BR_R> + '_ {
        (0..16).map(move |n| BR_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Port x reset pin 0
    #[inline(always)]
    pub fn br0(&self) -> BR_R {
        BR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x reset pin 1
    #[inline(always)]
    pub fn br1(&self) -> BR_R {
        BR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x reset pin 2
    #[inline(always)]
    pub fn br2(&self) -> BR_R {
        BR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x reset pin 3
    #[inline(always)]
    pub fn br3(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x reset pin 4
    #[inline(always)]
    pub fn br4(&self) -> BR_R {
        BR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port x reset pin 5
    #[inline(always)]
    pub fn br5(&self) -> BR_R {
        BR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port x reset pin 6
    #[inline(always)]
    pub fn br6(&self) -> BR_R {
        BR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port x reset pin 7
    #[inline(always)]
    pub fn br7(&self) -> BR_R {
        BR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port x reset pin 8
    #[inline(always)]
    pub fn br8(&self) -> BR_R {
        BR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port x reset pin 9
    #[inline(always)]
    pub fn br9(&self) -> BR_R {
        BR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port x reset pin 10
    #[inline(always)]
    pub fn br10(&self) -> BR_R {
        BR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port x reset pin 11
    #[inline(always)]
    pub fn br11(&self) -> BR_R {
        BR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port x reset pin 12
    #[inline(always)]
    pub fn br12(&self) -> BR_R {
        BR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port x reset pin 13
    #[inline(always)]
    pub fn br13(&self) -> BR_R {
        BR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port x reset pin 14
    #[inline(always)]
    pub fn br14(&self) -> BR_R {
        BR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port x reset pin 15
    #[inline(always)]
    pub fn br15(&self) -> BR_R {
        BR_R::new(((self.bits >> 15) & 1) != 0)
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
    ///Port x reset pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BR0` field.</div>
    #[inline(always)]
    pub fn br(&mut self, n: u8) -> BR_W<'_, BRRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        BR_W::new(self, n)
    }
    ///Bit 0 - Port x reset pin 0
    #[inline(always)]
    pub fn br0(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 0)
    }
    ///Bit 1 - Port x reset pin 1
    #[inline(always)]
    pub fn br1(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 1)
    }
    ///Bit 2 - Port x reset pin 2
    #[inline(always)]
    pub fn br2(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 2)
    }
    ///Bit 3 - Port x reset pin 3
    #[inline(always)]
    pub fn br3(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 3)
    }
    ///Bit 4 - Port x reset pin 4
    #[inline(always)]
    pub fn br4(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 4)
    }
    ///Bit 5 - Port x reset pin 5
    #[inline(always)]
    pub fn br5(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 5)
    }
    ///Bit 6 - Port x reset pin 6
    #[inline(always)]
    pub fn br6(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 6)
    }
    ///Bit 7 - Port x reset pin 7
    #[inline(always)]
    pub fn br7(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 7)
    }
    ///Bit 8 - Port x reset pin 8
    #[inline(always)]
    pub fn br8(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 8)
    }
    ///Bit 9 - Port x reset pin 9
    #[inline(always)]
    pub fn br9(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 9)
    }
    ///Bit 10 - Port x reset pin 10
    #[inline(always)]
    pub fn br10(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 10)
    }
    ///Bit 11 - Port x reset pin 11
    #[inline(always)]
    pub fn br11(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 11)
    }
    ///Bit 12 - Port x reset pin 12
    #[inline(always)]
    pub fn br12(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 12)
    }
    ///Bit 13 - Port x reset pin 13
    #[inline(always)]
    pub fn br13(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 13)
    }
    ///Bit 14 - Port x reset pin 14
    #[inline(always)]
    pub fn br14(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 14)
    }
    ///Bit 15 - Port x reset pin 15
    #[inline(always)]
    pub fn br15(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 15)
    }
}
/**GPIO port bit reset register

You can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F756.html#GPIOA:BRR)*/
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
///`read()` method returns [`brr::R`](R) reader structure
impl crate::Readable for BRRrs {}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRRrs {}
