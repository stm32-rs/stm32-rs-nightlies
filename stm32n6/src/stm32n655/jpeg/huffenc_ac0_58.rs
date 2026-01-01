///Register `HUFFENC_AC0_58` reader
pub type R = crate::R<HUFFENC_AC0_58rs>;
///Register `HUFFENC_AC0_58` writer
pub type W = crate::W<HUFFENC_AC0_58rs>;
///Field `HCODE116` reader - Huffman code 116
pub type HCODE116_R = crate::FieldReader;
///Field `HCODE116` writer - Huffman code 116
pub type HCODE116_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN116` reader - Huffman length 116
pub type HLEN116_R = crate::FieldReader;
///Field `HLEN116` writer - Huffman length 116
pub type HLEN116_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE117` reader - Huffman code 117
pub type HCODE117_R = crate::FieldReader;
///Field `HCODE117` writer - Huffman code 117
pub type HCODE117_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN117` reader - Huffman length 117
pub type HLEN117_R = crate::FieldReader;
///Field `HLEN117` writer - Huffman length 117
pub type HLEN117_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 116
    #[inline(always)]
    pub fn hcode116(&self) -> HCODE116_R {
        HCODE116_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 116
    #[inline(always)]
    pub fn hlen116(&self) -> HLEN116_R {
        HLEN116_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 117
    #[inline(always)]
    pub fn hcode117(&self) -> HCODE117_R {
        HCODE117_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 117
    #[inline(always)]
    pub fn hlen117(&self) -> HLEN117_R {
        HLEN117_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_58")
            .field("hcode116", &self.hcode116())
            .field("hlen116", &self.hlen116())
            .field("hcode117", &self.hcode117())
            .field("hlen117", &self.hlen117())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 116
    #[inline(always)]
    pub fn hcode116(&mut self) -> HCODE116_W<'_, HUFFENC_AC0_58rs> {
        HCODE116_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 116
    #[inline(always)]
    pub fn hlen116(&mut self) -> HLEN116_W<'_, HUFFENC_AC0_58rs> {
        HLEN116_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 117
    #[inline(always)]
    pub fn hcode117(&mut self) -> HCODE117_W<'_, HUFFENC_AC0_58rs> {
        HCODE117_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 117
    #[inline(always)]
    pub fn hlen117(&mut self) -> HLEN117_W<'_, HUFFENC_AC0_58rs> {
        HLEN117_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_58::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_58::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_58)*/
pub struct HUFFENC_AC0_58rs;
impl crate::RegisterSpec for HUFFENC_AC0_58rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_58::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_58rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_58::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_58rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_58 to value 0
impl crate::Resettable for HUFFENC_AC0_58rs {}
