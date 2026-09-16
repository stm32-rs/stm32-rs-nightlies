///Register `ASCR` reader
pub type R = crate::R<ASCRrs>;
///Register `ASCR` writer
pub type W = crate::W<ASCRrs>;
/**These bits are written by software to configure the analog connection of the IOs.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASC0W {
    ///0: Disconnect analog switch to the ADC input
    NoAction = 0,
    ///1: Connect analog switch to the ADC input
    Reset = 1,
}
impl From<ASC0W> for bool {
    #[inline(always)]
    fn from(variant: ASC0W) -> Self {
        variant as u8 != 0
    }
}
///Field `ASC0` reader - These bits are written by software to configure the analog connection of the IOs.
pub type ASC0_R = crate::BitReader<ASC0W>;
impl ASC0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASC0W {
        match self.bits {
            false => ASC0W::NoAction,
            true => ASC0W::Reset,
        }
    }
    ///Disconnect analog switch to the ADC input
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == ASC0W::NoAction
    }
    ///Connect analog switch to the ADC input
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ASC0W::Reset
    }
}
///Field `ASC0` writer - These bits are written by software to configure the analog connection of the IOs.
pub type ASC0_W<'a, REG> = crate::BitWriter<'a, REG, ASC0W>;
impl<'a, REG> ASC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disconnect analog switch to the ADC input
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(ASC0W::NoAction)
    }
    ///Connect analog switch to the ADC input
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(ASC0W::Reset)
    }
}
///Field `ASC1` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC1_R;
///Field `ASC2` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC2_R;
///Field `ASC3` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC3_R;
///Field `ASC4` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC4_R;
///Field `ASC5` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC5_R;
///Field `ASC6` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC6_R;
///Field `ASC7` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC7_R;
///Field `ASC8` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC8_R;
///Field `ASC9` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC9_R;
///Field `ASC10` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC10_R;
///Field `ASC11` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC11_R;
///Field `ASC12` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC12_R;
///Field `ASC13` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC13_R;
///Field `ASC14` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC14_R;
///Field `ASC15` reader - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_R as ASC15_R;
///Field `ASC1` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC1_W;
///Field `ASC2` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC2_W;
///Field `ASC3` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC3_W;
///Field `ASC4` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC4_W;
///Field `ASC5` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC5_W;
///Field `ASC6` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC6_W;
///Field `ASC7` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC7_W;
///Field `ASC8` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC8_W;
///Field `ASC9` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC9_W;
///Field `ASC10` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC10_W;
///Field `ASC11` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC11_W;
///Field `ASC12` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC12_W;
///Field `ASC13` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC13_W;
///Field `ASC14` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC14_W;
///Field `ASC15` writer - These bits are written by software to configure the analog connection of the IOs.
pub use ASC0_W as ASC15_W;
impl R {
    ///Bit 0 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc0(&self) -> ASC0_R {
        ASC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc1(&self) -> ASC1_R {
        ASC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc2(&self) -> ASC2_R {
        ASC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc3(&self) -> ASC3_R {
        ASC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc4(&self) -> ASC4_R {
        ASC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc5(&self) -> ASC5_R {
        ASC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc6(&self) -> ASC6_R {
        ASC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc7(&self) -> ASC7_R {
        ASC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc8(&self) -> ASC8_R {
        ASC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc9(&self) -> ASC9_R {
        ASC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc10(&self) -> ASC10_R {
        ASC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc11(&self) -> ASC11_R {
        ASC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc12(&self) -> ASC12_R {
        ASC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc13(&self) -> ASC13_R {
        ASC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc14(&self) -> ASC14_R {
        ASC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc15(&self) -> ASC15_R {
        ASC15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASCR")
            .field("asc0", &self.asc0())
            .field("asc1", &self.asc1())
            .field("asc2", &self.asc2())
            .field("asc3", &self.asc3())
            .field("asc4", &self.asc4())
            .field("asc5", &self.asc5())
            .field("asc6", &self.asc6())
            .field("asc7", &self.asc7())
            .field("asc8", &self.asc8())
            .field("asc9", &self.asc9())
            .field("asc10", &self.asc10())
            .field("asc11", &self.asc11())
            .field("asc12", &self.asc12())
            .field("asc13", &self.asc13())
            .field("asc14", &self.asc14())
            .field("asc15", &self.asc15())
            .finish()
    }
}
impl W {
    ///Bit 0 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc0(&mut self) -> ASC0_W<'_, ASCRrs> {
        ASC0_W::new(self, 0)
    }
    ///Bit 1 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc1(&mut self) -> ASC1_W<'_, ASCRrs> {
        ASC1_W::new(self, 1)
    }
    ///Bit 2 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc2(&mut self) -> ASC2_W<'_, ASCRrs> {
        ASC2_W::new(self, 2)
    }
    ///Bit 3 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc3(&mut self) -> ASC3_W<'_, ASCRrs> {
        ASC3_W::new(self, 3)
    }
    ///Bit 4 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc4(&mut self) -> ASC4_W<'_, ASCRrs> {
        ASC4_W::new(self, 4)
    }
    ///Bit 5 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc5(&mut self) -> ASC5_W<'_, ASCRrs> {
        ASC5_W::new(self, 5)
    }
    ///Bit 6 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc6(&mut self) -> ASC6_W<'_, ASCRrs> {
        ASC6_W::new(self, 6)
    }
    ///Bit 7 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc7(&mut self) -> ASC7_W<'_, ASCRrs> {
        ASC7_W::new(self, 7)
    }
    ///Bit 8 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc8(&mut self) -> ASC8_W<'_, ASCRrs> {
        ASC8_W::new(self, 8)
    }
    ///Bit 9 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc9(&mut self) -> ASC9_W<'_, ASCRrs> {
        ASC9_W::new(self, 9)
    }
    ///Bit 10 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc10(&mut self) -> ASC10_W<'_, ASCRrs> {
        ASC10_W::new(self, 10)
    }
    ///Bit 11 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc11(&mut self) -> ASC11_W<'_, ASCRrs> {
        ASC11_W::new(self, 11)
    }
    ///Bit 12 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc12(&mut self) -> ASC12_W<'_, ASCRrs> {
        ASC12_W::new(self, 12)
    }
    ///Bit 13 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc13(&mut self) -> ASC13_W<'_, ASCRrs> {
        ASC13_W::new(self, 13)
    }
    ///Bit 14 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc14(&mut self) -> ASC14_W<'_, ASCRrs> {
        ASC14_W::new(self, 14)
    }
    ///Bit 15 - These bits are written by software to configure the analog connection of the IOs.
    #[inline(always)]
    pub fn asc15(&mut self) -> ASC15_W<'_, ASCRrs> {
        ASC15_W::new(self, 15)
    }
}
/**GPIO port analog switch control register

You can [`read`](crate::Reg::read) this register and get [`ascr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ascr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#GPIOB:ASCR)*/
pub struct ASCRrs;
impl crate::RegisterSpec for ASCRrs {
    type Ux = u32;
}
///`read()` method returns [`ascr::R`](R) reader structure
impl crate::Readable for ASCRrs {}
///`write(|w| ..)` method takes [`ascr::W`](W) writer structure
impl crate::Writable for ASCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ASCR to value 0
impl crate::Resettable for ASCRrs {}
