#[doc = "Register `DHR12L1` reader"]
pub type R = crate::R<DHR12L1rs>;
#[doc = "Register `DHR12L1` writer"]
pub type W = crate::W<DHR12L1rs>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
pub type DACC1DHR_R = crate::FieldReader<u16>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
pub type DACC1DHR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
#[doc = "Field `DACC1DHRB` reader - DAC channel1 12-bit left-aligned data B"]
pub type DACC1DHRB_R = crate::FieldReader<u16>;
#[doc = "Field `DACC1DHRB` writer - DAC channel1 12-bit left-aligned data B"]
pub type DACC1DHRB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - DAC channel1 12-bit left-aligned data B"]
    #[inline(always)]
    pub fn dacc1dhrb(&self) -> DACC1DHRB_R {
        DACC1DHRB_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<DHR12L1rs> {
        DACC1DHR_W::new(self, 4)
    }
    #[doc = "Bits 20:31 - DAC channel1 12-bit left-aligned data B"]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhrb(&mut self) -> DACC1DHRB_W<DHR12L1rs> {
        DACC1DHRB_W::new(self, 20)
    }
}
#[doc = "DAC channel1 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr12l1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr12l1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHR12L1rs;
impl crate::RegisterSpec for DHR12L1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr12l1::R`](R) reader structure"]
impl crate::Readable for DHR12L1rs {}
#[doc = "`write(|w| ..)` method takes [`dhr12l1::W`](W) writer structure"]
impl crate::Writable for DHR12L1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DHR12L1 to value 0"]
impl crate::Resettable for DHR12L1rs {
    const RESET_VALUE: u32 = 0;
}
