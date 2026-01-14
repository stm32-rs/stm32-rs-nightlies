///Register `HUFFBASE29` reader
pub type R = crate::R<HUFFBASE29rs>;
///Register `HUFFBASE29` writer
pub type W = crate::W<HUFFBASE29rs>;
///Field `DATA58` reader - Data 58
pub type DATA58_R = crate::FieldReader<u16>;
///Field `DATA58` writer - Data 58
pub type DATA58_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA59` reader - Data 59
pub type DATA59_R = crate::FieldReader<u16>;
///Field `DATA59` writer - Data 59
pub type DATA59_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 58
    #[inline(always)]
    pub fn data58(&self) -> DATA58_R {
        DATA58_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 59
    #[inline(always)]
    pub fn data59(&self) -> DATA59_R {
        DATA59_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE29")
            .field("data58", &self.data58())
            .field("data59", &self.data59())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 58
    #[inline(always)]
    pub fn data58(&mut self) -> DATA58_W<'_, HUFFBASE29rs> {
        DATA58_W::new(self, 0)
    }
    ///Bits 16:24 - Data 59
    #[inline(always)]
    pub fn data59(&mut self) -> DATA59_W<'_, HUFFBASE29rs> {
        DATA59_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE29)*/
pub struct HUFFBASE29rs;
impl crate::RegisterSpec for HUFFBASE29rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase29::R`](R) reader structure
impl crate::Readable for HUFFBASE29rs {}
///`write(|w| ..)` method takes [`huffbase29::W`](W) writer structure
impl crate::Writable for HUFFBASE29rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE29 to value 0
impl crate::Resettable for HUFFBASE29rs {}
