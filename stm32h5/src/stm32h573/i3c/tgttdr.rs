#[doc = "Register `TGTTDR` reader"]
pub type R = crate::R<TGTTDRrs>;
#[doc = "Register `TGTTDR` writer"]
pub type W = crate::W<TGTTDRrs>;
#[doc = "Field `TGTTDCNT` reader - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO."]
pub type TGTTDCNT_R = crate::FieldReader<u16>;
#[doc = "Field `TGTTDCNT` writer - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO."]
pub type TGTTDCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PRELOAD` reader - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO."]
pub type PRELOAD_R = crate::BitReader;
#[doc = "Field `PRELOAD` writer - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO."]
pub type PRELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO."]
    #[inline(always)]
    pub fn tgttdcnt(&self) -> TGTTDCNT_R {
        TGTTDCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO."]
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tgttdcnt(&mut self) -> TGTTDCNT_W<TGTTDRrs> {
        TGTTDCNT_W::new(self, 0)
    }
    #[doc = "Bit 16 - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn preload(&mut self) -> PRELOAD_W<TGTTDRrs> {
        PRELOAD_W::new(self, 16)
    }
}
#[doc = "I3C target transmit configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tgttdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tgttdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TGTTDRrs;
impl crate::RegisterSpec for TGTTDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tgttdr::R`](R) reader structure"]
impl crate::Readable for TGTTDRrs {}
#[doc = "`write(|w| ..)` method takes [`tgttdr::W`](W) writer structure"]
impl crate::Writable for TGTTDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TGTTDR to value 0"]
impl crate::Resettable for TGTTDRrs {
    const RESET_VALUE: u32 = 0;
}
