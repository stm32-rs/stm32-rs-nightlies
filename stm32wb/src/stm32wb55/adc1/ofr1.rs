#[doc = "Register `OFR1` reader"]
pub type R = crate::R<OFR1rs>;
#[doc = "Register `OFR1` writer"]
pub type W = crate::W<OFR1rs>;
#[doc = "Field `OFFSET1` reader - ADC offset number 1 offset level"]
pub type OFFSET1_R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET1` writer - ADC offset number 1 offset level"]
pub type OFFSET1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `OFFSET1_CH` reader - ADC offset number 1 channel selection"]
pub type OFFSET1_CH_R = crate::FieldReader;
#[doc = "Field `OFFSET1_CH` writer - ADC offset number 1 channel selection"]
pub type OFFSET1_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OFFSET1_EN` reader - ADC offset number 1 enable"]
pub type OFFSET1_EN_R = crate::BitReader;
#[doc = "Field `OFFSET1_EN` writer - ADC offset number 1 enable"]
pub type OFFSET1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - ADC offset number 1 offset level"]
    #[inline(always)]
    pub fn offset1(&self) -> OFFSET1_R {
        OFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 26:30 - ADC offset number 1 channel selection"]
    #[inline(always)]
    pub fn offset1_ch(&self) -> OFFSET1_CH_R {
        OFFSET1_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - ADC offset number 1 enable"]
    #[inline(always)]
    pub fn offset1_en(&self) -> OFFSET1_EN_R {
        OFFSET1_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC offset number 1 offset level"]
    #[inline(always)]
    #[must_use]
    pub fn offset1(&mut self) -> OFFSET1_W<OFR1rs> {
        OFFSET1_W::new(self, 0)
    }
    #[doc = "Bits 26:30 - ADC offset number 1 channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn offset1_ch(&mut self) -> OFFSET1_CH_W<OFR1rs> {
        OFFSET1_CH_W::new(self, 26)
    }
    #[doc = "Bit 31 - ADC offset number 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn offset1_en(&mut self) -> OFFSET1_EN_W<OFR1rs> {
        OFFSET1_EN_W::new(self, 31)
    }
}
#[doc = "ADC offset number 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ofr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OFR1rs;
impl crate::RegisterSpec for OFR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofr1::R`](R) reader structure"]
impl crate::Readable for OFR1rs {}
#[doc = "`write(|w| ..)` method takes [`ofr1::W`](W) writer structure"]
impl crate::Writable for OFR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OFR1 to value 0"]
impl crate::Resettable for OFR1rs {
    const RESET_VALUE: u32 = 0;
}
