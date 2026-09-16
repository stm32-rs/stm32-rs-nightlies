///Register `HUFFENC_AC0_36` reader
pub type R = crate::R<HUFFENC_AC0_36rs>;
///Register `HUFFENC_AC0_36` writer
pub type W = crate::W<HUFFENC_AC0_36rs>;
///Field `HCODE72` reader - Huffman code 72
pub type HCODE72_R = crate::FieldReader;
///Field `HCODE72` writer - Huffman code 72
pub type HCODE72_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN72` reader - Huffman length 72
pub type HLEN72_R = crate::FieldReader;
///Field `HLEN72` writer - Huffman length 72
pub type HLEN72_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE73` reader - Huffman code 73
pub type HCODE73_R = crate::FieldReader;
///Field `HCODE73` writer - Huffman code 73
pub type HCODE73_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN73` reader - Huffman length 73
pub type HLEN73_R = crate::FieldReader;
///Field `HLEN73` writer - Huffman length 73
pub type HLEN73_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 72
    #[inline(always)]
    pub fn hcode72(&self) -> HCODE72_R {
        HCODE72_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 72
    #[inline(always)]
    pub fn hlen72(&self) -> HLEN72_R {
        HLEN72_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 73
    #[inline(always)]
    pub fn hcode73(&self) -> HCODE73_R {
        HCODE73_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 73
    #[inline(always)]
    pub fn hlen73(&self) -> HLEN73_R {
        HLEN73_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_36")
            .field("hcode72", &self.hcode72())
            .field("hlen72", &self.hlen72())
            .field("hcode73", &self.hcode73())
            .field("hlen73", &self.hlen73())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 72
    #[inline(always)]
    pub fn hcode72(&mut self) -> HCODE72_W<'_, HUFFENC_AC0_36rs> {
        HCODE72_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 72
    #[inline(always)]
    pub fn hlen72(&mut self) -> HLEN72_W<'_, HUFFENC_AC0_36rs> {
        HLEN72_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 73
    #[inline(always)]
    pub fn hcode73(&mut self) -> HCODE73_W<'_, HUFFENC_AC0_36rs> {
        HCODE73_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 73
    #[inline(always)]
    pub fn hlen73(&mut self) -> HLEN73_W<'_, HUFFENC_AC0_36rs> {
        HLEN73_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_36::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_36::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC0_36)*/
pub struct HUFFENC_AC0_36rs;
impl crate::RegisterSpec for HUFFENC_AC0_36rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_36::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_36rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_36::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_36rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_36 to value 0
impl crate::Resettable for HUFFENC_AC0_36rs {}
