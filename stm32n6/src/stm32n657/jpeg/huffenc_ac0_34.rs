///Register `HUFFENC_AC0_34` reader
pub type R = crate::R<HUFFENC_AC0_34rs>;
///Register `HUFFENC_AC0_34` writer
pub type W = crate::W<HUFFENC_AC0_34rs>;
///Field `HCODE68` reader - Huffman code 68
pub type HCODE68_R = crate::FieldReader;
///Field `HCODE68` writer - Huffman code 68
pub type HCODE68_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN68` reader - Huffman length 68
pub type HLEN68_R = crate::FieldReader;
///Field `HLEN68` writer - Huffman length 68
pub type HLEN68_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE69` reader - Huffman code 69
pub type HCODE69_R = crate::FieldReader;
///Field `HCODE69` writer - Huffman code 69
pub type HCODE69_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN69` reader - Huffman length 69
pub type HLEN69_R = crate::FieldReader;
///Field `HLEN69` writer - Huffman length 69
pub type HLEN69_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 68
    #[inline(always)]
    pub fn hcode68(&self) -> HCODE68_R {
        HCODE68_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 68
    #[inline(always)]
    pub fn hlen68(&self) -> HLEN68_R {
        HLEN68_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 69
    #[inline(always)]
    pub fn hcode69(&self) -> HCODE69_R {
        HCODE69_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 69
    #[inline(always)]
    pub fn hlen69(&self) -> HLEN69_R {
        HLEN69_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_34")
            .field("hcode68", &self.hcode68())
            .field("hlen68", &self.hlen68())
            .field("hcode69", &self.hcode69())
            .field("hlen69", &self.hlen69())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 68
    #[inline(always)]
    pub fn hcode68(&mut self) -> HCODE68_W<HUFFENC_AC0_34rs> {
        HCODE68_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 68
    #[inline(always)]
    pub fn hlen68(&mut self) -> HLEN68_W<HUFFENC_AC0_34rs> {
        HLEN68_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 69
    #[inline(always)]
    pub fn hcode69(&mut self) -> HCODE69_W<HUFFENC_AC0_34rs> {
        HCODE69_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 69
    #[inline(always)]
    pub fn hlen69(&mut self) -> HLEN69_W<HUFFENC_AC0_34rs> {
        HLEN69_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC0_34)*/
pub struct HUFFENC_AC0_34rs;
impl crate::RegisterSpec for HUFFENC_AC0_34rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_34::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_34rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_34::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_34rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_34 to value 0
impl crate::Resettable for HUFFENC_AC0_34rs {}
