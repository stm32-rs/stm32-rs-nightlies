///Register `HUFFENC_DC0_6` reader
pub type R = crate::R<HUFFENC_DC0_6rs>;
///Register `HUFFENC_DC0_6` writer
pub type W = crate::W<HUFFENC_DC0_6rs>;
///Field `HCODE12` reader - Huffman code 12
pub type HCODE12_R = crate::FieldReader;
///Field `HCODE12` writer - Huffman code 12
pub type HCODE12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN12` reader - Huffman length 12
pub type HLEN12_R = crate::FieldReader;
///Field `HLEN12` writer - Huffman length 12
pub type HLEN12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE13` reader - Huffman code 13
pub type HCODE13_R = crate::FieldReader;
///Field `HCODE13` writer - Huffman code 13
pub type HCODE13_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN13` reader - Huffman length 13
pub type HLEN13_R = crate::FieldReader;
///Field `HLEN13` writer - Huffman length 13
pub type HLEN13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 12
    #[inline(always)]
    pub fn hcode12(&self) -> HCODE12_R {
        HCODE12_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 12
    #[inline(always)]
    pub fn hlen12(&self) -> HLEN12_R {
        HLEN12_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 13
    #[inline(always)]
    pub fn hcode13(&self) -> HCODE13_R {
        HCODE13_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 13
    #[inline(always)]
    pub fn hlen13(&self) -> HLEN13_R {
        HLEN13_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_DC0_6")
            .field("hcode12", &self.hcode12())
            .field("hlen12", &self.hlen12())
            .field("hcode13", &self.hcode13())
            .field("hlen13", &self.hlen13())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 12
    #[inline(always)]
    pub fn hcode12(&mut self) -> HCODE12_W<'_, HUFFENC_DC0_6rs> {
        HCODE12_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 12
    #[inline(always)]
    pub fn hlen12(&mut self) -> HLEN12_W<'_, HUFFENC_DC0_6rs> {
        HLEN12_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 13
    #[inline(always)]
    pub fn hcode13(&mut self) -> HCODE13_W<'_, HUFFENC_DC0_6rs> {
        HCODE13_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 13
    #[inline(always)]
    pub fn hlen13(&mut self) -> HLEN13_W<'_, HUFFENC_DC0_6rs> {
        HLEN13_W::new(self, 24)
    }
}
/**JPEG Huffman encoder DC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc0_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc0_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_DC0_6)*/
pub struct HUFFENC_DC0_6rs;
impl crate::RegisterSpec for HUFFENC_DC0_6rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_dc0_6::R`](R) reader structure
impl crate::Readable for HUFFENC_DC0_6rs {}
///`write(|w| ..)` method takes [`huffenc_dc0_6::W`](W) writer structure
impl crate::Writable for HUFFENC_DC0_6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_DC0_6 to value 0
impl crate::Resettable for HUFFENC_DC0_6rs {}
