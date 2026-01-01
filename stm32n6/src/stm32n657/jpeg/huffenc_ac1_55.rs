///Register `HUFFENC_AC1_55` reader
pub type R = crate::R<HUFFENC_AC1_55rs>;
///Register `HUFFENC_AC1_55` writer
pub type W = crate::W<HUFFENC_AC1_55rs>;
///Field `HCODE110` reader - Huffman code 110
pub type HCODE110_R = crate::FieldReader;
///Field `HCODE110` writer - Huffman code 110
pub type HCODE110_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN110` reader - Huffman length 110
pub type HLEN110_R = crate::FieldReader;
///Field `HLEN110` writer - Huffman length 110
pub type HLEN110_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE111` reader - Huffman code 111
pub type HCODE111_R = crate::FieldReader;
///Field `HCODE111` writer - Huffman code 111
pub type HCODE111_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN111` reader - Huffman length 111
pub type HLEN111_R = crate::FieldReader;
///Field `HLEN111` writer - Huffman length 111
pub type HLEN111_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 110
    #[inline(always)]
    pub fn hcode110(&self) -> HCODE110_R {
        HCODE110_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 110
    #[inline(always)]
    pub fn hlen110(&self) -> HLEN110_R {
        HLEN110_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 111
    #[inline(always)]
    pub fn hcode111(&self) -> HCODE111_R {
        HCODE111_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 111
    #[inline(always)]
    pub fn hlen111(&self) -> HLEN111_R {
        HLEN111_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_55")
            .field("hcode110", &self.hcode110())
            .field("hlen110", &self.hlen110())
            .field("hcode111", &self.hcode111())
            .field("hlen111", &self.hlen111())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 110
    #[inline(always)]
    pub fn hcode110(&mut self) -> HCODE110_W<'_, HUFFENC_AC1_55rs> {
        HCODE110_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 110
    #[inline(always)]
    pub fn hlen110(&mut self) -> HLEN110_W<'_, HUFFENC_AC1_55rs> {
        HLEN110_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 111
    #[inline(always)]
    pub fn hcode111(&mut self) -> HCODE111_W<'_, HUFFENC_AC1_55rs> {
        HCODE111_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 111
    #[inline(always)]
    pub fn hlen111(&mut self) -> HLEN111_W<'_, HUFFENC_AC1_55rs> {
        HLEN111_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_55::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_55::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC1_55)*/
pub struct HUFFENC_AC1_55rs;
impl crate::RegisterSpec for HUFFENC_AC1_55rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_55::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_55rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_55::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_55rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_55 to value 0
impl crate::Resettable for HUFFENC_AC1_55rs {}
