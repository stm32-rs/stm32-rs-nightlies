///Register `FMAC_WDATA` writer
pub type W = crate::W<FMAC_WDATArs>;
///Field `WDATA` writer - Write data When a write access to this register occurs, the write data are transferred to the address offset indicated by the write pointer. The pointer address is automatically incremented after each write access.
pub type WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<FMAC_WDATArs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - Write data When a write access to this register occurs, the write data are transferred to the address offset indicated by the write pointer. The pointer address is automatically incremented after each write access.
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W<FMAC_WDATArs> {
        WDATA_W::new(self, 0)
    }
}
/**FMAC write data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmac_wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FMAC:FMAC_WDATA)*/
pub struct FMAC_WDATArs;
impl crate::RegisterSpec for FMAC_WDATArs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fmac_wdata::W`](W) writer structure
impl crate::Writable for FMAC_WDATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FMAC_WDATA to value 0
impl crate::Resettable for FMAC_WDATArs {
    const RESET_VALUE: u32 = 0;
}
