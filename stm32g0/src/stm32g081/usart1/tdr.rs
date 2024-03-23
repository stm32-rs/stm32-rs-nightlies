#[doc = "Register `TDR` reader"]
pub type R = crate::R<TDRrs>;
#[doc = "Register `TDR` writer"]
pub type W = crate::W<TDRrs>;
#[doc = "Field `TDR` reader - Transmit data value"]
pub type TDR_R = crate::FieldReader<u16>;
#[doc = "Field `TDR` writer - Transmit data value"]
pub type TDR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Transmit data value"]
    #[inline(always)]
    pub fn tdr(&self) -> TDR_R {
        TDR_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit data value"]
    #[inline(always)]
    #[must_use]
    pub fn tdr(&mut self) -> TDR_W<TDRrs> {
        TDR_W::new(self, 0)
    }
}
#[doc = "Transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDRrs;
impl crate::RegisterSpec for TDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdr::R`](R) reader structure"]
impl crate::Readable for TDRrs {}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::Resettable for TDRrs {
    const RESET_VALUE: u32 = 0;
}
