///Register `CONFRN1` reader
pub type R = crate::R<CONFRN1rs>;
///Register `CONFRN1` writer
pub type W = crate::W<CONFRN1rs>;
///Field `HD` reader - Huffman DC Selects the Huffman table for encoding the DC coefficients.
pub type HD_R = crate::BitReader;
///Field `HD` writer - Huffman DC Selects the Huffman table for encoding the DC coefficients.
pub type HD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HA` reader - Huffman AC Selects the Huffman table for encoding the AC coefficients.
pub type HA_R = crate::BitReader;
///Field `HA` writer - Huffman AC Selects the Huffman table for encoding the AC coefficients.
pub type HA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `QT` reader - Quantization Table Selects quantization table associated with a color component.
pub type QT_R = crate::FieldReader;
///Field `QT` writer - Quantization Table Selects quantization table associated with a color component.
pub type QT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NB` reader - Number of Block Number of data units minus 1 that belong to a particular color in the MCU.
pub type NB_R = crate::FieldReader;
///Field `NB` writer - Number of Block Number of data units minus 1 that belong to a particular color in the MCU.
pub type NB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `VSF` reader - Vertical Sampling Factor Vertical sampling factor for component i.
pub type VSF_R = crate::FieldReader;
///Field `VSF` writer - Vertical Sampling Factor Vertical sampling factor for component i.
pub type VSF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HSF` reader - Horizontal Sampling Factor Horizontal sampling factor for component i.
pub type HSF_R = crate::FieldReader;
///Field `HSF` writer - Horizontal Sampling Factor Horizontal sampling factor for component i.
pub type HSF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - Huffman DC Selects the Huffman table for encoding the DC coefficients.
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Huffman AC Selects the Huffman table for encoding the AC coefficients.
    #[inline(always)]
    pub fn ha(&self) -> HA_R {
        HA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Quantization Table Selects quantization table associated with a color component.
    #[inline(always)]
    pub fn qt(&self) -> QT_R {
        QT_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Number of Block Number of data units minus 1 that belong to a particular color in the MCU.
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Vertical Sampling Factor Vertical sampling factor for component i.
    #[inline(always)]
    pub fn vsf(&self) -> VSF_R {
        VSF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Horizontal Sampling Factor Horizontal sampling factor for component i.
    #[inline(always)]
    pub fn hsf(&self) -> HSF_R {
        HSF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFRN1")
            .field("hd", &self.hd())
            .field("ha", &self.ha())
            .field("qt", &self.qt())
            .field("nb", &self.nb())
            .field("vsf", &self.vsf())
            .field("hsf", &self.hsf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Huffman DC Selects the Huffman table for encoding the DC coefficients.
    #[inline(always)]
    pub fn hd(&mut self) -> HD_W<'_, CONFRN1rs> {
        HD_W::new(self, 0)
    }
    ///Bit 1 - Huffman AC Selects the Huffman table for encoding the AC coefficients.
    #[inline(always)]
    pub fn ha(&mut self) -> HA_W<'_, CONFRN1rs> {
        HA_W::new(self, 1)
    }
    ///Bits 2:3 - Quantization Table Selects quantization table associated with a color component.
    #[inline(always)]
    pub fn qt(&mut self) -> QT_W<'_, CONFRN1rs> {
        QT_W::new(self, 2)
    }
    ///Bits 4:7 - Number of Block Number of data units minus 1 that belong to a particular color in the MCU.
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W<'_, CONFRN1rs> {
        NB_W::new(self, 4)
    }
    ///Bits 8:11 - Vertical Sampling Factor Vertical sampling factor for component i.
    #[inline(always)]
    pub fn vsf(&mut self) -> VSF_W<'_, CONFRN1rs> {
        VSF_W::new(self, 8)
    }
    ///Bits 12:15 - Horizontal Sampling Factor Horizontal sampling factor for component i.
    #[inline(always)]
    pub fn hsf(&mut self) -> HSF_W<'_, CONFRN1rs> {
        HSF_W::new(self, 12)
    }
}
/**JPEG codec configuration register 4-7

You can [`read`](crate::Reg::read) this register and get [`confrn1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confrn1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#JPEG:CONFRN1)*/
pub struct CONFRN1rs;
impl crate::RegisterSpec for CONFRN1rs {
    type Ux = u32;
}
///`read()` method returns [`confrn1::R`](R) reader structure
impl crate::Readable for CONFRN1rs {}
///`write(|w| ..)` method takes [`confrn1::W`](W) writer structure
impl crate::Writable for CONFRN1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONFRN1 to value 0
impl crate::Resettable for CONFRN1rs {}
