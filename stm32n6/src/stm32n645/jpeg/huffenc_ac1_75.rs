///Register `HUFFENC_AC1_75` reader
pub type R = crate::R<HUFFENC_AC1_75rs>;
///Register `HUFFENC_AC1_75` writer
pub type W = crate::W<HUFFENC_AC1_75rs>;
///Field `HCODE150` reader - Huffman code 150
pub type HCODE150_R = crate::FieldReader;
///Field `HCODE150` writer - Huffman code 150
pub type HCODE150_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN150` reader - Huffman length 150
pub type HLEN150_R = crate::FieldReader;
///Field `HLEN150` writer - Huffman length 150
pub type HLEN150_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE151` reader - Huffman code 151
pub type HCODE151_R = crate::FieldReader;
///Field `HCODE151` writer - Huffman code 151
pub type HCODE151_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN151` reader - Huffman length 151
pub type HLEN151_R = crate::FieldReader;
///Field `HLEN151` writer - Huffman length 151
pub type HLEN151_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 150
    #[inline(always)]
    pub fn hcode150(&self) -> HCODE150_R {
        HCODE150_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 150
    #[inline(always)]
    pub fn hlen150(&self) -> HLEN150_R {
        HLEN150_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 151
    #[inline(always)]
    pub fn hcode151(&self) -> HCODE151_R {
        HCODE151_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 151
    #[inline(always)]
    pub fn hlen151(&self) -> HLEN151_R {
        HLEN151_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_75")
            .field("hcode150", &self.hcode150())
            .field("hlen150", &self.hlen150())
            .field("hcode151", &self.hcode151())
            .field("hlen151", &self.hlen151())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 150
    #[inline(always)]
    pub fn hcode150(&mut self) -> HCODE150_W<'_, HUFFENC_AC1_75rs> {
        HCODE150_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 150
    #[inline(always)]
    pub fn hlen150(&mut self) -> HLEN150_W<'_, HUFFENC_AC1_75rs> {
        HLEN150_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 151
    #[inline(always)]
    pub fn hcode151(&mut self) -> HCODE151_W<'_, HUFFENC_AC1_75rs> {
        HCODE151_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 151
    #[inline(always)]
    pub fn hlen151(&mut self) -> HLEN151_W<'_, HUFFENC_AC1_75rs> {
        HLEN151_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_75::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_75::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC1_75)*/
pub struct HUFFENC_AC1_75rs;
impl crate::RegisterSpec for HUFFENC_AC1_75rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_75::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_75rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_75::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_75rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_75 to value 0
impl crate::Resettable for HUFFENC_AC1_75rs {}
