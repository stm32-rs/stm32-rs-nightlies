#[doc = "Register `FIFOR7` reader"]
pub type R = crate::R<FIFOR7rs>;
#[doc = "Register `FIFOR7` writer"]
pub type W = crate::W<FIFOR7rs>;
#[doc = "Field `FIFODATA` reader - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT = 1). The FIFO data occupies 16 entries of 32-bit words."]
pub type FIFODATA_R = crate::FieldReader<u32>;
#[doc = "Field `FIFODATA` writer - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT = 1). The FIFO data occupies 16 entries of 32-bit words."]
pub type FIFODATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT = 1). The FIFO data occupies 16 entries of 32-bit words."]
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT = 1). The FIFO data occupies 16 entries of 32-bit words."]
    #[inline(always)]
    #[must_use]
    pub fn fifodata(&mut self) -> FIFODATA_W<FIFOR7rs> {
        FIFODATA_W::new(self, 0)
    }
}
#[doc = "SDMMC data FIFO registers 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFOR7rs;
impl crate::RegisterSpec for FIFOR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifor7::R`](R) reader structure"]
impl crate::Readable for FIFOR7rs {}
#[doc = "`write(|w| ..)` method takes [`fifor7::W`](W) writer structure"]
impl crate::Writable for FIFOR7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOR7 to value 0"]
impl crate::Resettable for FIFOR7rs {
    const RESET_VALUE: u32 = 0;
}
