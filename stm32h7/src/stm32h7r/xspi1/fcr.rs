///Register `FCR` writer
pub type W = crate::W<FCRrs>;
///Field `CTEF` writer - Clear transfer error flag Writing 1 clears the TEF flag in the XSPI_SR register.
pub type CTEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCF` writer - Clear transfer complete flag Writing 1 clears the TCF flag in the XSPI_SR register.
pub type CTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSMF` writer - Clear status match flag Writing 1 clears the SMF flag in the XSPI_SR register.
pub type CSMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTOF` writer - Clear timeout flag Writing 1 clears the TOF flag in the XSPI_SR register.
pub type CTOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear transfer error flag Writing 1 clears the TEF flag in the XSPI_SR register.
    #[inline(always)]
    pub fn ctef(&mut self) -> CTEF_W<FCRrs> {
        CTEF_W::new(self, 0)
    }
    ///Bit 1 - Clear transfer complete flag Writing 1 clears the TCF flag in the XSPI_SR register.
    #[inline(always)]
    pub fn ctcf(&mut self) -> CTCF_W<FCRrs> {
        CTCF_W::new(self, 1)
    }
    ///Bit 3 - Clear status match flag Writing 1 clears the SMF flag in the XSPI_SR register.
    #[inline(always)]
    pub fn csmf(&mut self) -> CSMF_W<FCRrs> {
        CSMF_W::new(self, 3)
    }
    ///Bit 4 - Clear timeout flag Writing 1 clears the TOF flag in the XSPI_SR register.
    #[inline(always)]
    pub fn ctof(&mut self) -> CTOF_W<FCRrs> {
        CTOF_W::new(self, 4)
    }
}
/**XSPI flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#XSPI1:FCR)*/
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr::W`](W) writer structure
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR to value 0
impl crate::Resettable for FCRrs {}
