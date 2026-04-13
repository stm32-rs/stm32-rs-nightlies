///Register `HUFFENC_AC0_50` reader
pub type R = crate::R<HUFFENC_AC0_50rs>;
///Register `HUFFENC_AC0_50` writer
pub type W = crate::W<HUFFENC_AC0_50rs>;
///Field `HCODE100` reader - Huffman code 100
pub type HCODE100_R = crate::FieldReader;
///Field `HCODE100` writer - Huffman code 100
pub type HCODE100_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN100` reader - Huffman length 100
pub type HLEN100_R = crate::FieldReader;
///Field `HLEN100` writer - Huffman length 100
pub type HLEN100_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE101` reader - Huffman code 101
pub type HCODE101_R = crate::FieldReader;
///Field `HCODE101` writer - Huffman code 101
pub type HCODE101_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN101` reader - Huffman length 101
pub type HLEN101_R = crate::FieldReader;
///Field `HLEN101` writer - Huffman length 101
pub type HLEN101_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 100
    #[inline(always)]
    pub fn hcode100(&self) -> HCODE100_R {
        HCODE100_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 100
    #[inline(always)]
    pub fn hlen100(&self) -> HLEN100_R {
        HLEN100_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 101
    #[inline(always)]
    pub fn hcode101(&self) -> HCODE101_R {
        HCODE101_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 101
    #[inline(always)]
    pub fn hlen101(&self) -> HLEN101_R {
        HLEN101_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_50")
            .field("hcode100", &self.hcode100())
            .field("hlen100", &self.hlen100())
            .field("hcode101", &self.hcode101())
            .field("hlen101", &self.hlen101())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 100
    #[inline(always)]
    pub fn hcode100(&mut self) -> HCODE100_W<'_, HUFFENC_AC0_50rs> {
        HCODE100_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 100
    #[inline(always)]
    pub fn hlen100(&mut self) -> HLEN100_W<'_, HUFFENC_AC0_50rs> {
        HLEN100_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 101
    #[inline(always)]
    pub fn hcode101(&mut self) -> HCODE101_W<'_, HUFFENC_AC0_50rs> {
        HCODE101_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 101
    #[inline(always)]
    pub fn hlen101(&mut self) -> HLEN101_W<'_, HUFFENC_AC0_50rs> {
        HLEN101_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_50)*/
pub struct HUFFENC_AC0_50rs;
impl crate::RegisterSpec for HUFFENC_AC0_50rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_50::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_50rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_50::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_50rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_50 to value 0
impl crate::Resettable for HUFFENC_AC0_50rs {}
