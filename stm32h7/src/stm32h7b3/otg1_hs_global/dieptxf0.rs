#[doc = "Register `DIEPTXF0` reader"]
pub type R = crate::R<DIEPTXF0rs>;
#[doc = "Register `DIEPTXF0` writer"]
pub type W = crate::W<DIEPTXF0rs>;
#[doc = "Field `TX0FSA` reader - Endpoint 0 transmit RAM start address"]
pub type TX0FSA_R = crate::FieldReader<u16>;
#[doc = "Field `TX0FSA` writer - Endpoint 0 transmit RAM start address"]
pub type TX0FSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TX0FD` reader - Endpoint 0 TxFIFO depth"]
pub type TX0FD_R = crate::FieldReader<u16>;
#[doc = "Field `TX0FD` writer - Endpoint 0 TxFIFO depth"]
pub type TX0FD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&self) -> TX0FSA_R {
        TX0FSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&self) -> TX0FD_R {
        TX0FD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn tx0fsa(&mut self) -> TX0FSA_W<DIEPTXF0rs> {
        TX0FSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn tx0fd(&mut self) -> TX0FD_W<DIEPTXF0rs> {
        TX0FD_W::new(self, 16)
    }
}
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF0rs;
impl crate::RegisterSpec for DIEPTXF0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf0::R`](R) reader structure"]
impl crate::Readable for DIEPTXF0rs {}
#[doc = "`write(|w| ..)` method takes [`dieptxf0::W`](W) writer structure"]
impl crate::Writable for DIEPTXF0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTXF0 to value 0x0200"]
impl crate::Resettable for DIEPTXF0rs {
    const RESET_VALUE: u32 = 0x0200;
}
