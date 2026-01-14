///Register `CONFR6` reader
pub type R = crate::R<CONFR6rs>;
///Register `CONFR6` writer
pub type W = crate::W<CONFR6rs>;
///Field `HD` reader - Huffman DC
pub type HD_R = crate::BitReader;
///Field `HD` writer - Huffman DC
pub type HD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HA` reader - Huffman AC
pub type HA_R = crate::BitReader;
///Field `HA` writer - Huffman AC
pub type HA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `QT` reader - Quantization Table
pub type QT_R = crate::FieldReader;
///Field `QT` writer - Quantization Table
pub type QT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NB` reader - Number of Block
pub type NB_R = crate::FieldReader;
///Field `NB` writer - Number of Block
pub type NB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `VSF` reader - Vertical Sampling Factor
pub type VSF_R = crate::FieldReader;
///Field `VSF` writer - Vertical Sampling Factor
pub type VSF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HSF` reader - Horizontal Sampling Factor
pub type HSF_R = crate::FieldReader;
///Field `HSF` writer - Horizontal Sampling Factor
pub type HSF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - Huffman DC
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Huffman AC
    #[inline(always)]
    pub fn ha(&self) -> HA_R {
        HA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Quantization Table
    #[inline(always)]
    pub fn qt(&self) -> QT_R {
        QT_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Number of Block
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Vertical Sampling Factor
    #[inline(always)]
    pub fn vsf(&self) -> VSF_R {
        VSF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Horizontal Sampling Factor
    #[inline(always)]
    pub fn hsf(&self) -> HSF_R {
        HSF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFR6")
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
    ///Bit 0 - Huffman DC
    #[inline(always)]
    pub fn hd(&mut self) -> HD_W<'_, CONFR6rs> {
        HD_W::new(self, 0)
    }
    ///Bit 1 - Huffman AC
    #[inline(always)]
    pub fn ha(&mut self) -> HA_W<'_, CONFR6rs> {
        HA_W::new(self, 1)
    }
    ///Bits 2:3 - Quantization Table
    #[inline(always)]
    pub fn qt(&mut self) -> QT_W<'_, CONFR6rs> {
        QT_W::new(self, 2)
    }
    ///Bits 4:7 - Number of Block
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W<'_, CONFR6rs> {
        NB_W::new(self, 4)
    }
    ///Bits 8:11 - Vertical Sampling Factor
    #[inline(always)]
    pub fn vsf(&mut self) -> VSF_W<'_, CONFR6rs> {
        VSF_W::new(self, 8)
    }
    ///Bits 12:15 - Horizontal Sampling Factor
    #[inline(always)]
    pub fn hsf(&mut self) -> HSF_W<'_, CONFR6rs> {
        HSF_W::new(self, 12)
    }
}
/**JPEG codec configuration register 6

You can [`read`](crate::Reg::read) this register and get [`confr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:CONFR6)*/
pub struct CONFR6rs;
impl crate::RegisterSpec for CONFR6rs {
    type Ux = u32;
}
///`read()` method returns [`confr6::R`](R) reader structure
impl crate::Readable for CONFR6rs {}
///`write(|w| ..)` method takes [`confr6::W`](W) writer structure
impl crate::Writable for CONFR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONFR6 to value 0
impl crate::Resettable for CONFR6rs {}
