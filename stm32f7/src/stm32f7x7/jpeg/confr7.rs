#[doc = "Register `CONFR7` reader"]
pub type R = crate::R<CONFR7rs>;
#[doc = "Register `CONFR7` writer"]
pub type W = crate::W<CONFR7rs>;
#[doc = "Field `HD` reader - Huffman DC"]
pub type HD_R = crate::BitReader;
#[doc = "Field `HD` writer - Huffman DC"]
pub type HD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HA` reader - Huffman AC"]
pub type HA_R = crate::BitReader;
#[doc = "Field `HA` writer - Huffman AC"]
pub type HA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QT` reader - Quantization Table"]
pub type QT_R = crate::FieldReader;
#[doc = "Field `QT` writer - Quantization Table"]
pub type QT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NB` reader - Number of Block"]
pub type NB_R = crate::FieldReader;
#[doc = "Field `NB` writer - Number of Block"]
pub type NB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VSF` reader - Vertical Sampling Factor"]
pub type VSF_R = crate::FieldReader;
#[doc = "Field `VSF` writer - Vertical Sampling Factor"]
pub type VSF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HSF` reader - Horizontal Sampling Factor"]
pub type HSF_R = crate::FieldReader;
#[doc = "Field `HSF` writer - Horizontal Sampling Factor"]
pub type HSF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Huffman DC"]
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Huffman AC"]
    #[inline(always)]
    pub fn ha(&self) -> HA_R {
        HA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Quantization Table"]
    #[inline(always)]
    pub fn qt(&self) -> QT_R {
        QT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Number of Block"]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Vertical Sampling Factor"]
    #[inline(always)]
    pub fn vsf(&self) -> VSF_R {
        VSF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Horizontal Sampling Factor"]
    #[inline(always)]
    pub fn hsf(&self) -> HSF_R {
        HSF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Huffman DC"]
    #[inline(always)]
    #[must_use]
    pub fn hd(&mut self) -> HD_W<CONFR7rs> {
        HD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Huffman AC"]
    #[inline(always)]
    #[must_use]
    pub fn ha(&mut self) -> HA_W<CONFR7rs> {
        HA_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Quantization Table"]
    #[inline(always)]
    #[must_use]
    pub fn qt(&mut self) -> QT_W<CONFR7rs> {
        QT_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - Number of Block"]
    #[inline(always)]
    #[must_use]
    pub fn nb(&mut self) -> NB_W<CONFR7rs> {
        NB_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Vertical Sampling Factor"]
    #[inline(always)]
    #[must_use]
    pub fn vsf(&mut self) -> VSF_W<CONFR7rs> {
        VSF_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Horizontal Sampling Factor"]
    #[inline(always)]
    #[must_use]
    pub fn hsf(&mut self) -> HSF_W<CONFR7rs> {
        HSF_W::new(self, 12)
    }
}
#[doc = "JPEG codec configuration register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`confr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFR7rs;
impl crate::RegisterSpec for CONFR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`confr7::R`](R) reader structure"]
impl crate::Readable for CONFR7rs {}
#[doc = "`write(|w| ..)` method takes [`confr7::W`](W) writer structure"]
impl crate::Writable for CONFR7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFR7 to value 0"]
impl crate::Resettable for CONFR7rs {
    const RESET_VALUE: u32 = 0;
}
