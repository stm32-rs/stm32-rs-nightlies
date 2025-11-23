///Register `HUFFBASE20` reader
pub type R = crate::R<HUFFBASE20rs>;
///Register `HUFFBASE20` writer
pub type W = crate::W<HUFFBASE20rs>;
///Field `DATA40` reader - Data 40
pub type DATA40_R = crate::FieldReader<u16>;
///Field `DATA40` writer - Data 40
pub type DATA40_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA41` reader - Data 41
pub type DATA41_R = crate::FieldReader<u16>;
///Field `DATA41` writer - Data 41
pub type DATA41_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 40
    #[inline(always)]
    pub fn data40(&self) -> DATA40_R {
        DATA40_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 41
    #[inline(always)]
    pub fn data41(&self) -> DATA41_R {
        DATA41_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE20")
            .field("data40", &self.data40())
            .field("data41", &self.data41())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 40
    #[inline(always)]
    pub fn data40(&mut self) -> DATA40_W<'_, HUFFBASE20rs> {
        DATA40_W::new(self, 0)
    }
    ///Bits 16:24 - Data 41
    #[inline(always)]
    pub fn data41(&mut self) -> DATA41_W<'_, HUFFBASE20rs> {
        DATA41_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE20)*/
pub struct HUFFBASE20rs;
impl crate::RegisterSpec for HUFFBASE20rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase20::R`](R) reader structure
impl crate::Readable for HUFFBASE20rs {}
///`write(|w| ..)` method takes [`huffbase20::W`](W) writer structure
impl crate::Writable for HUFFBASE20rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE20 to value 0
impl crate::Resettable for HUFFBASE20rs {}
