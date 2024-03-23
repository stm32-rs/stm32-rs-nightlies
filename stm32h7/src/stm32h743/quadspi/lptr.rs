#[doc = "Register `LPTR` reader"]
pub type R = crate::R<LPTRrs>;
#[doc = "Register `LPTR` writer"]
pub type W = crate::W<LPTRrs>;
#[doc = "Field `TIMEOUT` reader - Timeout period After each access in memory-mapped mode, the QUADSPI prefetches the subsequent bytes and holds these bytes in the FIFO. This field indicates how many CLK cycles the QUADSPI waits after the FIFO becomes full until it raises nCS, putting the Flash memory in a lower-consumption state. This field can be written only when BUSY = 0."]
pub type TIMEOUT_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUT` writer - Timeout period After each access in memory-mapped mode, the QUADSPI prefetches the subsequent bytes and holds these bytes in the FIFO. This field indicates how many CLK cycles the QUADSPI waits after the FIFO becomes full until it raises nCS, putting the Flash memory in a lower-consumption state. This field can be written only when BUSY = 0."]
pub type TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timeout period After each access in memory-mapped mode, the QUADSPI prefetches the subsequent bytes and holds these bytes in the FIFO. This field indicates how many CLK cycles the QUADSPI waits after the FIFO becomes full until it raises nCS, putting the Flash memory in a lower-consumption state. This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout period After each access in memory-mapped mode, the QUADSPI prefetches the subsequent bytes and holds these bytes in the FIFO. This field indicates how many CLK cycles the QUADSPI waits after the FIFO becomes full until it raises nCS, putting the Flash memory in a lower-consumption state. This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<LPTRrs> {
        TIMEOUT_W::new(self, 0)
    }
}
#[doc = "QUADSPI low-power timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPTRrs;
impl crate::RegisterSpec for LPTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptr::R`](R) reader structure"]
impl crate::Readable for LPTRrs {}
#[doc = "`write(|w| ..)` method takes [`lptr::W`](W) writer structure"]
impl crate::Writable for LPTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTR to value 0"]
impl crate::Resettable for LPTRrs {
    const RESET_VALUE: u32 = 0;
}
