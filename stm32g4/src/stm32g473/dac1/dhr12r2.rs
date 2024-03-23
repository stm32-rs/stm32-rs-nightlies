#[doc = "Register `DHR12R2` reader"]
pub type R = crate::R<DHR12R2rs>;
#[doc = "Register `DHR12R2` writer"]
pub type W = crate::W<DHR12R2rs>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
pub type DACC2DHR_R = crate::FieldReader<u16>;
#[doc = "Field `DACC2DHR` writer - DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
pub type DACC2DHR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
#[doc = "Field `DACC2DHRB` reader - DAC channel2 12-bit right-aligned data"]
pub type DACC2DHRB_R = crate::FieldReader<u16>;
#[doc = "Field `DACC2DHRB` writer - DAC channel2 12-bit right-aligned data"]
pub type DACC2DHRB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhrb(&self) -> DACC2DHRB_R {
        DACC2DHRB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
    #[inline(always)]
    #[must_use]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<DHR12R2rs> {
        DACC2DHR_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dacc2dhrb(&mut self) -> DACC2DHRB_W<DHR12R2rs> {
        DACC2DHRB_W::new(self, 16)
    }
}
#[doc = "DAC channel2 12-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr12r2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr12r2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHR12R2rs;
impl crate::RegisterSpec for DHR12R2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr12r2::R`](R) reader structure"]
impl crate::Readable for DHR12R2rs {}
#[doc = "`write(|w| ..)` method takes [`dhr12r2::W`](W) writer structure"]
impl crate::Writable for DHR12R2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DHR12R2 to value 0"]
impl crate::Resettable for DHR12R2rs {
    const RESET_VALUE: u32 = 0;
}
