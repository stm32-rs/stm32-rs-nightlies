///Register `CSQICR` writer
pub type W = crate::W<CSQICRrs>;
///Field `CTCF` writer - CTCF
pub type CTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSCF` writer - CSCF
pub type CSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSEF` writer - CSEF
pub type CSEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSUEF` writer - CSUEF
pub type CSUEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCMDTCF` writer - CCMDTCF
pub type CCMDTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CSQICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CTCF
    #[inline(always)]
    pub fn ctcf(&mut self) -> CTCF_W<'_, CSQICRrs> {
        CTCF_W::new(self, 0)
    }
    ///Bit 1 - CSCF
    #[inline(always)]
    pub fn cscf(&mut self) -> CSCF_W<'_, CSQICRrs> {
        CSCF_W::new(self, 1)
    }
    ///Bit 2 - CSEF
    #[inline(always)]
    pub fn csef(&mut self) -> CSEF_W<'_, CSQICRrs> {
        CSEF_W::new(self, 2)
    }
    ///Bit 3 - CSUEF
    #[inline(always)]
    pub fn csuef(&mut self) -> CSUEF_W<'_, CSQICRrs> {
        CSUEF_W::new(self, 3)
    }
    ///Bit 4 - CCMDTCF
    #[inline(always)]
    pub fn ccmdtcf(&mut self) -> CCMDTCF_W<'_, CSQICRrs> {
        CCMDTCF_W::new(self, 4)
    }
}
/**FMC NAND Command Sequencer Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FMC:CSQICR)*/
pub struct CSQICRrs;
impl crate::RegisterSpec for CSQICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`csqicr::W`](W) writer structure
impl crate::Writable for CSQICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSQICR to value 0
impl crate::Resettable for CSQICRrs {}
