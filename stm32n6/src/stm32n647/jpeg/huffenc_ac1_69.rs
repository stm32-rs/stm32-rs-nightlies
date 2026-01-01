///Register `HUFFENC_AC1_69` reader
pub type R = crate::R<HUFFENC_AC1_69rs>;
///Register `HUFFENC_AC1_69` writer
pub type W = crate::W<HUFFENC_AC1_69rs>;
///Field `HCODE138` reader - Huffman code 138
pub type HCODE138_R = crate::FieldReader;
///Field `HCODE138` writer - Huffman code 138
pub type HCODE138_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN138` reader - Huffman length 138
pub type HLEN138_R = crate::FieldReader;
///Field `HLEN138` writer - Huffman length 138
pub type HLEN138_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE139` reader - Huffman code 139
pub type HCODE139_R = crate::FieldReader;
///Field `HCODE139` writer - Huffman code 139
pub type HCODE139_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN139` reader - Huffman length 139
pub type HLEN139_R = crate::FieldReader;
///Field `HLEN139` writer - Huffman length 139
pub type HLEN139_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 138
    #[inline(always)]
    pub fn hcode138(&self) -> HCODE138_R {
        HCODE138_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 138
    #[inline(always)]
    pub fn hlen138(&self) -> HLEN138_R {
        HLEN138_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 139
    #[inline(always)]
    pub fn hcode139(&self) -> HCODE139_R {
        HCODE139_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 139
    #[inline(always)]
    pub fn hlen139(&self) -> HLEN139_R {
        HLEN139_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_69")
            .field("hcode138", &self.hcode138())
            .field("hlen138", &self.hlen138())
            .field("hcode139", &self.hcode139())
            .field("hlen139", &self.hlen139())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 138
    #[inline(always)]
    pub fn hcode138(&mut self) -> HCODE138_W<'_, HUFFENC_AC1_69rs> {
        HCODE138_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 138
    #[inline(always)]
    pub fn hlen138(&mut self) -> HLEN138_W<'_, HUFFENC_AC1_69rs> {
        HLEN138_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 139
    #[inline(always)]
    pub fn hcode139(&mut self) -> HCODE139_W<'_, HUFFENC_AC1_69rs> {
        HCODE139_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 139
    #[inline(always)]
    pub fn hlen139(&mut self) -> HLEN139_W<'_, HUFFENC_AC1_69rs> {
        HLEN139_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_69::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_69::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC1_69)*/
pub struct HUFFENC_AC1_69rs;
impl crate::RegisterSpec for HUFFENC_AC1_69rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_69::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_69rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_69::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_69rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_69 to value 0
impl crate::Resettable for HUFFENC_AC1_69rs {}
