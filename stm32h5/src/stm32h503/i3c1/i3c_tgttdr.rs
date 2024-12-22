///Register `I3C_TGTTDR` reader
pub type R = crate::R<I3C_TGTTDRrs>;
///Register `I3C_TGTTDR` writer
pub type W = crate::W<I3C_TGTTDRrs>;
///Field `TGTTDCNT` reader - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO.
pub type TGTTDCNT_R = crate::FieldReader<u16>;
///Field `TGTTDCNT` writer - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO.
pub type TGTTDCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PRELOAD` reader - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO.
pub type PRELOAD_R = crate::BitReader;
///Field `PRELOAD` writer - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO.
pub type PRELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO.
    #[inline(always)]
    pub fn tgttdcnt(&self) -> TGTTDCNT_R {
        TGTTDCNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO.
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_TGTTDR")
            .field("tgttdcnt", &self.tgttdcnt())
            .field("preload", &self.preload())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO.
    #[inline(always)]
    pub fn tgttdcnt(&mut self) -> TGTTDCNT_W<I3C_TGTTDRrs> {
        TGTTDCNT_W::new(self, 0)
    }
    ///Bit 16 - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO.
    #[inline(always)]
    pub fn preload(&mut self) -> PRELOAD_W<I3C_TGTTDRrs> {
        PRELOAD_W::new(self, 16)
    }
}
/**I3C target transmit configuration register

You can [`read`](crate::Reg::read) this register and get [`i3c_tgttdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_tgttdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_TGTTDR)*/
pub struct I3C_TGTTDRrs;
impl crate::RegisterSpec for I3C_TGTTDRrs {
    type Ux = u32;
}
///`read()` method returns [`i3c_tgttdr::R`](R) reader structure
impl crate::Readable for I3C_TGTTDRrs {}
///`write(|w| ..)` method takes [`i3c_tgttdr::W`](W) writer structure
impl crate::Writable for I3C_TGTTDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I3C_TGTTDR to value 0
impl crate::Resettable for I3C_TGTTDRrs {
    const RESET_VALUE: u32 = 0;
}
