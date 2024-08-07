///Register `DMAMUX_CFR` writer
pub type W = crate::W<DMAMUX_CFRrs>;
///Field `CSOF0` writer - CSOF0
pub type CSOF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF1` writer - CSOF1
pub type CSOF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF2` writer - CSOF2
pub type CSOF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF3` writer - CSOF3
pub type CSOF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF4` writer - CSOF4
pub type CSOF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF5` writer - CSOF5
pub type CSOF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF6` writer - CSOF6
pub type CSOF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF7` writer - CSOF7
pub type CSOF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF8` writer - CSOF8
pub type CSOF8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF9` writer - CSOF9
pub type CSOF9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF10` writer - CSOF10
pub type CSOF10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF11` writer - CSOF11
pub type CSOF11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF12` writer - CSOF12
pub type CSOF12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF13` writer - CSOF13
pub type CSOF13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF14` writer - CSOF14
pub type CSOF14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF15` writer - CSOF15
pub type CSOF15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DMAMUX_CFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CSOF0
    #[inline(always)]
    #[must_use]
    pub fn csof0(&mut self) -> CSOF0_W<DMAMUX_CFRrs> {
        CSOF0_W::new(self, 0)
    }
    ///Bit 1 - CSOF1
    #[inline(always)]
    #[must_use]
    pub fn csof1(&mut self) -> CSOF1_W<DMAMUX_CFRrs> {
        CSOF1_W::new(self, 1)
    }
    ///Bit 2 - CSOF2
    #[inline(always)]
    #[must_use]
    pub fn csof2(&mut self) -> CSOF2_W<DMAMUX_CFRrs> {
        CSOF2_W::new(self, 2)
    }
    ///Bit 3 - CSOF3
    #[inline(always)]
    #[must_use]
    pub fn csof3(&mut self) -> CSOF3_W<DMAMUX_CFRrs> {
        CSOF3_W::new(self, 3)
    }
    ///Bit 4 - CSOF4
    #[inline(always)]
    #[must_use]
    pub fn csof4(&mut self) -> CSOF4_W<DMAMUX_CFRrs> {
        CSOF4_W::new(self, 4)
    }
    ///Bit 5 - CSOF5
    #[inline(always)]
    #[must_use]
    pub fn csof5(&mut self) -> CSOF5_W<DMAMUX_CFRrs> {
        CSOF5_W::new(self, 5)
    }
    ///Bit 6 - CSOF6
    #[inline(always)]
    #[must_use]
    pub fn csof6(&mut self) -> CSOF6_W<DMAMUX_CFRrs> {
        CSOF6_W::new(self, 6)
    }
    ///Bit 7 - CSOF7
    #[inline(always)]
    #[must_use]
    pub fn csof7(&mut self) -> CSOF7_W<DMAMUX_CFRrs> {
        CSOF7_W::new(self, 7)
    }
    ///Bit 8 - CSOF8
    #[inline(always)]
    #[must_use]
    pub fn csof8(&mut self) -> CSOF8_W<DMAMUX_CFRrs> {
        CSOF8_W::new(self, 8)
    }
    ///Bit 9 - CSOF9
    #[inline(always)]
    #[must_use]
    pub fn csof9(&mut self) -> CSOF9_W<DMAMUX_CFRrs> {
        CSOF9_W::new(self, 9)
    }
    ///Bit 10 - CSOF10
    #[inline(always)]
    #[must_use]
    pub fn csof10(&mut self) -> CSOF10_W<DMAMUX_CFRrs> {
        CSOF10_W::new(self, 10)
    }
    ///Bit 11 - CSOF11
    #[inline(always)]
    #[must_use]
    pub fn csof11(&mut self) -> CSOF11_W<DMAMUX_CFRrs> {
        CSOF11_W::new(self, 11)
    }
    ///Bit 12 - CSOF12
    #[inline(always)]
    #[must_use]
    pub fn csof12(&mut self) -> CSOF12_W<DMAMUX_CFRrs> {
        CSOF12_W::new(self, 12)
    }
    ///Bit 13 - CSOF13
    #[inline(always)]
    #[must_use]
    pub fn csof13(&mut self) -> CSOF13_W<DMAMUX_CFRrs> {
        CSOF13_W::new(self, 13)
    }
    ///Bit 14 - CSOF14
    #[inline(always)]
    #[must_use]
    pub fn csof14(&mut self) -> CSOF14_W<DMAMUX_CFRrs> {
        CSOF14_W::new(self, 14)
    }
    ///Bit 15 - CSOF15
    #[inline(always)]
    #[must_use]
    pub fn csof15(&mut self) -> CSOF15_W<DMAMUX_CFRrs> {
        CSOF15_W::new(self, 15)
    }
}
/**DMAMUX request line multiplexer interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:DMAMUX_CFR)*/
pub struct DMAMUX_CFRrs;
impl crate::RegisterSpec for DMAMUX_CFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dmamux_cfr::W`](W) writer structure
impl crate::Writable for DMAMUX_CFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMAMUX_CFR to value 0
impl crate::Resettable for DMAMUX_CFRrs {
    const RESET_VALUE: u32 = 0;
}
