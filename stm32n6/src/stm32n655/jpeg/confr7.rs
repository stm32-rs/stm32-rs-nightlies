///Register `CONFR7` reader
pub type R = crate::R<CONFR7rs>;
///Register `CONFR7` writer
pub type W = crate::W<CONFR7rs>;
///Field `HD` reader - Huffman DC
pub type HD_R = crate::BitReader;
///Field `HD` writer - Huffman DC
pub type HD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HA` reader - Huffman AC
pub type HA_R = crate::BitReader;
///Field `HA` writer - Huffman AC
pub type HA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `QT` reader - Quantization table
pub type QT_R = crate::FieldReader;
///Field `QT` writer - Quantization table
pub type QT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NB` reader - Number of blocks
pub type NB_R = crate::FieldReader;
///Field `NB` writer - Number of blocks
pub type NB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `VSF` reader - Vertical sampling factor
pub type VSF_R = crate::FieldReader;
///Field `VSF` writer - Vertical sampling factor
pub type VSF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HSF` reader - Horizontal sampling factor
pub type HSF_R = crate::FieldReader;
///Field `HSF` writer - Horizontal sampling factor
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
    ///Bits 2:3 - Quantization table
    #[inline(always)]
    pub fn qt(&self) -> QT_R {
        QT_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Number of blocks
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Vertical sampling factor
    #[inline(always)]
    pub fn vsf(&self) -> VSF_R {
        VSF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Horizontal sampling factor
    #[inline(always)]
    pub fn hsf(&self) -> HSF_R {
        HSF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFR7")
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
    pub fn hd(&mut self) -> HD_W<'_, CONFR7rs> {
        HD_W::new(self, 0)
    }
    ///Bit 1 - Huffman AC
    #[inline(always)]
    pub fn ha(&mut self) -> HA_W<'_, CONFR7rs> {
        HA_W::new(self, 1)
    }
    ///Bits 2:3 - Quantization table
    #[inline(always)]
    pub fn qt(&mut self) -> QT_W<'_, CONFR7rs> {
        QT_W::new(self, 2)
    }
    ///Bits 4:7 - Number of blocks
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W<'_, CONFR7rs> {
        NB_W::new(self, 4)
    }
    ///Bits 8:11 - Vertical sampling factor
    #[inline(always)]
    pub fn vsf(&mut self) -> VSF_W<'_, CONFR7rs> {
        VSF_W::new(self, 8)
    }
    ///Bits 12:15 - Horizontal sampling factor
    #[inline(always)]
    pub fn hsf(&mut self) -> HSF_W<'_, CONFR7rs> {
        HSF_W::new(self, 12)
    }
}
/**JPEG codec configuration register 7

You can [`read`](crate::Reg::read) this register and get [`confr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:CONFR7)*/
pub struct CONFR7rs;
impl crate::RegisterSpec for CONFR7rs {
    type Ux = u32;
}
///`read()` method returns [`confr7::R`](R) reader structure
impl crate::Readable for CONFR7rs {}
///`write(|w| ..)` method takes [`confr7::W`](W) writer structure
impl crate::Writable for CONFR7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONFR7 to value 0
impl crate::Resettable for CONFR7rs {}
