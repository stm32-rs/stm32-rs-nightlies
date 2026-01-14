///Register `HUFFENC_AC0_78` reader
pub type R = crate::R<HUFFENC_AC0_78rs>;
///Register `HUFFENC_AC0_78` writer
pub type W = crate::W<HUFFENC_AC0_78rs>;
///Field `HCODE156` reader - Huffman code 156
pub type HCODE156_R = crate::FieldReader;
///Field `HCODE156` writer - Huffman code 156
pub type HCODE156_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN156` reader - Huffman length 156
pub type HLEN156_R = crate::FieldReader;
///Field `HLEN156` writer - Huffman length 156
pub type HLEN156_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE157` reader - Huffman code 157
pub type HCODE157_R = crate::FieldReader;
///Field `HCODE157` writer - Huffman code 157
pub type HCODE157_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN157` reader - Huffman length 157
pub type HLEN157_R = crate::FieldReader;
///Field `HLEN157` writer - Huffman length 157
pub type HLEN157_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 156
    #[inline(always)]
    pub fn hcode156(&self) -> HCODE156_R {
        HCODE156_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 156
    #[inline(always)]
    pub fn hlen156(&self) -> HLEN156_R {
        HLEN156_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 157
    #[inline(always)]
    pub fn hcode157(&self) -> HCODE157_R {
        HCODE157_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 157
    #[inline(always)]
    pub fn hlen157(&self) -> HLEN157_R {
        HLEN157_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_78")
            .field("hcode156", &self.hcode156())
            .field("hlen156", &self.hlen156())
            .field("hcode157", &self.hcode157())
            .field("hlen157", &self.hlen157())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 156
    #[inline(always)]
    pub fn hcode156(&mut self) -> HCODE156_W<'_, HUFFENC_AC0_78rs> {
        HCODE156_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 156
    #[inline(always)]
    pub fn hlen156(&mut self) -> HLEN156_W<'_, HUFFENC_AC0_78rs> {
        HLEN156_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 157
    #[inline(always)]
    pub fn hcode157(&mut self) -> HCODE157_W<'_, HUFFENC_AC0_78rs> {
        HCODE157_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 157
    #[inline(always)]
    pub fn hlen157(&mut self) -> HLEN157_W<'_, HUFFENC_AC0_78rs> {
        HLEN157_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_78::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_78::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC0_78)*/
pub struct HUFFENC_AC0_78rs;
impl crate::RegisterSpec for HUFFENC_AC0_78rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_78::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_78rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_78::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_78rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_78 to value 0
impl crate::Resettable for HUFFENC_AC0_78rs {}
