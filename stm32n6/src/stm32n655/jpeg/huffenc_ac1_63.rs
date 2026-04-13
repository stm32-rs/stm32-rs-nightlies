///Register `HUFFENC_AC1_63` reader
pub type R = crate::R<HUFFENC_AC1_63rs>;
///Register `HUFFENC_AC1_63` writer
pub type W = crate::W<HUFFENC_AC1_63rs>;
///Field `HCODE126` reader - Huffman code 126
pub type HCODE126_R = crate::FieldReader;
///Field `HCODE126` writer - Huffman code 126
pub type HCODE126_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN126` reader - Huffman length 126
pub type HLEN126_R = crate::FieldReader;
///Field `HLEN126` writer - Huffman length 126
pub type HLEN126_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE127` reader - Huffman code 127
pub type HCODE127_R = crate::FieldReader;
///Field `HCODE127` writer - Huffman code 127
pub type HCODE127_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN127` reader - Huffman length 127
pub type HLEN127_R = crate::FieldReader;
///Field `HLEN127` writer - Huffman length 127
pub type HLEN127_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 126
    #[inline(always)]
    pub fn hcode126(&self) -> HCODE126_R {
        HCODE126_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 126
    #[inline(always)]
    pub fn hlen126(&self) -> HLEN126_R {
        HLEN126_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 127
    #[inline(always)]
    pub fn hcode127(&self) -> HCODE127_R {
        HCODE127_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 127
    #[inline(always)]
    pub fn hlen127(&self) -> HLEN127_R {
        HLEN127_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_63")
            .field("hcode126", &self.hcode126())
            .field("hlen126", &self.hlen126())
            .field("hcode127", &self.hcode127())
            .field("hlen127", &self.hlen127())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 126
    #[inline(always)]
    pub fn hcode126(&mut self) -> HCODE126_W<'_, HUFFENC_AC1_63rs> {
        HCODE126_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 126
    #[inline(always)]
    pub fn hlen126(&mut self) -> HLEN126_W<'_, HUFFENC_AC1_63rs> {
        HLEN126_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 127
    #[inline(always)]
    pub fn hcode127(&mut self) -> HCODE127_W<'_, HUFFENC_AC1_63rs> {
        HCODE127_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 127
    #[inline(always)]
    pub fn hlen127(&mut self) -> HLEN127_W<'_, HUFFENC_AC1_63rs> {
        HLEN127_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_63::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_63::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_63)*/
pub struct HUFFENC_AC1_63rs;
impl crate::RegisterSpec for HUFFENC_AC1_63rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_63::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_63rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_63::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_63rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_63 to value 0
impl crate::Resettable for HUFFENC_AC1_63rs {}
