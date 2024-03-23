#[doc = "Register `CONFRN1` reader"]
pub type R = crate::R<CONFRN1rs>;
#[doc = "Register `CONFRN1` writer"]
pub type W = crate::W<CONFRN1rs>;
#[doc = "Field `HD` reader - Huffman DC Selects the Huffman table for encoding the DC coefficients."]
pub type HD_R = crate::BitReader;
#[doc = "Field `HD` writer - Huffman DC Selects the Huffman table for encoding the DC coefficients."]
pub type HD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HA` reader - Huffman AC Selects the Huffman table for encoding the AC coefficients."]
pub type HA_R = crate::BitReader;
#[doc = "Field `HA` writer - Huffman AC Selects the Huffman table for encoding the AC coefficients."]
pub type HA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QT` reader - Quantization Table Selects quantization table associated with a color component."]
pub type QT_R = crate::FieldReader;
#[doc = "Field `QT` writer - Quantization Table Selects quantization table associated with a color component."]
pub type QT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NB` reader - Number of Block Number of data units minus 1 that belong to a particular color in the MCU."]
pub type NB_R = crate::FieldReader;
#[doc = "Field `NB` writer - Number of Block Number of data units minus 1 that belong to a particular color in the MCU."]
pub type NB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VSF` reader - Vertical Sampling Factor Vertical sampling factor for component i."]
pub type VSF_R = crate::FieldReader;
#[doc = "Field `VSF` writer - Vertical Sampling Factor Vertical sampling factor for component i."]
pub type VSF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HSF` reader - Horizontal Sampling Factor Horizontal sampling factor for component i."]
pub type HSF_R = crate::FieldReader;
#[doc = "Field `HSF` writer - Horizontal Sampling Factor Horizontal sampling factor for component i."]
pub type HSF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Huffman DC Selects the Huffman table for encoding the DC coefficients."]
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Huffman AC Selects the Huffman table for encoding the AC coefficients."]
    #[inline(always)]
    pub fn ha(&self) -> HA_R {
        HA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Quantization Table Selects quantization table associated with a color component."]
    #[inline(always)]
    pub fn qt(&self) -> QT_R {
        QT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Number of Block Number of data units minus 1 that belong to a particular color in the MCU."]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Vertical Sampling Factor Vertical sampling factor for component i."]
    #[inline(always)]
    pub fn vsf(&self) -> VSF_R {
        VSF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Horizontal Sampling Factor Horizontal sampling factor for component i."]
    #[inline(always)]
    pub fn hsf(&self) -> HSF_R {
        HSF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Huffman DC Selects the Huffman table for encoding the DC coefficients."]
    #[inline(always)]
    #[must_use]
    pub fn hd(&mut self) -> HD_W<CONFRN1rs> {
        HD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Huffman AC Selects the Huffman table for encoding the AC coefficients."]
    #[inline(always)]
    #[must_use]
    pub fn ha(&mut self) -> HA_W<CONFRN1rs> {
        HA_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Quantization Table Selects quantization table associated with a color component."]
    #[inline(always)]
    #[must_use]
    pub fn qt(&mut self) -> QT_W<CONFRN1rs> {
        QT_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - Number of Block Number of data units minus 1 that belong to a particular color in the MCU."]
    #[inline(always)]
    #[must_use]
    pub fn nb(&mut self) -> NB_W<CONFRN1rs> {
        NB_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Vertical Sampling Factor Vertical sampling factor for component i."]
    #[inline(always)]
    #[must_use]
    pub fn vsf(&mut self) -> VSF_W<CONFRN1rs> {
        VSF_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Horizontal Sampling Factor Horizontal sampling factor for component i."]
    #[inline(always)]
    #[must_use]
    pub fn hsf(&mut self) -> HSF_W<CONFRN1rs> {
        HSF_W::new(self, 12)
    }
}
#[doc = "JPEG codec configuration register 4-7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`confrn1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confrn1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFRN1rs;
impl crate::RegisterSpec for CONFRN1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`confrn1::R`](R) reader structure"]
impl crate::Readable for CONFRN1rs {}
#[doc = "`write(|w| ..)` method takes [`confrn1::W`](W) writer structure"]
impl crate::Writable for CONFRN1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFRN1 to value 0"]
impl crate::Resettable for CONFRN1rs {
    const RESET_VALUE: u32 = 0;
}
