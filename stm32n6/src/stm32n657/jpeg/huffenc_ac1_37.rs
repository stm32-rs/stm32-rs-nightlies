///Register `HUFFENC_AC1_37` reader
pub type R = crate::R<HUFFENC_AC1_37rs>;
///Register `HUFFENC_AC1_37` writer
pub type W = crate::W<HUFFENC_AC1_37rs>;
///Field `HCODE74` reader - Huffman code 74
pub type HCODE74_R = crate::FieldReader;
///Field `HCODE74` writer - Huffman code 74
pub type HCODE74_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN74` reader - Huffman length 74
pub type HLEN74_R = crate::FieldReader;
///Field `HLEN74` writer - Huffman length 74
pub type HLEN74_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE75` reader - Huffman code 75
pub type HCODE75_R = crate::FieldReader;
///Field `HCODE75` writer - Huffman code 75
pub type HCODE75_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN75` reader - Huffman length 75
pub type HLEN75_R = crate::FieldReader;
///Field `HLEN75` writer - Huffman length 75
pub type HLEN75_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 74
    #[inline(always)]
    pub fn hcode74(&self) -> HCODE74_R {
        HCODE74_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 74
    #[inline(always)]
    pub fn hlen74(&self) -> HLEN74_R {
        HLEN74_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 75
    #[inline(always)]
    pub fn hcode75(&self) -> HCODE75_R {
        HCODE75_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 75
    #[inline(always)]
    pub fn hlen75(&self) -> HLEN75_R {
        HLEN75_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_37")
            .field("hcode74", &self.hcode74())
            .field("hlen74", &self.hlen74())
            .field("hcode75", &self.hcode75())
            .field("hlen75", &self.hlen75())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 74
    #[inline(always)]
    pub fn hcode74(&mut self) -> HCODE74_W<'_, HUFFENC_AC1_37rs> {
        HCODE74_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 74
    #[inline(always)]
    pub fn hlen74(&mut self) -> HLEN74_W<'_, HUFFENC_AC1_37rs> {
        HLEN74_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 75
    #[inline(always)]
    pub fn hcode75(&mut self) -> HCODE75_W<'_, HUFFENC_AC1_37rs> {
        HCODE75_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 75
    #[inline(always)]
    pub fn hlen75(&mut self) -> HLEN75_W<'_, HUFFENC_AC1_37rs> {
        HLEN75_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_37::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_37::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC1_37)*/
pub struct HUFFENC_AC1_37rs;
impl crate::RegisterSpec for HUFFENC_AC1_37rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_37::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_37rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_37::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_37rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_37 to value 0
impl crate::Resettable for HUFFENC_AC1_37rs {}
