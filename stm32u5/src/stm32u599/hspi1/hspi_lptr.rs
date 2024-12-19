///Register `HSPI_LPTR` reader
pub type R = crate::R<HSPI_LPTRrs>;
///Register `HSPI_LPTR` writer
pub type W = crate::W<HSPI_LPTRrs>;
///Field `TIMEOUT` reader - 15: 0\]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state.
pub type TIMEOUT_R = crate::FieldReader<u16>;
///Field `TIMEOUT` writer - 15: 0\]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state.
pub type TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - 15: 0\]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state.
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSPI_LPTR")
            .field("timeout", &self.timeout())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - 15: 0\]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state.
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<HSPI_LPTRrs> {
        TIMEOUT_W::new(self, 0)
    }
}
/**HSPI low-power timeout register

You can [`read`](crate::Reg::read) this register and get [`hspi_lptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspi_lptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:HSPI_LPTR)*/
pub struct HSPI_LPTRrs;
impl crate::RegisterSpec for HSPI_LPTRrs {
    type Ux = u32;
}
///`read()` method returns [`hspi_lptr::R`](R) reader structure
impl crate::Readable for HSPI_LPTRrs {}
///`write(|w| ..)` method takes [`hspi_lptr::W`](W) writer structure
impl crate::Writable for HSPI_LPTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HSPI_LPTR to value 0
impl crate::Resettable for HSPI_LPTRrs {
    const RESET_VALUE: u32 = 0;
}
