///Register `HUFFENC_AC1_30` reader
pub type R = crate::R<HUFFENC_AC1_30rs>;
///Register `HUFFENC_AC1_30` writer
pub type W = crate::W<HUFFENC_AC1_30rs>;
///Field `HCODE60` reader - Huffman code 60
pub type HCODE60_R = crate::FieldReader;
///Field `HCODE60` writer - Huffman code 60
pub type HCODE60_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN60` reader - Huffman length 60
pub type HLEN60_R = crate::FieldReader;
///Field `HLEN60` writer - Huffman length 60
pub type HLEN60_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE61` reader - Huffman code 61
pub type HCODE61_R = crate::FieldReader;
///Field `HCODE61` writer - Huffman code 61
pub type HCODE61_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN61` reader - Huffman length 61
pub type HLEN61_R = crate::FieldReader;
///Field `HLEN61` writer - Huffman length 61
pub type HLEN61_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 60
    #[inline(always)]
    pub fn hcode60(&self) -> HCODE60_R {
        HCODE60_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 60
    #[inline(always)]
    pub fn hlen60(&self) -> HLEN60_R {
        HLEN60_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 61
    #[inline(always)]
    pub fn hcode61(&self) -> HCODE61_R {
        HCODE61_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 61
    #[inline(always)]
    pub fn hlen61(&self) -> HLEN61_R {
        HLEN61_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_30")
            .field("hcode60", &self.hcode60())
            .field("hlen60", &self.hlen60())
            .field("hcode61", &self.hcode61())
            .field("hlen61", &self.hlen61())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 60
    #[inline(always)]
    pub fn hcode60(&mut self) -> HCODE60_W<'_, HUFFENC_AC1_30rs> {
        HCODE60_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 60
    #[inline(always)]
    pub fn hlen60(&mut self) -> HLEN60_W<'_, HUFFENC_AC1_30rs> {
        HLEN60_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 61
    #[inline(always)]
    pub fn hcode61(&mut self) -> HCODE61_W<'_, HUFFENC_AC1_30rs> {
        HCODE61_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 61
    #[inline(always)]
    pub fn hlen61(&mut self) -> HLEN61_W<'_, HUFFENC_AC1_30rs> {
        HLEN61_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC1_30)*/
pub struct HUFFENC_AC1_30rs;
impl crate::RegisterSpec for HUFFENC_AC1_30rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_30::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_30rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_30::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_30rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_30 to value 0
impl crate::Resettable for HUFFENC_AC1_30rs {}
