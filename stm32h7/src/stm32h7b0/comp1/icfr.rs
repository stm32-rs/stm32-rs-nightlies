///Register `ICFR` writer
pub type W = crate::W<ICFRrs>;
///Field `CC1IF` writer - Clear COMP channel 1 Interrupt Flag
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2IF` writer - Clear COMP channel 2 Interrupt Flag
pub type CC2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 16 - Clear COMP channel 1 Interrupt Flag
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<'_, ICFRrs> {
        CC1IF_W::new(self, 16)
    }
    ///Bit 17 - Clear COMP channel 2 Interrupt Flag
    #[inline(always)]
    pub fn cc2if(&mut self) -> CC2IF_W<'_, ICFRrs> {
        CC2IF_W::new(self, 17)
    }
}
/**Comparator interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#COMP1:ICFR)*/
pub struct ICFRrs;
impl crate::RegisterSpec for ICFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icfr::W`](W) writer structure
impl crate::Writable for ICFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICFR to value 0
impl crate::Resettable for ICFRrs {}
