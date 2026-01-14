///Register `HUFFENC_AC1_25` reader
pub type R = crate::R<HUFFENC_AC1_25rs>;
///Register `HUFFENC_AC1_25` writer
pub type W = crate::W<HUFFENC_AC1_25rs>;
///Field `HCODE50` reader - Huffman code 50
pub type HCODE50_R = crate::FieldReader;
///Field `HCODE50` writer - Huffman code 50
pub type HCODE50_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN50` reader - Huffman length 50
pub type HLEN50_R = crate::FieldReader;
///Field `HLEN50` writer - Huffman length 50
pub type HLEN50_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE51` reader - Huffman code 51
pub type HCODE51_R = crate::FieldReader;
///Field `HCODE51` writer - Huffman code 51
pub type HCODE51_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN51` reader - Huffman length 51
pub type HLEN51_R = crate::FieldReader;
///Field `HLEN51` writer - Huffman length 51
pub type HLEN51_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 50
    #[inline(always)]
    pub fn hcode50(&self) -> HCODE50_R {
        HCODE50_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 50
    #[inline(always)]
    pub fn hlen50(&self) -> HLEN50_R {
        HLEN50_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 51
    #[inline(always)]
    pub fn hcode51(&self) -> HCODE51_R {
        HCODE51_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 51
    #[inline(always)]
    pub fn hlen51(&self) -> HLEN51_R {
        HLEN51_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_25")
            .field("hcode50", &self.hcode50())
            .field("hlen50", &self.hlen50())
            .field("hcode51", &self.hcode51())
            .field("hlen51", &self.hlen51())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 50
    #[inline(always)]
    pub fn hcode50(&mut self) -> HCODE50_W<'_, HUFFENC_AC1_25rs> {
        HCODE50_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 50
    #[inline(always)]
    pub fn hlen50(&mut self) -> HLEN50_W<'_, HUFFENC_AC1_25rs> {
        HLEN50_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 51
    #[inline(always)]
    pub fn hcode51(&mut self) -> HCODE51_W<'_, HUFFENC_AC1_25rs> {
        HCODE51_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 51
    #[inline(always)]
    pub fn hlen51(&mut self) -> HLEN51_W<'_, HUFFENC_AC1_25rs> {
        HLEN51_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC1_25)*/
pub struct HUFFENC_AC1_25rs;
impl crate::RegisterSpec for HUFFENC_AC1_25rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_25::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_25rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_25::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_25rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_25 to value 0
impl crate::Resettable for HUFFENC_AC1_25rs {}
