///Register `HUFFENC_AC0_61` reader
pub type R = crate::R<HUFFENC_AC0_61rs>;
///Register `HUFFENC_AC0_61` writer
pub type W = crate::W<HUFFENC_AC0_61rs>;
///Field `HCODE122` reader - Huffman code 122
pub type HCODE122_R = crate::FieldReader;
///Field `HCODE122` writer - Huffman code 122
pub type HCODE122_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN122` reader - Huffman length 122
pub type HLEN122_R = crate::FieldReader;
///Field `HLEN122` writer - Huffman length 122
pub type HLEN122_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE123` reader - Huffman code 123
pub type HCODE123_R = crate::FieldReader;
///Field `HCODE123` writer - Huffman code 123
pub type HCODE123_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN123` reader - Huffman length 123
pub type HLEN123_R = crate::FieldReader;
///Field `HLEN123` writer - Huffman length 123
pub type HLEN123_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 122
    #[inline(always)]
    pub fn hcode122(&self) -> HCODE122_R {
        HCODE122_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 122
    #[inline(always)]
    pub fn hlen122(&self) -> HLEN122_R {
        HLEN122_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 123
    #[inline(always)]
    pub fn hcode123(&self) -> HCODE123_R {
        HCODE123_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 123
    #[inline(always)]
    pub fn hlen123(&self) -> HLEN123_R {
        HLEN123_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_61")
            .field("hcode122", &self.hcode122())
            .field("hlen122", &self.hlen122())
            .field("hcode123", &self.hcode123())
            .field("hlen123", &self.hlen123())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 122
    #[inline(always)]
    pub fn hcode122(&mut self) -> HCODE122_W<'_, HUFFENC_AC0_61rs> {
        HCODE122_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 122
    #[inline(always)]
    pub fn hlen122(&mut self) -> HLEN122_W<'_, HUFFENC_AC0_61rs> {
        HLEN122_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 123
    #[inline(always)]
    pub fn hcode123(&mut self) -> HCODE123_W<'_, HUFFENC_AC0_61rs> {
        HCODE123_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 123
    #[inline(always)]
    pub fn hlen123(&mut self) -> HLEN123_W<'_, HUFFENC_AC0_61rs> {
        HLEN123_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_61::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_61::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC0_61)*/
pub struct HUFFENC_AC0_61rs;
impl crate::RegisterSpec for HUFFENC_AC0_61rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_61::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_61rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_61::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_61rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_61 to value 0
impl crate::Resettable for HUFFENC_AC0_61rs {}
