///Register `MISCENSR` writer
pub type W = crate::W<MISCENSRrs>;
///Field `DBGENS` writer - DBG enable
pub type DBGENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCO1ENS` writer - MCO1 enable
pub type MCO1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCO2ENS` writer - MCO2 enable
pub type MCO2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIPHYCOMPENS` writer - XSPIPHYCOMP enable
pub type XSPIPHYCOMPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERENS` writer - PER enable
pub type PERENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MISCENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - DBG enable
    #[inline(always)]
    pub fn dbgens(&mut self) -> DBGENS_W<'_, MISCENSRrs> {
        DBGENS_W::new(self, 0)
    }
    ///Bit 1 - MCO1 enable
    #[inline(always)]
    pub fn mco1ens(&mut self) -> MCO1ENS_W<'_, MISCENSRrs> {
        MCO1ENS_W::new(self, 1)
    }
    ///Bit 2 - MCO2 enable
    #[inline(always)]
    pub fn mco2ens(&mut self) -> MCO2ENS_W<'_, MISCENSRrs> {
        MCO2ENS_W::new(self, 2)
    }
    ///Bit 3 - XSPIPHYCOMP enable
    #[inline(always)]
    pub fn xspiphycompens(&mut self) -> XSPIPHYCOMPENS_W<'_, MISCENSRrs> {
        XSPIPHYCOMPENS_W::new(self, 3)
    }
    ///Bit 6 - PER enable
    #[inline(always)]
    pub fn perens(&mut self) -> PERENS_W<'_, MISCENSRrs> {
        PERENS_W::new(self, 6)
    }
}
/**RCC miscellaneous enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MISCENSR)*/
pub struct MISCENSRrs;
impl crate::RegisterSpec for MISCENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`miscensr::W`](W) writer structure
impl crate::Writable for MISCENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MISCENSR to value 0
impl crate::Resettable for MISCENSRrs {}
