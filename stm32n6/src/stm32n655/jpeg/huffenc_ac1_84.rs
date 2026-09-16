///Register `HUFFENC_AC1_84` reader
pub type R = crate::R<HUFFENC_AC1_84rs>;
///Register `HUFFENC_AC1_84` writer
pub type W = crate::W<HUFFENC_AC1_84rs>;
///Field `HCODE168` reader - Huffman code 168
pub type HCODE168_R = crate::FieldReader;
///Field `HCODE168` writer - Huffman code 168
pub type HCODE168_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN168` reader - Huffman length 168
pub type HLEN168_R = crate::FieldReader;
///Field `HLEN168` writer - Huffman length 168
pub type HLEN168_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE169` reader - Huffman code 169
pub type HCODE169_R = crate::FieldReader;
///Field `HCODE169` writer - Huffman code 169
pub type HCODE169_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN169` reader - Huffman length 169
pub type HLEN169_R = crate::FieldReader;
///Field `HLEN169` writer - Huffman length 169
pub type HLEN169_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 168
    #[inline(always)]
    pub fn hcode168(&self) -> HCODE168_R {
        HCODE168_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 168
    #[inline(always)]
    pub fn hlen168(&self) -> HLEN168_R {
        HLEN168_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 169
    #[inline(always)]
    pub fn hcode169(&self) -> HCODE169_R {
        HCODE169_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 169
    #[inline(always)]
    pub fn hlen169(&self) -> HLEN169_R {
        HLEN169_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_84")
            .field("hcode168", &self.hcode168())
            .field("hlen168", &self.hlen168())
            .field("hcode169", &self.hcode169())
            .field("hlen169", &self.hlen169())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 168
    #[inline(always)]
    pub fn hcode168(&mut self) -> HCODE168_W<'_, HUFFENC_AC1_84rs> {
        HCODE168_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 168
    #[inline(always)]
    pub fn hlen168(&mut self) -> HLEN168_W<'_, HUFFENC_AC1_84rs> {
        HLEN168_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 169
    #[inline(always)]
    pub fn hcode169(&mut self) -> HCODE169_W<'_, HUFFENC_AC1_84rs> {
        HCODE169_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 169
    #[inline(always)]
    pub fn hlen169(&mut self) -> HLEN169_W<'_, HUFFENC_AC1_84rs> {
        HLEN169_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_84::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_84::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_84)*/
pub struct HUFFENC_AC1_84rs;
impl crate::RegisterSpec for HUFFENC_AC1_84rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_84::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_84rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_84::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_84rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_84 to value 0
impl crate::Resettable for HUFFENC_AC1_84rs {}
