///Register `HUFFENC_AC0_47` reader
pub type R = crate::R<HUFFENC_AC0_47rs>;
///Register `HUFFENC_AC0_47` writer
pub type W = crate::W<HUFFENC_AC0_47rs>;
///Field `HCODE94` reader - Huffman code 94
pub type HCODE94_R = crate::FieldReader;
///Field `HCODE94` writer - Huffman code 94
pub type HCODE94_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN94` reader - Huffman length 94
pub type HLEN94_R = crate::FieldReader;
///Field `HLEN94` writer - Huffman length 94
pub type HLEN94_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE95` reader - Huffman code 95
pub type HCODE95_R = crate::FieldReader;
///Field `HCODE95` writer - Huffman code 95
pub type HCODE95_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN95` reader - Huffman length 95
pub type HLEN95_R = crate::FieldReader;
///Field `HLEN95` writer - Huffman length 95
pub type HLEN95_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 94
    #[inline(always)]
    pub fn hcode94(&self) -> HCODE94_R {
        HCODE94_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 94
    #[inline(always)]
    pub fn hlen94(&self) -> HLEN94_R {
        HLEN94_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 95
    #[inline(always)]
    pub fn hcode95(&self) -> HCODE95_R {
        HCODE95_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 95
    #[inline(always)]
    pub fn hlen95(&self) -> HLEN95_R {
        HLEN95_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_47")
            .field("hcode94", &self.hcode94())
            .field("hlen94", &self.hlen94())
            .field("hcode95", &self.hcode95())
            .field("hlen95", &self.hlen95())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 94
    #[inline(always)]
    pub fn hcode94(&mut self) -> HCODE94_W<'_, HUFFENC_AC0_47rs> {
        HCODE94_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 94
    #[inline(always)]
    pub fn hlen94(&mut self) -> HLEN94_W<'_, HUFFENC_AC0_47rs> {
        HLEN94_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 95
    #[inline(always)]
    pub fn hcode95(&mut self) -> HCODE95_W<'_, HUFFENC_AC0_47rs> {
        HCODE95_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 95
    #[inline(always)]
    pub fn hlen95(&mut self) -> HLEN95_W<'_, HUFFENC_AC0_47rs> {
        HLEN95_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_47::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_47::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC0_47)*/
pub struct HUFFENC_AC0_47rs;
impl crate::RegisterSpec for HUFFENC_AC0_47rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_47::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_47rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_47::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_47rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_47 to value 0
impl crate::Resettable for HUFFENC_AC0_47rs {}
