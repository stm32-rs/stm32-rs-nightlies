///Register `HUFFENC_DC1_5` reader
pub type R = crate::R<HUFFENC_DC1_5rs>;
///Register `HUFFENC_DC1_5` writer
pub type W = crate::W<HUFFENC_DC1_5rs>;
///Field `HCODE10` reader - Huffman code 10
pub type HCODE10_R = crate::FieldReader;
///Field `HCODE10` writer - Huffman code 10
pub type HCODE10_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN10` reader - Huffman length 10
pub type HLEN10_R = crate::FieldReader;
///Field `HLEN10` writer - Huffman length 10
pub type HLEN10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE11` reader - Huffman code 11
pub type HCODE11_R = crate::FieldReader;
///Field `HCODE11` writer - Huffman code 11
pub type HCODE11_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN11` reader - Huffman length 11
pub type HLEN11_R = crate::FieldReader;
///Field `HLEN11` writer - Huffman length 11
pub type HLEN11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 10
    #[inline(always)]
    pub fn hcode10(&self) -> HCODE10_R {
        HCODE10_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 10
    #[inline(always)]
    pub fn hlen10(&self) -> HLEN10_R {
        HLEN10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 11
    #[inline(always)]
    pub fn hcode11(&self) -> HCODE11_R {
        HCODE11_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 11
    #[inline(always)]
    pub fn hlen11(&self) -> HLEN11_R {
        HLEN11_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_DC1_5")
            .field("hcode10", &self.hcode10())
            .field("hlen10", &self.hlen10())
            .field("hcode11", &self.hcode11())
            .field("hlen11", &self.hlen11())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 10
    #[inline(always)]
    pub fn hcode10(&mut self) -> HCODE10_W<'_, HUFFENC_DC1_5rs> {
        HCODE10_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 10
    #[inline(always)]
    pub fn hlen10(&mut self) -> HLEN10_W<'_, HUFFENC_DC1_5rs> {
        HLEN10_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 11
    #[inline(always)]
    pub fn hcode11(&mut self) -> HCODE11_W<'_, HUFFENC_DC1_5rs> {
        HCODE11_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 11
    #[inline(always)]
    pub fn hlen11(&mut self) -> HLEN11_W<'_, HUFFENC_DC1_5rs> {
        HLEN11_W::new(self, 24)
    }
}
/**JPEG Huffman encoder DC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc1_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc1_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_DC1_5)*/
pub struct HUFFENC_DC1_5rs;
impl crate::RegisterSpec for HUFFENC_DC1_5rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_dc1_5::R`](R) reader structure
impl crate::Readable for HUFFENC_DC1_5rs {}
///`write(|w| ..)` method takes [`huffenc_dc1_5::W`](W) writer structure
impl crate::Writable for HUFFENC_DC1_5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_DC1_5 to value 0
impl crate::Resettable for HUFFENC_DC1_5rs {}
