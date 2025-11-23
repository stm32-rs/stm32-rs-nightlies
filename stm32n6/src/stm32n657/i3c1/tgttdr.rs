///Register `TGTTDR` reader
pub type R = crate::R<TGTTDRrs>;
///Register `TGTTDR` writer
pub type W = crate::W<TGTTDRrs>;
///Field `TGTTDCNT` reader - Transmit data counter, in bytes (when I3C is configured as target)
pub type TGTTDCNT_R = crate::FieldReader<u16>;
///Field `TGTTDCNT` writer - Transmit data counter, in bytes (when I3C is configured as target)
pub type TGTTDCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PRELOAD` reader - Preload of the TX-FIFO (when I3C is configured as target)
pub type PRELOAD_R = crate::BitReader;
///Field `PRELOAD` writer - Preload of the TX-FIFO (when I3C is configured as target)
pub type PRELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Transmit data counter, in bytes (when I3C is configured as target)
    #[inline(always)]
    pub fn tgttdcnt(&self) -> TGTTDCNT_R {
        TGTTDCNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - Preload of the TX-FIFO (when I3C is configured as target)
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TGTTDR")
            .field("tgttdcnt", &self.tgttdcnt())
            .field("preload", &self.preload())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Transmit data counter, in bytes (when I3C is configured as target)
    #[inline(always)]
    pub fn tgttdcnt(&mut self) -> TGTTDCNT_W<'_, TGTTDRrs> {
        TGTTDCNT_W::new(self, 0)
    }
    ///Bit 16 - Preload of the TX-FIFO (when I3C is configured as target)
    #[inline(always)]
    pub fn preload(&mut self) -> PRELOAD_W<'_, TGTTDRrs> {
        PRELOAD_W::new(self, 16)
    }
}
/**I3C target transmit configuration register

You can [`read`](crate::Reg::read) this register and get [`tgttdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgttdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#I3C1:TGTTDR)*/
pub struct TGTTDRrs;
impl crate::RegisterSpec for TGTTDRrs {
    type Ux = u32;
}
///`read()` method returns [`tgttdr::R`](R) reader structure
impl crate::Readable for TGTTDRrs {}
///`write(|w| ..)` method takes [`tgttdr::W`](W) writer structure
impl crate::Writable for TGTTDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TGTTDR to value 0
impl crate::Resettable for TGTTDRrs {}
