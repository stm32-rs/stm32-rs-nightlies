#[doc = "Register `HSPI_LPTR` reader"]
pub type R = crate::R<HSPI_LPTRrs>;
#[doc = "Register `HSPI_LPTR` writer"]
pub type W = crate::W<HSPI_LPTRrs>;
#[doc = "Field `TIMEOUT` reader - 15: 0\\]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state."]
pub type TIMEOUT_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUT` writer - 15: 0\\]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state."]
pub type TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15: 0\\]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15: 0\\]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state."]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<HSPI_LPTRrs> {
        TIMEOUT_W::new(self, 0)
    }
}
#[doc = "HSPI low-power timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_lptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_lptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSPI_LPTRrs;
impl crate::RegisterSpec for HSPI_LPTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hspi_lptr::R`](R) reader structure"]
impl crate::Readable for HSPI_LPTRrs {}
#[doc = "`write(|w| ..)` method takes [`hspi_lptr::W`](W) writer structure"]
impl crate::Writable for HSPI_LPTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSPI_LPTR to value 0"]
impl crate::Resettable for HSPI_LPTRrs {
    const RESET_VALUE: u32 = 0;
}
