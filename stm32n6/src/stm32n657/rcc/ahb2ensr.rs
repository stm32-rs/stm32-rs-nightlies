///Register `AHB2ENSR` writer
pub type W = crate::W<AHB2ENSRrs>;
///Field `RAMCFGENS` writer - RAMCFG enable
pub type RAMCFGENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDF1ENS` writer - MDF1 enable
pub type MDF1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1ENS` writer - ADF1 enable
pub type ADF1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB2ENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 12 - RAMCFG enable
    #[inline(always)]
    pub fn ramcfgens(&mut self) -> RAMCFGENS_W<'_, AHB2ENSRrs> {
        RAMCFGENS_W::new(self, 12)
    }
    ///Bit 16 - MDF1 enable
    #[inline(always)]
    pub fn mdf1ens(&mut self) -> MDF1ENS_W<'_, AHB2ENSRrs> {
        MDF1ENS_W::new(self, 16)
    }
    ///Bit 17 - ADF1 enable
    #[inline(always)]
    pub fn adf1ens(&mut self) -> ADF1ENS_W<'_, AHB2ENSRrs> {
        ADF1ENS_W::new(self, 17)
    }
}
/**RCC AHB2 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2ensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:AHB2ENSR)*/
pub struct AHB2ENSRrs;
impl crate::RegisterSpec for AHB2ENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb2ensr::W`](W) writer structure
impl crate::Writable for AHB2ENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2ENSR to value 0
impl crate::Resettable for AHB2ENSRrs {}
