#[doc = "Register `DHR12RD` reader"]
pub type R = crate::R<DHR12RDrs>;
#[doc = "Register `DHR12RD` writer"]
pub type W = crate::W<DHR12RDrs>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 12-bit right-aligned data"]
pub type DACC1DHR_R = crate::FieldReader<u16>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 12-bit right-aligned data"]
pub type DACC1DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 12-bit right-aligned data"]
pub type DACC2DHR_R = crate::FieldReader<u16>;
#[doc = "Field `DACC2DHR` writer - DAC channel2 12-bit right-aligned data"]
pub type DACC2DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<DHR12RDrs> {
        DACC1DHR_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<DHR12RDrs> {
        DACC2DHR_W::new(self, 16)
    }
}
#[doc = "Dual DAC 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr12rd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr12rd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHR12RDrs;
impl crate::RegisterSpec for DHR12RDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr12rd::R`](R) reader structure"]
impl crate::Readable for DHR12RDrs {}
#[doc = "`write(|w| ..)` method takes [`dhr12rd::W`](W) writer structure"]
impl crate::Writable for DHR12RDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DHR12RD to value 0"]
impl crate::Resettable for DHR12RDrs {
    const RESET_VALUE: u32 = 0;
}
