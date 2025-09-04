///Register `WDATA` writer
pub type W = crate::W<WDATArs>;
///Field `WDATA` writer - Write data When a write access to this register occurs, the write data are transferred to the address offset indicated by the write pointer. The pointer address is automatically incremented after each write access.
pub type WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<WDATArs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - Write data When a write access to this register occurs, the write data are transferred to the address offset indicated by the write pointer. The pointer address is automatically incremented after each write access.
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W<WDATArs> {
        WDATA_W::new(self, 0)
    }
}
/**FMAC write data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FMAC:WDATA)*/
pub struct WDATArs;
impl crate::RegisterSpec for WDATArs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`wdata::W`](W) writer structure
impl crate::Writable for WDATArs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDATA to value 0
impl crate::Resettable for WDATArs {}
