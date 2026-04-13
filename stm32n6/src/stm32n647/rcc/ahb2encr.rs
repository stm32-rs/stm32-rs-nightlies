///Register `AHB2ENCR` writer
pub type W = crate::W<AHB2ENCRrs>;
///Field `RAMCFGENC` writer - RAMCFG enable
pub type RAMCFGENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDF1ENC` writer - MDF1 enable
pub type MDF1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1ENC` writer - ADF1 enable
pub type ADF1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB2ENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 12 - RAMCFG enable
    #[inline(always)]
    pub fn ramcfgenc(&mut self) -> RAMCFGENC_W<'_, AHB2ENCRrs> {
        RAMCFGENC_W::new(self, 12)
    }
    ///Bit 16 - MDF1 enable
    #[inline(always)]
    pub fn mdf1enc(&mut self) -> MDF1ENC_W<'_, AHB2ENCRrs> {
        MDF1ENC_W::new(self, 16)
    }
    ///Bit 17 - ADF1 enable
    #[inline(always)]
    pub fn adf1enc(&mut self) -> ADF1ENC_W<'_, AHB2ENCRrs> {
        ADF1ENC_W::new(self, 17)
    }
}
/**RCC AHB2 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2encr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB2ENCR)*/
pub struct AHB2ENCRrs;
impl crate::RegisterSpec for AHB2ENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb2encr::W`](W) writer structure
impl crate::Writable for AHB2ENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2ENCR to value 0
impl crate::Resettable for AHB2ENCRrs {}
