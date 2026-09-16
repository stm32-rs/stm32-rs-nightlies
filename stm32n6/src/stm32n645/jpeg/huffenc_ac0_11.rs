///Register `HUFFENC_AC0_11` reader
pub type R = crate::R<HUFFENC_AC0_11rs>;
///Register `HUFFENC_AC0_11` writer
pub type W = crate::W<HUFFENC_AC0_11rs>;
///Field `HCODE22` reader - Huffman code 22
pub type HCODE22_R = crate::FieldReader;
///Field `HCODE22` writer - Huffman code 22
pub type HCODE22_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN22` reader - Huffman length 22
pub type HLEN22_R = crate::FieldReader;
///Field `HLEN22` writer - Huffman length 22
pub type HLEN22_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE23` reader - Huffman code 23
pub type HCODE23_R = crate::FieldReader;
///Field `HCODE23` writer - Huffman code 23
pub type HCODE23_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN23` reader - Huffman length 23
pub type HLEN23_R = crate::FieldReader;
///Field `HLEN23` writer - Huffman length 23
pub type HLEN23_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 22
    #[inline(always)]
    pub fn hcode22(&self) -> HCODE22_R {
        HCODE22_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 22
    #[inline(always)]
    pub fn hlen22(&self) -> HLEN22_R {
        HLEN22_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 23
    #[inline(always)]
    pub fn hcode23(&self) -> HCODE23_R {
        HCODE23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 23
    #[inline(always)]
    pub fn hlen23(&self) -> HLEN23_R {
        HLEN23_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_11")
            .field("hcode22", &self.hcode22())
            .field("hlen22", &self.hlen22())
            .field("hcode23", &self.hcode23())
            .field("hlen23", &self.hlen23())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 22
    #[inline(always)]
    pub fn hcode22(&mut self) -> HCODE22_W<'_, HUFFENC_AC0_11rs> {
        HCODE22_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 22
    #[inline(always)]
    pub fn hlen22(&mut self) -> HLEN22_W<'_, HUFFENC_AC0_11rs> {
        HLEN22_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 23
    #[inline(always)]
    pub fn hcode23(&mut self) -> HCODE23_W<'_, HUFFENC_AC0_11rs> {
        HCODE23_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 23
    #[inline(always)]
    pub fn hlen23(&mut self) -> HLEN23_W<'_, HUFFENC_AC0_11rs> {
        HLEN23_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC0_11)*/
pub struct HUFFENC_AC0_11rs;
impl crate::RegisterSpec for HUFFENC_AC0_11rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_11::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_11rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_11::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_11rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_11 to value 0
impl crate::Resettable for HUFFENC_AC0_11rs {}
