///Register `SWPR2` writer
pub type W = crate::W<SWPR2rs>;
/**P32WP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P32WP {
    ///0: SRAM2 1 KB page protection disabled
    Disabled = 0,
    ///1: SRAM2 1 KB page protection enabled
    Enabled = 1,
}
impl From<P32WP> for bool {
    #[inline(always)]
    fn from(variant: P32WP) -> Self {
        variant as u8 != 0
    }
}
///Field `P32WP` writer - P32WP
pub type P32WP_W<'a, REG> = crate::BitWriter<'a, REG, P32WP>;
impl<'a, REG> P32WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM2 1 KB page protection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P32WP::Disabled)
    }
    ///SRAM2 1 KB page protection enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(P32WP::Enabled)
    }
}
///Field `P33WP` writer - P33WP
pub use P32WP_W as P33WP_W;
///Field `P34WP` writer - P34WP
pub use P32WP_W as P34WP_W;
///Field `P35WP` writer - P35WP
pub use P32WP_W as P35WP_W;
///Field `P36WP` writer - P36WP
pub use P32WP_W as P36WP_W;
///Field `P37WP` writer - P37WP
pub use P32WP_W as P37WP_W;
///Field `P38WP` writer - P38WP
pub use P32WP_W as P38WP_W;
///Field `P39WP` writer - P39WP
pub use P32WP_W as P39WP_W;
///Field `P40WP` writer - P40WP
pub use P32WP_W as P40WP_W;
///Field `P41WP` writer - P41WP
pub use P32WP_W as P41WP_W;
///Field `P42WP` writer - P42WP
pub use P32WP_W as P42WP_W;
///Field `P43WP` writer - P43WP
pub use P32WP_W as P43WP_W;
///Field `P44WP` writer - P44WP
pub use P32WP_W as P44WP_W;
///Field `P45WP` writer - P45WP
pub use P32WP_W as P45WP_W;
///Field `P46WP` writer - P46WP
pub use P32WP_W as P46WP_W;
///Field `P47WP` writer - P47WP
pub use P32WP_W as P47WP_W;
///Field `P48WP` writer - P48WP
pub use P32WP_W as P48WP_W;
///Field `P49WP` writer - P49WP
pub use P32WP_W as P49WP_W;
///Field `P50WP` writer - P50WP
pub use P32WP_W as P50WP_W;
///Field `P51WP` writer - P51WP
pub use P32WP_W as P51WP_W;
///Field `P52WP` writer - P52WP
pub use P32WP_W as P52WP_W;
///Field `P53WP` writer - P53WP
pub use P32WP_W as P53WP_W;
///Field `P54WP` writer - P54WP
pub use P32WP_W as P54WP_W;
///Field `P55WP` writer - P55WP
pub use P32WP_W as P55WP_W;
///Field `P56WP` writer - P56WP
pub use P32WP_W as P56WP_W;
///Field `P57WP` writer - P57WP
pub use P32WP_W as P57WP_W;
///Field `P58WP` writer - P58WP
pub use P32WP_W as P58WP_W;
///Field `P59WP` writer - P59WP
pub use P32WP_W as P59WP_W;
///Field `P60WP` writer - P60WP
pub use P32WP_W as P60WP_W;
///Field `P61WP` writer - P61WP
pub use P32WP_W as P61WP_W;
///Field `P62WP` writer - P62WP
pub use P32WP_W as P62WP_W;
///Field `P63WP` writer - SRAM2 page 63 write protection
pub use P32WP_W as P63WP_W;
impl core::fmt::Debug for crate::generic::Reg<SWPR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - P32WP
    #[inline(always)]
    pub fn p32wp(&mut self) -> P32WP_W<'_, SWPR2rs> {
        P32WP_W::new(self, 0)
    }
    ///Bit 1 - P33WP
    #[inline(always)]
    pub fn p33wp(&mut self) -> P33WP_W<'_, SWPR2rs> {
        P33WP_W::new(self, 1)
    }
    ///Bit 2 - P34WP
    #[inline(always)]
    pub fn p34wp(&mut self) -> P34WP_W<'_, SWPR2rs> {
        P34WP_W::new(self, 2)
    }
    ///Bit 3 - P35WP
    #[inline(always)]
    pub fn p35wp(&mut self) -> P35WP_W<'_, SWPR2rs> {
        P35WP_W::new(self, 3)
    }
    ///Bit 4 - P36WP
    #[inline(always)]
    pub fn p36wp(&mut self) -> P36WP_W<'_, SWPR2rs> {
        P36WP_W::new(self, 4)
    }
    ///Bit 5 - P37WP
    #[inline(always)]
    pub fn p37wp(&mut self) -> P37WP_W<'_, SWPR2rs> {
        P37WP_W::new(self, 5)
    }
    ///Bit 6 - P38WP
    #[inline(always)]
    pub fn p38wp(&mut self) -> P38WP_W<'_, SWPR2rs> {
        P38WP_W::new(self, 6)
    }
    ///Bit 7 - P39WP
    #[inline(always)]
    pub fn p39wp(&mut self) -> P39WP_W<'_, SWPR2rs> {
        P39WP_W::new(self, 7)
    }
    ///Bit 8 - P40WP
    #[inline(always)]
    pub fn p40wp(&mut self) -> P40WP_W<'_, SWPR2rs> {
        P40WP_W::new(self, 8)
    }
    ///Bit 9 - P41WP
    #[inline(always)]
    pub fn p41wp(&mut self) -> P41WP_W<'_, SWPR2rs> {
        P41WP_W::new(self, 9)
    }
    ///Bit 10 - P42WP
    #[inline(always)]
    pub fn p42wp(&mut self) -> P42WP_W<'_, SWPR2rs> {
        P42WP_W::new(self, 10)
    }
    ///Bit 11 - P43WP
    #[inline(always)]
    pub fn p43wp(&mut self) -> P43WP_W<'_, SWPR2rs> {
        P43WP_W::new(self, 11)
    }
    ///Bit 12 - P44WP
    #[inline(always)]
    pub fn p44wp(&mut self) -> P44WP_W<'_, SWPR2rs> {
        P44WP_W::new(self, 12)
    }
    ///Bit 13 - P45WP
    #[inline(always)]
    pub fn p45wp(&mut self) -> P45WP_W<'_, SWPR2rs> {
        P45WP_W::new(self, 13)
    }
    ///Bit 14 - P46WP
    #[inline(always)]
    pub fn p46wp(&mut self) -> P46WP_W<'_, SWPR2rs> {
        P46WP_W::new(self, 14)
    }
    ///Bit 15 - P47WP
    #[inline(always)]
    pub fn p47wp(&mut self) -> P47WP_W<'_, SWPR2rs> {
        P47WP_W::new(self, 15)
    }
    ///Bit 16 - P48WP
    #[inline(always)]
    pub fn p48wp(&mut self) -> P48WP_W<'_, SWPR2rs> {
        P48WP_W::new(self, 16)
    }
    ///Bit 17 - P49WP
    #[inline(always)]
    pub fn p49wp(&mut self) -> P49WP_W<'_, SWPR2rs> {
        P49WP_W::new(self, 17)
    }
    ///Bit 18 - P50WP
    #[inline(always)]
    pub fn p50wp(&mut self) -> P50WP_W<'_, SWPR2rs> {
        P50WP_W::new(self, 18)
    }
    ///Bit 19 - P51WP
    #[inline(always)]
    pub fn p51wp(&mut self) -> P51WP_W<'_, SWPR2rs> {
        P51WP_W::new(self, 19)
    }
    ///Bit 20 - P52WP
    #[inline(always)]
    pub fn p52wp(&mut self) -> P52WP_W<'_, SWPR2rs> {
        P52WP_W::new(self, 20)
    }
    ///Bit 21 - P53WP
    #[inline(always)]
    pub fn p53wp(&mut self) -> P53WP_W<'_, SWPR2rs> {
        P53WP_W::new(self, 21)
    }
    ///Bit 22 - P54WP
    #[inline(always)]
    pub fn p54wp(&mut self) -> P54WP_W<'_, SWPR2rs> {
        P54WP_W::new(self, 22)
    }
    ///Bit 23 - P55WP
    #[inline(always)]
    pub fn p55wp(&mut self) -> P55WP_W<'_, SWPR2rs> {
        P55WP_W::new(self, 23)
    }
    ///Bit 24 - P56WP
    #[inline(always)]
    pub fn p56wp(&mut self) -> P56WP_W<'_, SWPR2rs> {
        P56WP_W::new(self, 24)
    }
    ///Bit 25 - P57WP
    #[inline(always)]
    pub fn p57wp(&mut self) -> P57WP_W<'_, SWPR2rs> {
        P57WP_W::new(self, 25)
    }
    ///Bit 26 - P58WP
    #[inline(always)]
    pub fn p58wp(&mut self) -> P58WP_W<'_, SWPR2rs> {
        P58WP_W::new(self, 26)
    }
    ///Bit 27 - P59WP
    #[inline(always)]
    pub fn p59wp(&mut self) -> P59WP_W<'_, SWPR2rs> {
        P59WP_W::new(self, 27)
    }
    ///Bit 28 - P60WP
    #[inline(always)]
    pub fn p60wp(&mut self) -> P60WP_W<'_, SWPR2rs> {
        P60WP_W::new(self, 28)
    }
    ///Bit 29 - P61WP
    #[inline(always)]
    pub fn p61wp(&mut self) -> P61WP_W<'_, SWPR2rs> {
        P61WP_W::new(self, 29)
    }
    ///Bit 30 - P62WP
    #[inline(always)]
    pub fn p62wp(&mut self) -> P62WP_W<'_, SWPR2rs> {
        P62WP_W::new(self, 30)
    }
    ///Bit 31 - SRAM2 page 63 write protection
    #[inline(always)]
    pub fn p63wp(&mut self) -> P63WP_W<'_, SWPR2rs> {
        P63WP_W::new(self, 31)
    }
}
/**SRAM2 write protection register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:SWPR2)*/
pub struct SWPR2rs;
impl crate::RegisterSpec for SWPR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`swpr2::W`](W) writer structure
impl crate::Writable for SWPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWPR2 to value 0
impl crate::Resettable for SWPR2rs {}
