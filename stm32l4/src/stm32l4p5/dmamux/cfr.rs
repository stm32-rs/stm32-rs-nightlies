///Register `CFR` writer
pub type W = crate::W<CFRrs>;
/**Clear synchronization overrun event flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSOF0 {
    ///1: Clear synchronization flag
    Clear = 1,
}
impl From<CSOF0> for bool {
    #[inline(always)]
    fn from(variant: CSOF0) -> Self {
        variant as u8 != 0
    }
}
///Field `CSOF0` writer - Clear synchronization overrun event flag
pub type CSOF0_W<'a, REG> = crate::BitWriter<'a, REG, CSOF0>;
impl<'a, REG> CSOF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear synchronization flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSOF0::Clear)
    }
}
///Field `CSOF1` writer - Clear synchronization overrun event flag
pub use CSOF0_W as CSOF1_W;
///Field `CSOF2` writer - Clear synchronization overrun event flag
pub use CSOF0_W as CSOF2_W;
///Field `CSOF3` writer - Clear synchronization overrun event flag
pub use CSOF0_W as CSOF3_W;
///Field `CSOF4` writer - Clear synchronization overrun event flag
pub use CSOF0_W as CSOF4_W;
///Field `CSOF5` writer - Clear synchronization overrun event flag
pub use CSOF0_W as CSOF5_W;
///Field `CSOF6` writer - Clear synchronization overrun event flag
pub use CSOF0_W as CSOF6_W;
///Field `CSOF7` writer - Clear synchronization overrun event flag
pub use CSOF0_W as CSOF7_W;
///Field `CSOF8` writer - Clear synchronization overrun event flag
pub use CSOF0_W as CSOF8_W;
///Field `CSOF9` writer - Clear synchronization overrun event flag
pub use CSOF0_W as CSOF9_W;
///Field `CSOF10` writer - Clear synchronization overrun event flag
pub use CSOF0_W as CSOF10_W;
///Field `CSOF11` writer - Clear synchronization overrun event flag
pub use CSOF0_W as CSOF11_W;
///Field `CSOF12` writer - Clear synchronization overrun event flag
pub use CSOF0_W as CSOF12_W;
///Field `CSOF13` writer - Clear synchronization overrun event flag
pub use CSOF0_W as CSOF13_W;
impl core::fmt::Debug for crate::generic::Reg<CFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof0(&mut self) -> CSOF0_W<CFRrs> {
        CSOF0_W::new(self, 0)
    }
    ///Bit 1 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof1(&mut self) -> CSOF1_W<CFRrs> {
        CSOF1_W::new(self, 1)
    }
    ///Bit 2 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof2(&mut self) -> CSOF2_W<CFRrs> {
        CSOF2_W::new(self, 2)
    }
    ///Bit 3 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof3(&mut self) -> CSOF3_W<CFRrs> {
        CSOF3_W::new(self, 3)
    }
    ///Bit 4 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof4(&mut self) -> CSOF4_W<CFRrs> {
        CSOF4_W::new(self, 4)
    }
    ///Bit 5 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof5(&mut self) -> CSOF5_W<CFRrs> {
        CSOF5_W::new(self, 5)
    }
    ///Bit 6 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof6(&mut self) -> CSOF6_W<CFRrs> {
        CSOF6_W::new(self, 6)
    }
    ///Bit 7 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof7(&mut self) -> CSOF7_W<CFRrs> {
        CSOF7_W::new(self, 7)
    }
    ///Bit 8 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof8(&mut self) -> CSOF8_W<CFRrs> {
        CSOF8_W::new(self, 8)
    }
    ///Bit 9 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof9(&mut self) -> CSOF9_W<CFRrs> {
        CSOF9_W::new(self, 9)
    }
    ///Bit 10 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof10(&mut self) -> CSOF10_W<CFRrs> {
        CSOF10_W::new(self, 10)
    }
    ///Bit 11 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof11(&mut self) -> CSOF11_W<CFRrs> {
        CSOF11_W::new(self, 11)
    }
    ///Bit 12 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof12(&mut self) -> CSOF12_W<CFRrs> {
        CSOF12_W::new(self, 12)
    }
    ///Bit 13 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof13(&mut self) -> CSOF13_W<CFRrs> {
        CSOF13_W::new(self, 13)
    }
}
/**clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#DMAMUX:CFR)*/
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cfr::W`](W) writer structure
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFR to value 0
impl crate::Resettable for CFRrs {
    const RESET_VALUE: u32 = 0;
}
