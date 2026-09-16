///Register `FCR` writer
pub type W = crate::W<FCRrs>;
///Field `RCHECKF` writer - Root code check flag clear Set this bit to clear RCHECKF bit in FLASH_SR.
pub type RCHECKF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 25 - Root code check flag clear Set this bit to clear RCHECKF bit in FLASH_SR.
    #[inline(always)]
    pub fn rcheckf(&mut self) -> RCHECKF_W<'_, FCRrs> {
        RCHECKF_W::new(self, 25)
    }
}
/**FLASH status register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:FCR)*/
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
