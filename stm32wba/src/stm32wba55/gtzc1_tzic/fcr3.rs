///Register `FCR3` writer
pub type W = crate::W<FCR3rs>;
///Field `CCRCF` writer - clear the illegal access flag for CRC
pub type CCRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSCF` writer - clear the illegal access flag for TSC
pub type CTSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CICACHEF` writer - clear the illegal access flag for ICACHE registers
pub type CICACHEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAESF` writer - clear the illegal access flag for AES
pub type CAESF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHASHF` writer - clear the illegal access flag for HASH
pub type CHASHF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRNGF` writer - clear the illegal access flag for RNG
pub type CRNGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSAESF` writer - clear the illegal access flag for SAES
pub type CSAESF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEMF` writer - clear the illegal access flag for HSEM
pub type CHSEMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPKAF` writer - clear the illegal access flag for PKA
pub type CPKAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRAMCFGF` writer - clear the illegal access flag for RAMCFG
pub type CRAMCFGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRADIOF` writer - clear the illegal access flag for 2.4 GHz RADIO
pub type CRADIOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - clear the illegal access flag for CRC
    #[inline(always)]
    pub fn ccrcf(&mut self) -> CCRCF_W<'_, FCR3rs> {
        CCRCF_W::new(self, 3)
    }
    ///Bit 4 - clear the illegal access flag for TSC
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W<'_, FCR3rs> {
        CTSCF_W::new(self, 4)
    }
    ///Bit 6 - clear the illegal access flag for ICACHE registers
    #[inline(always)]
    pub fn cicachef(&mut self) -> CICACHEF_W<'_, FCR3rs> {
        CICACHEF_W::new(self, 6)
    }
    ///Bit 11 - clear the illegal access flag for AES
    #[inline(always)]
    pub fn caesf(&mut self) -> CAESF_W<'_, FCR3rs> {
        CAESF_W::new(self, 11)
    }
    ///Bit 12 - clear the illegal access flag for HASH
    #[inline(always)]
    pub fn chashf(&mut self) -> CHASHF_W<'_, FCR3rs> {
        CHASHF_W::new(self, 12)
    }
    ///Bit 13 - clear the illegal access flag for RNG
    #[inline(always)]
    pub fn crngf(&mut self) -> CRNGF_W<'_, FCR3rs> {
        CRNGF_W::new(self, 13)
    }
    ///Bit 14 - clear the illegal access flag for SAES
    #[inline(always)]
    pub fn csaesf(&mut self) -> CSAESF_W<'_, FCR3rs> {
        CSAESF_W::new(self, 14)
    }
    ///Bit 15 - clear the illegal access flag for HSEM
    #[inline(always)]
    pub fn chsemf(&mut self) -> CHSEMF_W<'_, FCR3rs> {
        CHSEMF_W::new(self, 15)
    }
    ///Bit 16 - clear the illegal access flag for PKA
    #[inline(always)]
    pub fn cpkaf(&mut self) -> CPKAF_W<'_, FCR3rs> {
        CPKAF_W::new(self, 16)
    }
    ///Bit 22 - clear the illegal access flag for RAMCFG
    #[inline(always)]
    pub fn cramcfgf(&mut self) -> CRAMCFGF_W<'_, FCR3rs> {
        CRAMCFGF_W::new(self, 22)
    }
    ///Bit 23 - clear the illegal access flag for 2.4 GHz RADIO
    #[inline(always)]
    pub fn cradiof(&mut self) -> CRADIOF_W<'_, FCR3rs> {
        CRADIOF_W::new(self, 23)
    }
}
/**TZIC flag clear register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#GTZC1_TZIC:FCR3)*/
pub struct FCR3rs;
impl crate::RegisterSpec for FCR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr3::W`](W) writer structure
impl crate::Writable for FCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR3 to value 0
impl crate::Resettable for FCR3rs {}
