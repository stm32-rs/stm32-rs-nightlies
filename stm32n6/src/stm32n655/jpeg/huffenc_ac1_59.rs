///Register `HUFFENC_AC1_59` reader
pub type R = crate::R<HUFFENC_AC1_59rs>;
///Register `HUFFENC_AC1_59` writer
pub type W = crate::W<HUFFENC_AC1_59rs>;
///Field `HCODE118` reader - Huffman code 118
pub type HCODE118_R = crate::FieldReader;
///Field `HCODE118` writer - Huffman code 118
pub type HCODE118_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN118` reader - Huffman length 118
pub type HLEN118_R = crate::FieldReader;
///Field `HLEN118` writer - Huffman length 118
pub type HLEN118_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE119` reader - Huffman code 119
pub type HCODE119_R = crate::FieldReader;
///Field `HCODE119` writer - Huffman code 119
pub type HCODE119_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN119` reader - Huffman length 119
pub type HLEN119_R = crate::FieldReader;
///Field `HLEN119` writer - Huffman length 119
pub type HLEN119_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 118
    #[inline(always)]
    pub fn hcode118(&self) -> HCODE118_R {
        HCODE118_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 118
    #[inline(always)]
    pub fn hlen118(&self) -> HLEN118_R {
        HLEN118_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 119
    #[inline(always)]
    pub fn hcode119(&self) -> HCODE119_R {
        HCODE119_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 119
    #[inline(always)]
    pub fn hlen119(&self) -> HLEN119_R {
        HLEN119_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_59")
            .field("hcode118", &self.hcode118())
            .field("hlen118", &self.hlen118())
            .field("hcode119", &self.hcode119())
            .field("hlen119", &self.hlen119())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 118
    #[inline(always)]
    pub fn hcode118(&mut self) -> HCODE118_W<'_, HUFFENC_AC1_59rs> {
        HCODE118_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 118
    #[inline(always)]
    pub fn hlen118(&mut self) -> HLEN118_W<'_, HUFFENC_AC1_59rs> {
        HLEN118_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 119
    #[inline(always)]
    pub fn hcode119(&mut self) -> HCODE119_W<'_, HUFFENC_AC1_59rs> {
        HCODE119_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 119
    #[inline(always)]
    pub fn hlen119(&mut self) -> HLEN119_W<'_, HUFFENC_AC1_59rs> {
        HLEN119_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_59::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_59::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_59)*/
pub struct HUFFENC_AC1_59rs;
impl crate::RegisterSpec for HUFFENC_AC1_59rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_59::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_59rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_59::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_59rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_59 to value 0
impl crate::Resettable for HUFFENC_AC1_59rs {}
