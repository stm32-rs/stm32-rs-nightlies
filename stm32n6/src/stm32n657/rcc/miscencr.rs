///Register `MISCENCR` writer
pub type W = crate::W<MISCENCRrs>;
///Field `DBGENC` writer - DBG enable
pub type DBGENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCO1ENC` writer - MCO1 enable
pub type MCO1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCO2ENC` writer - MCO2 enable
pub type MCO2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIPHYCOMPENC` writer - XSPIPHYCOMP enable
pub type XSPIPHYCOMPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERENC` writer - PER enable
pub type PERENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MISCENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - DBG enable
    #[inline(always)]
    pub fn dbgenc(&mut self) -> DBGENC_W<'_, MISCENCRrs> {
        DBGENC_W::new(self, 0)
    }
    ///Bit 1 - MCO1 enable
    #[inline(always)]
    pub fn mco1enc(&mut self) -> MCO1ENC_W<'_, MISCENCRrs> {
        MCO1ENC_W::new(self, 1)
    }
    ///Bit 2 - MCO2 enable
    #[inline(always)]
    pub fn mco2enc(&mut self) -> MCO2ENC_W<'_, MISCENCRrs> {
        MCO2ENC_W::new(self, 2)
    }
    ///Bit 3 - XSPIPHYCOMP enable
    #[inline(always)]
    pub fn xspiphycompenc(&mut self) -> XSPIPHYCOMPENC_W<'_, MISCENCRrs> {
        XSPIPHYCOMPENC_W::new(self, 3)
    }
    ///Bit 6 - PER enable
    #[inline(always)]
    pub fn perenc(&mut self) -> PERENC_W<'_, MISCENCRrs> {
        PERENC_W::new(self, 6)
    }
}
/**RCC miscellaneous enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:MISCENCR)*/
pub struct MISCENCRrs;
impl crate::RegisterSpec for MISCENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`miscencr::W`](W) writer structure
impl crate::Writable for MISCENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MISCENCR to value 0
impl crate::Resettable for MISCENCRrs {}
