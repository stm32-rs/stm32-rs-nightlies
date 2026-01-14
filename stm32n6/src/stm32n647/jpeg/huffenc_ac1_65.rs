///Register `HUFFENC_AC1_65` reader
pub type R = crate::R<HUFFENC_AC1_65rs>;
///Register `HUFFENC_AC1_65` writer
pub type W = crate::W<HUFFENC_AC1_65rs>;
///Field `HCODE130` reader - Huffman code 130
pub type HCODE130_R = crate::FieldReader;
///Field `HCODE130` writer - Huffman code 130
pub type HCODE130_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN130` reader - Huffman length 130
pub type HLEN130_R = crate::FieldReader;
///Field `HLEN130` writer - Huffman length 130
pub type HLEN130_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE131` reader - Huffman code 131
pub type HCODE131_R = crate::FieldReader;
///Field `HCODE131` writer - Huffman code 131
pub type HCODE131_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN131` reader - Huffman length 131
pub type HLEN131_R = crate::FieldReader;
///Field `HLEN131` writer - Huffman length 131
pub type HLEN131_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 130
    #[inline(always)]
    pub fn hcode130(&self) -> HCODE130_R {
        HCODE130_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 130
    #[inline(always)]
    pub fn hlen130(&self) -> HLEN130_R {
        HLEN130_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 131
    #[inline(always)]
    pub fn hcode131(&self) -> HCODE131_R {
        HCODE131_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 131
    #[inline(always)]
    pub fn hlen131(&self) -> HLEN131_R {
        HLEN131_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_65")
            .field("hcode130", &self.hcode130())
            .field("hlen130", &self.hlen130())
            .field("hcode131", &self.hcode131())
            .field("hlen131", &self.hlen131())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 130
    #[inline(always)]
    pub fn hcode130(&mut self) -> HCODE130_W<'_, HUFFENC_AC1_65rs> {
        HCODE130_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 130
    #[inline(always)]
    pub fn hlen130(&mut self) -> HLEN130_W<'_, HUFFENC_AC1_65rs> {
        HLEN130_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 131
    #[inline(always)]
    pub fn hcode131(&mut self) -> HCODE131_W<'_, HUFFENC_AC1_65rs> {
        HCODE131_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 131
    #[inline(always)]
    pub fn hlen131(&mut self) -> HLEN131_W<'_, HUFFENC_AC1_65rs> {
        HLEN131_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_65::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_65::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC1_65)*/
pub struct HUFFENC_AC1_65rs;
impl crate::RegisterSpec for HUFFENC_AC1_65rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_65::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_65rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_65::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_65rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_65 to value 0
impl crate::Resettable for HUFFENC_AC1_65rs {}
